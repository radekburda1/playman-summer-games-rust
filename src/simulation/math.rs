//! Original integer trigonometry; no display or platform dependency.
const fn sine_table() -> [i32; 512] {
    let mut a = [0; 512];
    let mut l = 0x40000000i64;
    let mut r = 1073660973i64;
    a[128] = (l >> 16) as i32;
    a[129] = (r >> 16) as i32;
    let mut n = 2;
    while n < 128 {
        let next = ((r * 2147321946) >> 30) - l;
        a[(128 + n) & 511] = (next >> 16) as i32;
        l = r;
        r = next;
        n += 1;
    }
    n = 128;
    while n > 0 {
        n -= 1;
        a[(128 + 256 - n) & 511] = -a[(128 + n) & 511];
    }
    n = 255;
    while n > 0 {
        a[(128 + 256 + n) & 511] = -a[(128 + n) & 511];
        n -= 1;
    }
    a
}
pub const SINE: [i32; 512] = sine_table();
pub fn sin(a: i32) -> i32 {
    let i = ((a >> 8) & 511) as usize;
    SINE[i] + (((a & 255) * (SINE[(i + 1) & 511] - SINE[i])) >> 8)
}
pub fn cos(a: i32) -> i32 {
    sin(a + 32768)
}
