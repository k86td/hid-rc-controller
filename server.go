package main

import (
	"bytes"
	"fmt"
	"io"
	"log"

	"github.com/sstallion/go-hid"
)

const MAX_READ = 512

//go:generate stringer -type=ItemType
type ItemType byte

// setting 6 bits values

const (
	ItemMainInput           ItemType = 0b100000
	ItemMainOutput          ItemType = 0b100100
	ItemMainFeature         ItemType = 0b101100
	ItemMainStartCollection ItemType = 0b101000
	ItemMainEndCollection   ItemType = 0b110000

	ItemGlobalUsagePage       ItemType = 0b000001
	ItemGlobalLogicalMinimum  ItemType = 0b000101
	ItemGlobalLogicalMaximum  ItemType = 0b001001
	ItemGlobalPhysicalMinimum ItemType = 0b001101
	ItemGlobalPhysicalMaximum ItemType = 0b010001
	ItemGlobalUnitExponent    ItemType = 0b010101
	ItemGlobalUnit            ItemType = 0b011001
	ItemGlobalReportSize      ItemType = 0b011101
	ItemGlobalReportId        ItemType = 0b100001
	ItemGlobalReportCount     ItemType = 0b100101
	ItemGlobalPush            ItemType = 0b101001
	ItemGlobalPop             ItemType = 0b101101

	ItemLocalUsage             ItemType = 0b000010
	ItemLocalUsageMinimum      ItemType = 0b000110
	ItemLocalUsageMaximum      ItemType = 0b001010
	ItemLocalDesignatorIndex   ItemType = 0b001110
	ItemLocalDesignatorMinimum ItemType = 0b010010
	ItemLocalDesignatorMaximum ItemType = 0b010110
	ItemLocalStringIndex       ItemType = 0b011110
	ItemLocalStringMinimum     ItemType = 0b100010
	ItemLocalStringMaximum     ItemType = 0b100110
	ItemLocalDelimiter         ItemType = 0b101010
)

//go:generate stringer -type=MainCollectionType
type MainCollectionType byte

const (
	MainCollectionTypePhysical      MainCollectionType = 0x00
	MainCollectionTypeApplication   MainCollectionType = 0x01
	MainCollectionTypeLogical       MainCollectionType = 0x02
	MainCollectionTypeReport        MainCollectionType = 0x03
	MainCollectionTypeNamedArray    MainCollectionType = 0x04
	MainCollectionTypeUsageSwitch   MainCollectionType = 0x05
	MainCollectionTypeUsageModifier MainCollectionType = 0x06
)

type ShortItem struct {
	BSize uint8
	BType ItemType
	Data  []byte
}

func (i ShortItem) String() string {
	switch i.BType {
	case ItemMainStartCollection:
		return fmt.Sprintf("ShortItem<%v>{ %v }", i.BType, MainCollectionType(i.Data[0]))
	default:
		return fmt.Sprintf("ShortItem<%v>{ %v } (%08b)", i.BType, i.Data, i.Data)
	}
}

func NewShortItem(raw byte) ShortItem {
	return ShortItem{
		BSize: []byte{0, 1, 2, 4}[0b00000011&raw],
		BType: ItemType(0b11111100 & raw >> 2),
	}
}

func (i *ShortItem) ConsumeData(rd io.ByteReader) {
	buf := make([]byte, 0)
	for range i.BSize {
		b, err := rd.ReadByte()
		if err != nil {
			log.Fatalf("err while consume ShortItem data: %v\n", err)
		}
		buf = append(buf, b)
	}

	i.Data = buf
}

// HID Usage Table: https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf
// Spec: https://www.usb.org/sites/default/files/documents/hid1_11.pdf
// ~ page 36

func ParseReportDescriptor(raw []byte) {
	rd := bytes.NewReader(raw)

	for {
		var cur, err = rd.ReadByte()
		if err == io.EOF {
			break
		}

		if cur == 0xFE {
			log.Fatal("not supporting long items yet")
		}

		itm := NewShortItem(cur)
		itm.ConsumeData(rd)
		fmt.Println(itm)
	}
}

func main() {
	hid.Init()
	defer hid.Exit()

	// this is the steering wheel
	dev, err := hid.Open(1103, 46742, "")
	// dev, err := hid.Open(1118, 2354, "fe:8c:7c:9e:14:69")
	if err != nil {
		log.Fatalf("error while opening device: %v", err)
	}
	defer dev.Close()

	devInfo, err := dev.GetDeviceInfo()
	if err != nil {
		log.Fatalf("error while getting device info: %v", err)
	}
	fmt.Printf("busType: %v, usage: %v, usagePage: %v, path: %v\n", devInfo.BusType.String(), devInfo.Usage, devInfo.UsagePage, devInfo.Path)

	b := make([]byte, MAX_READ)
	r, err := dev.GetReportDescriptor(b)
	if err != nil {
		log.Fatalf("error while getting device info: %v", err)
	}
	b = b[0:r]

	ParseReportDescriptor(b)

	// throttle; [5..=6]
	// b = make([]byte, 128)
	// for {
	// 	dev.Read(b)
	// 	fmt.Printf("%08b\n", b)
	// }
}
