#[derive(Default)]
struct Registers {
    pc: u16,
    sp: u16,
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

// concat_idents! doesn't work

macro_rules! get_u16 {
    ($hi: ident, $lo: ident, $name: ident) => (
        fn $name(&self) -> u16 {
            (self.$hi as u16) << 8 | (self.$lo as u16)
        }
    );
    // ($hi: ident, $lo: ident) => (
    //     get_u16!(concat_idents!(get_, $hi, $lo), $hi , $lo );
    // );
}
macro_rules! set_u16 {
    ($hi: ident, $lo: ident, $name: ident) => (
        fn $name(&mut self, val: u16) {
            self.$hi = (val >> 8) as u8;
            self.$lo = val as u8;
        }
    );
}
macro_rules! get_set_u16 {
    ($hi: ident, $lo: ident, $get: ident, $set: ident) => (
        get_u16!($hi, $lo, $get);
        set_u16!($hi, $lo, $set);
    );
}

macro_rules! get {
    ($t: ty, $name: ident, $reg: ident) => (
        fn $name(&self) -> $t {
            self.$reg
        }
    );
}
macro_rules! set {
    ($t: ty, $name: ident, $reg: ident) => (
        fn $name(&mut self, val: $t) {
            self.$reg = val
        }
    );
}
macro_rules! get_set {
    ($t: ty, $field: ident, $get: ident, $set: ident) => (
        get!($t, $get, $field);
        set!($t, $set, $field);
    );
}

impl Registers {
    get_set!(u8, a, get_a, set_a);
    get_set!(u8, b, get_b, set_b);
    get_set!(u8, c, get_c, set_c);
    get_set!(u8, d, get_d, set_d);
    get_set!(u8, e, get_e, set_e);
    get_set!(u8, h, get_h, set_h);
    get_set!(u8, l, get_l, set_l);

    get_set!(u16, pc, get_pc, set_pc);
    get_set!(u16, sp, get_sp, set_sp);

    get_set_u16!(b, c, get_bc, set_bc);
    get_set_u16!(d, e, get_de, set_de);
    get_set_u16!(h, l, get_hl, set_hl);
    
    // get_u16!(get_bc, b, c);
    // set_u16!(set_bc, b, c);
    // get_u16!(get_hl, h, l);
    // set_u16!(set_hl, h, l);
}

#[derive(Default)]
struct CPU {
    regs: Registers,
}

// const ops: [fn (mut CPU);] = [];

macro_rules! ld_r16_i16 {
    ($name: ident, $set: ident) => (
        fn $name(&mut self) {
            let val: u16 = self.fetch_u16();
            self.regs.$set(val);
        }
    );
}
macro_rules! ld_r8_r8 {
    ($name: ident, $set: ident, $get: ident) => (
        fn $name(&mut self) {
            let val: u8 = self.regs.$get();
            self.regs.$set(val);
        }
    );
}
macro_rules! inc_r16 {
    ($name: ident, $set: ident, $get: ident) => (
        fn $name(&mut self) {
            let val: u16 = self.regs.$get() + 1;
            self.regs.$set(val);
        }
    );
}
macro_rules! dec_r16 {
    ($name: ident, $set: ident, $get: ident) => (
        fn $name(&mut self) {
            let val: u16 = self.regs.$get() - 1;
            self.regs.$set(val);
        }
    );
}

impl CPU {
    fn fetch_u8(&mut self) -> u8 {
        0
    }
    fn fetch_u16(&mut self) -> u16 {
        0
    }

    fn execute_instruction(&mut self) {
        let op = self.fetch_u8();
        match op {
            0x00 => self.nop(),

            0x01 => self.ld_bc_i16(),
            0x03 => self.inc_bc(),
            0x0B => self.dec_bc(),

            0x11 => self.ld_de_i16(),
            0x13 => self.inc_de(),
            0x1B => self.dec_de(),

            0x21 => self.ld_hl_i16(),
            0x23 => self.inc_hl(),
            0x2B => self.dec_hl(),

            0x31 => self.ld_sp_i16(),
            0x33 => self.inc_sp(),
            0x3B => self.dec_sp(),

            0x40 => self.ld_b_b(),
            0x41 => self.ld_b_c(),
            0x42 => self.ld_b_d(),
            0x43 => self.ld_b_e(),
            0x44 => self.ld_b_h(),
            0x45 => self.ld_b_l(),
            0x47 => self.ld_b_a(),

            0x48 => self.ld_c_b(),
            0x49 => self.ld_c_c(),
            0x4a => self.ld_c_d(),
            0x4b => self.ld_c_e(),
            0x4c => self.ld_c_h(),
            0x4d => self.ld_c_l(),
            0x4f => self.ld_c_a(),

            0x50 => self.ld_d_b(),
            0x51 => self.ld_d_c(),
            0x52 => self.ld_d_d(),
            0x53 => self.ld_d_e(),
            0x54 => self.ld_d_h(),
            0x55 => self.ld_d_l(),
            0x57 => self.ld_d_a(),

            0x58 => self.ld_e_b(),
            0x59 => self.ld_e_c(),
            0x5a => self.ld_e_d(),
            0x5b => self.ld_e_e(),
            0x5c => self.ld_e_h(),
            0x5d => self.ld_e_l(),
            0x5f => self.ld_e_a(),

            0x60 => self.ld_h_b(),
            0x61 => self.ld_h_c(),
            0x62 => self.ld_h_d(),
            0x63 => self.ld_h_e(),
            0x64 => self.ld_h_h(),
            0x65 => self.ld_h_l(),
            0x67 => self.ld_h_a(),

            0x68 => self.ld_l_b(),
            0x69 => self.ld_l_c(),
            0x6a => self.ld_l_d(),
            0x6b => self.ld_l_e(),
            0x6c => self.ld_l_h(),
            0x6d => self.ld_l_l(),
            0x6f => self.ld_l_a(),

            0x78 => self.ld_a_b(),
            0x79 => self.ld_a_c(),
            0x7a => self.ld_a_d(),
            0x7b => self.ld_a_e(),
            0x7c => self.ld_a_h(),
            0x7d => self.ld_a_l(),
            0x7f => self.ld_a_a(),

            _ => self.unimplemented(op),
        }
    }

    fn unimplemented(&mut self, op: u8) {
        println!("unimplemented opcode: 0x{:X}", op);
    }
    fn nop(&mut self) {}

    ld_r16_i16!(ld_bc_i16, set_bc);
    ld_r16_i16!(ld_de_i16, set_de);
    ld_r16_i16!(ld_hl_i16, set_hl);
    ld_r16_i16!(ld_sp_i16, set_sp);

    ld_r8_r8!(ld_b_b, set_b, get_b);
    ld_r8_r8!(ld_d_b, set_d, get_b);
    ld_r8_r8!(ld_h_b, set_h, get_b);

    ld_r8_r8!(ld_b_c, set_b, get_c);
    ld_r8_r8!(ld_d_c, set_d, get_c);
    ld_r8_r8!(ld_h_c, set_h, get_c);

    ld_r8_r8!(ld_b_d, set_b, get_d);
    ld_r8_r8!(ld_d_d, set_d, get_d);
    ld_r8_r8!(ld_h_d, set_h, get_d);

    ld_r8_r8!(ld_b_e, set_b, get_e);
    ld_r8_r8!(ld_d_e, set_d, get_e);
    ld_r8_r8!(ld_h_e, set_h, get_e);

    ld_r8_r8!(ld_b_h, set_b, get_h);
    ld_r8_r8!(ld_d_h, set_d, get_h);
    ld_r8_r8!(ld_h_h, set_h, get_h);

    ld_r8_r8!(ld_b_l, set_b, get_l);
    ld_r8_r8!(ld_d_l, set_d, get_l);
    ld_r8_r8!(ld_h_l, set_h, get_l);

    ld_r8_r8!(ld_b_a, set_b, get_a);
    ld_r8_r8!(ld_d_a, set_d, get_a);
    ld_r8_r8!(ld_h_a, set_h, get_a);

    ld_r8_r8!(ld_c_b, set_c, get_b);
    ld_r8_r8!(ld_e_b, set_e, get_b);
    ld_r8_r8!(ld_l_b, set_l, get_b);

    ld_r8_r8!(ld_c_c, set_c, get_c);
    ld_r8_r8!(ld_e_c, set_e, get_c);
    ld_r8_r8!(ld_l_c, set_l, get_c);

    ld_r8_r8!(ld_c_d, set_c, get_d);
    ld_r8_r8!(ld_e_d, set_e, get_d);
    ld_r8_r8!(ld_l_d, set_l, get_d);

    ld_r8_r8!(ld_c_e, set_c, get_e);
    ld_r8_r8!(ld_e_e, set_e, get_e);
    ld_r8_r8!(ld_l_e, set_l, get_e);

    ld_r8_r8!(ld_c_h, set_c, get_h);
    ld_r8_r8!(ld_e_h, set_e, get_h);
    ld_r8_r8!(ld_l_h, set_l, get_h);

    ld_r8_r8!(ld_c_l, set_c, get_l);
    ld_r8_r8!(ld_e_l, set_e, get_l);
    ld_r8_r8!(ld_l_l, set_l, get_l);

    ld_r8_r8!(ld_c_a, set_c, get_a);
    ld_r8_r8!(ld_e_a, set_e, get_a);
    ld_r8_r8!(ld_l_a, set_l, get_a);

    ld_r8_r8!(ld_a_b, set_a, get_b);
    ld_r8_r8!(ld_a_c, set_a, get_c);
    ld_r8_r8!(ld_a_d, set_a, get_d);
    ld_r8_r8!(ld_a_e, set_a, get_e);
    ld_r8_r8!(ld_a_h, set_a, get_h);
    ld_r8_r8!(ld_a_l, set_a, get_l);
    ld_r8_r8!(ld_a_a, set_a, get_a);

    inc_r16!(inc_bc, set_bc, get_bc);
    inc_r16!(inc_de, set_de, get_de);
    inc_r16!(inc_hl, set_hl, get_hl);
    inc_r16!(inc_sp, set_sp, get_sp);

    dec_r16!(dec_bc, set_bc, get_bc);
    dec_r16!(dec_de, set_de, get_de);
    dec_r16!(dec_hl, set_hl, get_hl);
    dec_r16!(dec_sp, set_sp, get_sp);
}

// impl Default for CPU {
//     fn default() -> CPU {
//         CPU { regs: Default::default(), ops: [nop]}
//     }
// }

fn main() {
    let mut cpu: CPU = Default::default();
    cpu.regs.set_hl(0xAABB);
    println!("{:X} {:X}", cpu.regs.get_h(), cpu.regs.get_l());
}
