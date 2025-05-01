package gen

//go:generate stringer -type=ValueType
type ValueType uint8

//go:generate stringer -type=FieldType
type FieldType uint8

//go:generate stringer -type=DataType
type DataType uint8

//go:generate stringer -type=WrapType
type WrapType uint8

//go:generate stringer -type=LinearityType
type LinearityType uint8

//go:generate stringer -type=PrefStateType
type PrefStateType uint8

//go:generate stringer -type=NullPositionType
type NullPositionType uint8

//go:generate stringer -type=VolatilityType
type VolatilityType uint8

//go:generate stringer -type=StreamType
type StreamType uint8

const (
	Data ValueType = iota
	Constant
)

const (
	Array FieldType = iota
	Variable
)

const (
	Absolute DataType = iota
	Relative
)

const (
	NoWrap WrapType = iota
	Wrap
)

const (
	Linear LinearityType = iota
	Nonlinear
)

const (
	PreferredState PrefStateType = iota
	NoPreferredState
)

const (
	NoNullPosition NullPositionType = iota
	NullState
)

const (
	NonVolatile VolatilityType = iota
	Volatile
)

const (
	BitFiled StreamType = iota
	BufferedBytes
)

//go:generate stringer -type=ItemType
type ItemType byte

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
