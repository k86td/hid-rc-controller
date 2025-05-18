package gen

//go:generate stringer -type=UsagePage
type UsagePage byte

const (
	GenericDesktop UsagePage = 0x01
)

//go:generate stringer -type=GenericDesktopUsageType
type GenericDesktopUsageType byte

const (
	Undefined              GenericDesktopUsageType = 0x00
	Pointer                GenericDesktopUsageType = 0x01
	Mouse                  GenericDesktopUsageType = 0x02
	Joystick               GenericDesktopUsageType = 0x04
	GamePad                GenericDesktopUsageType = 0x05
	Keyboard               GenericDesktopUsageType = 0x06
	Keypad                 GenericDesktopUsageType = 0x07
	MultiAxisController    GenericDesktopUsageType = 0x08
	TabletPCSystemControls GenericDesktopUsageType = 0x09

	X             GenericDesktopUsageType = 0x30
	Y             GenericDesktopUsageType = 0x31
	Z             GenericDesktopUsageType = 0x32
	Rx            GenericDesktopUsageType = 0x33
	Ry            GenericDesktopUsageType = 0x34
	Rz            GenericDesktopUsageType = 0x35
	Slider        GenericDesktopUsageType = 0x36
	Dial          GenericDesktopUsageType = 0x37
	Wheel         GenericDesktopUsageType = 0x38
	HatSwitch     GenericDesktopUsageType = 0x39
	CountedBuffer GenericDesktopUsageType = 0x3A
	ByteCount     GenericDesktopUsageType = 0x3B
	MotionWakeup  GenericDesktopUsageType = 0x3C
	Start         GenericDesktopUsageType = 0x3D
	Select        GenericDesktopUsageType = 0x3E

	Vx                   GenericDesktopUsageType = 0x40
	Vy                   GenericDesktopUsageType = 0x41
	Vz                   GenericDesktopUsageType = 0x42
	Vbrx                 GenericDesktopUsageType = 0x43
	Vbry                 GenericDesktopUsageType = 0x44
	Vbrz                 GenericDesktopUsageType = 0x45
	Vno                  GenericDesktopUsageType = 0x46
	FeatureNotification  GenericDesktopUsageType = 0x47
	ResolutionMultiplier GenericDesktopUsageType = 0x48

	SystemControl     GenericDesktopUsageType = 0x80
	SystemPowerDown   GenericDesktopUsageType = 0x81
	SystemSleep       GenericDesktopUsageType = 0x82
	SystemWakeUp      GenericDesktopUsageType = 0x83
	SystemContextMenu GenericDesktopUsageType = 0x84
	SystemMainMenu    GenericDesktopUsageType = 0x85
	SystemAppMenu     GenericDesktopUsageType = 0x86
	SystemMenuHelp    GenericDesktopUsageType = 0x87
	SystemMenuExit    GenericDesktopUsageType = 0x88
	SystemMenuSelect  GenericDesktopUsageType = 0x89
	SystemMenuRight   GenericDesktopUsageType = 0x8A
	SystemMenuLeft    GenericDesktopUsageType = 0x8B
	SystemMenuUp      GenericDesktopUsageType = 0x8C
	SystemMenuDown    GenericDesktopUsageType = 0x8D
	SystemColdRestart GenericDesktopUsageType = 0x8E
	SystemWarmRestart GenericDesktopUsageType = 0x8F

	DpadUp    GenericDesktopUsageType = 0x90
	DpadDown  GenericDesktopUsageType = 0x91
	DpadRight GenericDesktopUsageType = 0x92
	DpadLeft  GenericDesktopUsageType = 0x93

	SystemDock               GenericDesktopUsageType = 0xA0
	SystemUndock             GenericDesktopUsageType = 0xA1
	SystemSetup              GenericDesktopUsageType = 0xA2
	SystemBreak              GenericDesktopUsageType = 0xA3
	SystemDebuggerBreak      GenericDesktopUsageType = 0xA4
	ApplicationBreak         GenericDesktopUsageType = 0xA5
	ApplicationDebuggerBreak GenericDesktopUsageType = 0xA6
	SystemSpeakerMute        GenericDesktopUsageType = 0xA7
	SystemHibernate          GenericDesktopUsageType = 0xA8

	SystemDisplayInvert       GenericDesktopUsageType = 0xB0
	SystemDisplayInternal     GenericDesktopUsageType = 0xB1
	SystemDisplayExternal     GenericDesktopUsageType = 0xB2
	SystemDisplayBoth         GenericDesktopUsageType = 0xB3
	SystemDisplayDual         GenericDesktopUsageType = 0xB4
	SystemDisplayToggleIntExt GenericDesktopUsageType = 0xB5
)
