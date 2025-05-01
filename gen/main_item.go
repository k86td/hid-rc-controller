package gen

type ItemMainInputFlags struct {
	ValueType        ValueType
	FieldType        FieldType
	DataType         DataType
	WrapType         WrapType
	LinearityType    LinearityType
	PrefStateType    PrefStateType
	NullPositionType NullPositionType
	VolatilityType   VolatilityType
	StreamType       StreamType
}

func NewItemMainInputFlags(bits byte) ItemMainInputFlags {
	return ItemMainInputFlags{
		ValueType:        ValueType(bits & 0b00000001),               // bit 0
		FieldType:        FieldType((bits & 0b00000010) >> 1),        // bit 1
		DataType:         DataType((bits & 0b00000100) >> 2),         // bit 2
		WrapType:         WrapType((bits & 0b00001000) >> 3),         // bit 3
		LinearityType:    LinearityType((bits & 0b00010000) >> 4),    // bit 4
		PrefStateType:    PrefStateType((bits & 0b00100000) >> 5),    // bit 5
		NullPositionType: NullPositionType((bits & 0b01000000) >> 6), // bit 6
		VolatilityType:   VolatilityType((bits & 0b10000000) >> 7),   // bit 7
	}
}
