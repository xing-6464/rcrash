use std::mem;

fn main() {
    unsafe {
        let a = [0u8, 1u8, 0u8, 0u8];
        let b: u32 = mem::transmute(a);
        println!("{}", b)
    }
}