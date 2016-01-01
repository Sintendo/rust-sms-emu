use std::ops::Deref;
use std::ops::DerefMut;

struct CPU {

    // Registers
    pc: u16,
    sp: u16,
}

#[derive(Default, Debug)]
struct Register8 {
    val: u8,
}


impl Deref for Register8 {
    type Target = u8;

    fn deref<'a>(&'a self) -> &u8 {
        &self.val
    }
}
impl DerefMut for Register8 {
    fn deref_mut<'a>(&'a mut self) -> &'a mut u8 {
        &mut self.val
    }
}

#[derive(Debug)]
struct Register16<'a> {
    hi: &'a mut Register8,
    lo: &'a mut Register8,
}

fn main() {
    let mut x: Register8 = Default::default();
    let mut y: Register8 = Default::default();
    let mut xy: Register16 = Register16 { hi: &mut x, lo: &mut y };

    *x = 23;
    println!("{}", *x);
}
