#[link(name = "pspdisplay")]
extern {
    fn sceDisplayWaitVblankStart() -> int;
}

// Wait for vertical blank start
pub fn wait_vblank_start() -> i32 {
    unsafe { sceDisplayWaitVblankStart() as i32 }
}
