use libc::c_int;

#[repr(C)]
pub struct SceCtrlData {
    time_stamp: u32,
    pub buttons: u32,
    lx: u8,
    ly: u8,
    rsrv: [u8; 6]
}

impl SceCtrlData {
    pub fn new() -> SceCtrlData {
        SceCtrlData {
            time_stamp: 0,
            buttons: 0,
            lx: 0,
            ly: 0,
            rsrv: [0; 6]
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

pub mod button {
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

pub enum Button {
    Select,
    Start,
    Up,
    Right,
    Down,
    Left,
    LTrigger,
    RTrigger,
    Triangle,
    Circle,
    Cross,
    Square,
    Home,
    Hold,
    Note,
    Screen,
    Volup,
    Voldown,
    WlanUp,
    Remote,
    Disc,
    Ms,
    None
}

#[link(name = "pspctrl")]
#[allow(improper_ctypes)]
extern {
    fn sceCtrlSetSamplingCycle(cycle: c_int) -> c_int;
    fn sceCtrlGetSamplingCycle(cycle: &mut c_int) -> c_int;
    fn sceCtrlSetSamplingMode(mode: c_int) -> c_int;
    fn sceCtrlGetSamplingMode(cycle: &mut c_int) -> c_int;

    fn sceCtrlPeekBufferPositive(pad_data: &mut SceCtrlData,
                                 count: c_int) -> c_int;
    fn sceCtrlPeekBufferNegative(pad_data: &mut SceCtrlData,
                                 count: c_int) -> c_int;

    fn sceCtrlReadBufferPositive(pad_data: &mut SceCtrlData,
                                 count: c_int) -> c_int;

    fn sceCtrlReadBufferNegative(pad_data: &mut SceCtrlData,
                                 count: c_int) -> c_int;

    fn sceCtrlPeekLatch(latch_data: &mut SceCtrlLatch) -> c_int;
    fn sceCtrlReadLatch(latch_data: &mut SceCtrlLatch) -> c_int;
}

// let pdata = SceCtrlData { ... }
// let pad = ctrl::read(&pad_data);
// if button_pressed(pad, button::PSP_CTRL_UP) {
// }
pub fn button_pressed(pad_data: &SceCtrlData, but: u32) -> bool {
    (pad_data.buttons & but) == but
}

macro_rules! if_pressed(
    ($pad:expr, $button:expr, $name:expr) => (
        if ($pad.buttons & $button) == $button {
            return $name
        }
    )
);

pub struct Input {
    ctrl_data: SceCtrlData
}

impl Input {
    /// Create a new instance, wrapping the low-level SceCtrlData
    pub fn new() -> Input {
        Input {
            ctrl_data: SceCtrlData::new()
        }
    }

    /// Read new input data into the contained SceCtrlData
    pub fn read(&mut self) {
        unsafe { sceCtrlPeekBufferPositive(&mut self.ctrl_data, 1); }
    }

    /// Read new input data
    /// and determine if input changed compared to the last read
    pub fn read_changed(&mut self) -> bool {
        let old_buttons = self.ctrl_data.buttons;
        self.read();
        old_buttons == self.ctrl_data.buttons
    }

    /// Return the key that was pressed.
    ///
    /// Does not handle multiple keys pressed at once.
    pub fn pressed_key(&mut self) -> Button {
        if_pressed!(self.ctrl_data, button::PSP_CTRL_START, Button::Start);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_UP, Button::Up);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_RIGHT, Button::Right);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_DOWN, Button::Down);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_LEFT, Button::Left);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_LTRIGGER, Button::LTrigger);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_RTRIGGER, Button::RTrigger);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_TRIANGLE, Button::Triangle);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_CIRCLE, Button::Circle);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_CROSS, Button::Cross);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_SQUARE, Button::Square);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_HOME, Button::Home);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_HOLD, Button::Hold);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_NOTE, Button::Note);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_SCREEN, Button::Screen);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_VOLUP, Button::Volup);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_VOLDOWN, Button::Voldown);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_WLAN_UP, Button::WlanUp);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_REMOTE, Button::Remote);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_DISC, Button::Disc);
        if_pressed!(self.ctrl_data, button::PSP_CTRL_MS, Button::Ms);

        Button::None
    }
}

