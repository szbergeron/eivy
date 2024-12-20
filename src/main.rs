#![feature(generic_const_exprs)]

pub mod memory;
pub mod cpu;
pub mod builder;
pub mod instruction;
pub mod front;
pub mod ir;
pub mod chase;
pub mod leg;
pub mod util;

pub mod prelude {
    pub use crate::front::*;
    pub use crate::instruction::*;
    pub use crate::memory::*;
    pub use crate::util::*;
}

fn main() {
    println!("Hello, world!");
}
