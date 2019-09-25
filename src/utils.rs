use std::thread::sleep;
use std::time::Duration;

#[inline]
pub fn wait_ns(ns: u32) {
    sleep(Duration::new(0, ns));
}

#[inline]
pub fn wait_us(us: u32) {
    wait_ns(us * 1000);
}

#[inline]
pub fn wait_ms(ms: u32) {
    wait_ns(ms * 1000)
}