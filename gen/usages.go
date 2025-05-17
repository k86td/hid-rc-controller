package gen

//go:generate stringer -type=UsagePage
type UsagePage byte

const (
	GenericDesktop UsagePage = 0x01
)

//go:generate stringer -type=UsageType
type UsageType byte

const (
	Undefined              UsageType = 0x00
	Pointer                UsageType = 0x01
	Mouse                  UsageType = 0x02
	Joystick               UsageType = 0x04
	GamePad                UsageType = 0x05
	Keyboard               UsageType = 0x06
	Keypad                 UsageType = 0x07
	MultiAxisController    UsageType = 0x08
	TabletPCSystemControls UsageType = 0x09

	X             UsageType = 0x30
	Y             UsageType = 0x31
	Z             UsageType = 0x32
	Rx            UsageType = 0x33
	Ry            UsageType = 0x34
	Rz            UsageType = 0x35
	Slider        UsageType = 0x36
	Dial          UsageType = 0x37
	Wheel         UsageType = 0x38
	HatSwitch     UsageType = 0x39
	CountedBuffer UsageType = 0x3A
	ByteCount     UsageType = 0x3B
	MotionWakeup  UsageType = 0x3C
	Start         UsageType = 0x3D
	Select        UsageType = 0x3E

	Vx                   UsageType = 0x40
	Vy                   UsageType = 0x41
	Vz                   UsageType = 0x42
	Vbrx                 UsageType = 0x43
	Vbry                 UsageType = 0x44
	Vbrz                 UsageType = 0x45
	Vno                  UsageType = 0x46
	FeatureNotification  UsageType = 0x47
	ResolutionMultiplier UsageType = 0x48

	SystemControl     UsageType = 0x80
	SystemPowerDown   UsageType = 0x81
	SystemSleep       UsageType = 0x82
	SystemWakeUp      UsageType = 0x83
	SystemContextMenu UsageType = 0x84
	SystemMainMenu    UsageType = 0x85
	SystemAppMenu     UsageType = 0x86
	SystemMenuHelp    UsageType = 0x87
	SystemMenuExit    UsageType = 0x88
	SystemMenuSelect  UsageType = 0x89
	SystemMenuRight   UsageType = 0x8A
	SystemMenuLeft    UsageType = 0x8B
	SystemMenuUp      UsageType = 0x8C
	SystemMenuDown    UsageType = 0x8D
	SystemColdRestart UsageType = 0x8E
	SystemWarmRestart UsageType = 0x8F

	DpadUp    UsageType = 0x90
	DpadDown  UsageType = 0x91
	DpadRight UsageType = 0x92
	DpadLeft  UsageType = 0x93

	SystemDock               UsageType = 0xA0
	SystemUndock             UsageType = 0xA1
	SystemSetup              UsageType = 0xA2
	SystemBreak              UsageType = 0xA3
	SystemDebuggerBreak      UsageType = 0xA4
	ApplicationBreak         UsageType = 0xA5
	ApplicationDebuggerBreak UsageType = 0xA6
	SystemSpeakerMute        UsageType = 0xA7
	SystemHibernate          UsageType = 0xA8

	SystemDisplayInvert       UsageType = 0xB0
	SystemDisplayInternal     UsageType = 0xB1
	SystemDisplayExternal     UsageType = 0xB2
	SystemDisplayBoth         UsageType = 0xB3
	SystemDisplayDual         UsageType = 0xB4
	SystemDisplayToggleIntExt UsageType = 0xB5
)
