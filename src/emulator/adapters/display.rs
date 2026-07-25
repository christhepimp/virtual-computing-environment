//! Display adapter — framebuffer view placeholder for GPU/monitor path.

#[derive(Default, Debug)]
pub struct DisplayAdapter {
    pub width: u32,
    pub height: u32,
    pub framebuffer_guest_addr: u64,
    pub dirty: bool,
}

impl DisplayAdapter {
    pub fn configure(&mut self, width: u32, height: u32, fb_addr: u64) {
        self.width = width;
        self.height = height;
        self.framebuffer_guest_addr = fb_addr;
        self.dirty = true;
    }
}
