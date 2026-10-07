mod register;
use register::{Flag,Flags};

fn main() {
    let mut flags = Flags::default();

    flags.set(Flag::Zero, true);
    flags.set(Flag::Carry, true);

    println!("Flags: {:08b}", flags.bits());
    println!("Zero set: {}", flags.is_set(Flag::Zero));

    flags.set(Flag::Zero, false);

    println!("After clearing zero: {:08b}", flags.bits());

    flags.load(0xff);

    println!("After loading 0xff: {:08b}", flags.bits());
}