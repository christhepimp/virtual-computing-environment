//! Input adapter — keyboard/mouse queues toward guest (virtio-input style later).

use super::super::qemu::QemuTransport;

#[derive(Default, Debug)]
pub struct InputAdapter {
    pub key_queue: Vec<u8>,
    pub mouse_queue: Vec<[u8; 3]>,
}

impl InputAdapter {
    pub fn push_key(&mut self, scancode: u8) {
        self.key_queue.push(scancode);
    }

    pub fn push_mouse(&mut self, packet: [u8; 3]) {
        self.mouse_queue.push(packet);
    }

    pub fn flush_to_transport(&mut self, transport: &mut QemuTransport) {
        for sc in self.key_queue.drain(..) {
            transport.inject_key(sc);
        }
        for pkt in self.mouse_queue.drain(..) {
            transport.inject_mouse(pkt);
        }
    }
}
