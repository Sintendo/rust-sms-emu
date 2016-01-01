use std::fs::File;
use std::io::Read;

/*fn main1() {
    let x = 4;
    let y = 10;

    let mut r = &x;
    r = &y;

    println!("{} + {} = {}", r, y, x + y);
    println!("hello world!");
    println!("hello world!");
}*/

// const FILENAME: $'static str = "Sonic the Hedgehod.sms";
const FILENAME: &'static str = "Alex Kidd in Miracle World.sms";

fn main() {
    let mut file = File::open(FILENAME).unwrap();
    let mut contents: Vec<u8> = Vec::new();
    let result = file.read_to_end(&mut contents).unwrap();
    println!("{}", result);

    let regs: Regs = Default::default();
    println!("{:?}", regs);
    regs.wtf();

    
}

fn add(x: u32, y: u32) -> u32 {
    return x + y;
}

#[derive(Default, Debug)]
struct Regs {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

impl Regs {
    fn wtf(&self) {
        println!("wtf");
    }
}

struct Register8 {
    val: u8,
}

struct Register16<'a> {
    hi: &'a u8,
    lo: &'a u8,
}
