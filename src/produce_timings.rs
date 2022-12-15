#[macro_use]
extern crate timeit;

use timeit::timeit;

mod j15;
mod j14;
mod j13;
mod j12;
mod j11;
mod j10;
mod j9;
mod j8;
mod j7;
mod j6;
mod j5;
mod j4;
mod j3;
mod j2;
mod j1;

fn main() {
    timeit!({j1::p1();});
    timeit!({j1::p2();});
    timeit!({j2::p1();});
    timeit!({j2::p2();});
    timeit!({j3::p1();});
    timeit!({j3::p2();});
    timeit!({j4::p1();});
    timeit!({j4::p2();});
    timeit!({j5::p1();});
    timeit!({j5::p2();});
    timeit!({j6::p1();});
    timeit!({j6::p2();});
    timeit!({j7::p1();});
    timeit!({j7::p2();});
    timeit!({j8::p1();});
    timeit!({j8::p2();});
    timeit!({j9::p1();});
    timeit!({j9::p2();});
    timeit!({j10::p1();});
    timeit!({j10::p2();});
    timeit!({j11::p1();});
    timeit!({j11::p2();});
    timeit!({j12::p1();});
    timeit!({j12::p2();});
    timeit!({j13::p1();});
    timeit!({j13::p2();});
    timeit!({j14::p1();});
    timeit!({j14::p2();});
    timeit!({j15::p1();});
    timeit!({j15::p2();});
}