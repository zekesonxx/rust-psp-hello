#[repr(C)]
pub struct SceCtrlData {
    time_stamp: u32,
    pub buttons: u32,
    lx: u8,
    ly: u8,
    rsrv: [u8, ..6]
}

impl SceCtrlData {
    pub fn new() -> SceCtrlData {
        SceCtrlData {
            time_stamp: 0,
            buttons: 0,
            lx: 0,
            ly: 0,
            rsrv: [0, ..6]
        }
    }
}

#[repr(C)]
pub struct SceCtrlLatch {
    ui_make: u32,
    ui_break: u32,
    ui_press: u32,
    ui_release: u32
}

pub mod Button {
    pub static PSP_CTRL_SELECT   : u32 = 0x000001;
    pub static PSP_CTRL_START    : u32 = 0x000008;
    pub static PSP_CTRL_UP       : u32 = 0x000010;
    pub static PSP_CTRL_RIGHT    : u32 = 0x000020;
    pub static PSP_CTRL_DOWN     : u32 = 0x000040;
    pub static PSP_CTRL_LEFT     : u32 = 0x000080;
    pub static PSP_CTRL_LTRIGGER : u32 = 0x000100;
    pub static PSP_CTRL_RTRIGGER : u32 = 0x000200;
    pub static PSP_CTRL_TRIANGLE : u32 = 0x001000;
    pub static PSP_CTRL_CIRCLE   : u32 = 0x002000;
    pub static PSP_CTRL_CROSS    : u32 = 0x004000;
    pub static PSP_CTRL_SQUARE   : u32 = 0x008000;
    pub static PSP_CTRL_HOME     : u32 = 0x010000;
    pub static PSP_CTRL_HOLD     : u32 = 0x020000;
    pub static PSP_CTRL_NOTE     : u32 = 0x800000;
    pub static PSP_CTRL_SCREEN   : u32 = 0x400000;
    pub static PSP_CTRL_VOLUP    : u32 = 0x100000;
    pub static PSP_CTRL_VOLDOWN  : u32 = 0x200000;
    pub static PSP_CTRL_WLAN_UP  : u32 = 0x040000;
    pub static PSP_CTRL_REMOTE   : u32 = 0x080000;
    pub static PSP_CTRL_DISC     : u32 = 0x1000000;
    pub static PSP_CTRL_MS       : u32 = 0x2000000;
}

#[link(name = "pspctrl")]
extern {
    fn sceCtrlSetSamplingCycle(cycle: int) -> int;
    fn sceCtrlGetSamplingCycle(cycle: &mut int) -> int;
    fn sceCtrlSetSamplingMode(mode: int) -> int;
    fn sceCtrlGetSamplingMode(cycle: &mut int) -> int;

    fn sceCtrlPeekBufferPositive(pad_data: &mut SceCtrlData,
                                 count: int) -> int;
    fn sceCtrlPeekBufferNegative(pad_data: &mut SceCtrlData,
                                 count: int) -> int;

    fn sceCtrlReadBufferPositive(pad_data: &mut SceCtrlData,
                                 count: int) -> int;

    fn sceCtrlReadBufferNegative(pad_data: &mut SceCtrlData,
                                 count: int) -> int;

    fn sceCtrlPeekLatch(latch_data: &mut SceCtrlLatch) -> int;
    fn sceCtrlReadLatch(latch_data: &mut SceCtrlLatch) -> int;
}

pub fn read(pad_data: &mut SceCtrlData) -> u32 {
    unsafe { sceCtrlPeekBufferPositive(pad_data, 1) as u32 }
}

//
// let pdata = SceCtrlData { ... }
// let pad = ctrl::read(&pad_data);
// if button_pressed(pad, Button::PSP_CTRL_UP) {
// }
pub fn button_pressed(pad_data: &SceCtrlData, but: u32) -> bool {
    (pad_data.buttons & but) == but
}
