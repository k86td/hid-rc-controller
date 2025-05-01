package main

import (
	"bytes"
	"fmt"
	"io"
	"log"

	"github.com/sstallion/go-hid"

	"github.com/k86td/web-hid-controller/gen"
)

const MAX_READ = 512

type ShortItem struct {
	BSize uint8
	BType gen.ItemType
	Data  []byte
}

func (i ShortItem) String() string {
	switch i.BType {
	case gen.ItemMainStartCollection:
		return fmt.Sprintf("ShortItem<%v>{ %v }", i.BType, gen.MainCollectionType(i.Data[0]))
	case gen.ItemMainInput:
		return fmt.Sprintf("ShortItem<%v>{ %v }", i.BType, gen.NewItemMainInputFlags(i.Data[0]))
	default:
		return fmt.Sprintf("ShortItem<%v>{ %v } (%08b)", i.BType, i.Data, i.Data)
	}
}

func NewShortItem(raw byte) ShortItem {
	return ShortItem{
		BSize: []byte{0, 1, 2, 4}[0b00000011&raw],
		BType: gen.ItemType(0b11111100 & raw >> 2),
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
