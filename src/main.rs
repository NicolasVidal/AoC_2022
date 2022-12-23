mod j23;
mod j22;
mod j21;
mod j20;
mod j19;
mod j18;
mod j17;
mod j16;
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

extern crate core;

use std::alloc::System;

#[allow(unused)]
use tracking_allocator::{
    AllocationGroupId, AllocationRegistry, AllocationTracker, Allocator,
};

#[global_allocator]
static GLOBAL: Allocator<System> = tracking_allocator::Allocator::system();

// #[global_allocator]
// static GLOBAL: Allocator<System> = Allocator::system();

struct StdoutTracker;

impl AllocationTracker for StdoutTracker {
    fn allocated(
        &self,
        addr: usize,
        object_size: usize,
        wrapped_size: usize,
        group_id: AllocationGroupId,
    ) {
        println!(
            "allocation -> addr=0x{:0x} object_size={} wrapped_size={} group_id={:?}",
            addr, object_size, wrapped_size, group_id
        );
    }

    fn deallocated(
        &self,
        addr: usize,
        object_size: usize,
        wrapped_size: usize,
        source_group_id: AllocationGroupId,
        current_group_id: AllocationGroupId,
    ) {
        println!(
            "deallocation -> addr=0x{:0x} object_size={} wrapped_size={} source_group_id={:?} current_group_id={:?}",
            addr, object_size, wrapped_size, source_group_id, current_group_id
        );
    }
}

fn main() {
    AllocationRegistry::set_global_tracker(StdoutTracker)
        .expect("no other global tracker should be set yet");

    println!("STARTING DAYS COMPUTATIONS");

    AllocationRegistry::enable_tracking();
    println!("J1 -----------------------------------------------------");
    println!("p1");
    println!("{}", j1::p1());
    println!("p2");
    println!("{}", j1::p2());
    println!("J2 -----------------------------------------------------");
    println!("p1");
    println!("{}", j2::p1());
    println!("p2");
    println!("{}", j2::p2());
    println!("J3 -----------------------------------------------------");
    println!("p1");
    println!("{}", j3::p1());
    println!("p2");
    println!("{}", j3::p2());
    println!("J4 -----------------------------------------------------");
    println!("p1");
    println!("{}", j4::p1());
    println!("p2");
    println!("{}", j4::p2());
    println!("J5 -----------------------------------------------------");
    println!("p1");
    println!("{}", j5::p1());
    println!("p2");
    println!("{}", j5::p2());
    println!("J6 -----------------------------------------------------");
    println!("p1");
    println!("{}", j6::p1());
    println!("p2");
    println!("{}", j6::p2());
    println!("J7 -----------------------------------------------------");
    println!("p1");
    println!("{}", j7::p1());
    println!("p2");
    println!("{}", j7::p2());
    println!("J8 -----------------------------------------------------");
    println!("p1");
    println!("{}", j8::p1());
    println!("p2");
    println!("{}", j8::p2());
    println!("J9 -----------------------------------------------------");
    println!("p1");
    println!("{}", j9::p1());
    println!("p2");
    println!("{}", j9::p2());
    println!("J10 -----------------------------------------------------");
    println!("p1");
    println!("{}", j10::p1());
    println!("p2");
    println!("{}", j10::p2());
    println!("J11 -----------------------------------------------------");
    println!("p1");
    println!("{}", j11::p1());
    println!("p2");
    println!("{}", j11::p2());
    println!("J12 -----------------------------------------------------");
    println!("p1");
    println!("{}", j12::p1());
    println!("p2");
    println!("{}", j12::p2());
    println!("J13 -----------------------------------------------------");
    println!("p1");
    println!("{}", j13::p1());
    println!("p2");
    println!("{}", j13::p2());
    println!("J14 -----------------------------------------------------");
    println!("p1");
    println!("{}", j14::p1());
    println!("p2");
    println!("{}", j14::p2());
    println!("J15 -----------------------------------------------------");
    println!("p1");
    println!("{}", j15::p1());
    println!("p2");
    println!("{}", j15::p2());
    AllocationRegistry::disable_tracking();
    println!("J16 -----------------------------------------------------");
    println!("p1");
    println!("{}", j16::p1());
    println!("p2");
    println!("{}", j16::p2());
    AllocationRegistry::enable_tracking();
    println!("J17 -----------------------------------------------------");
    println!("p1");
    println!("{}", j17::p1());
    println!("p2");
    println!("{}", j17::p2());
    println!("J18 -----------------------------------------------------");
    println!("p1");
    println!("{}", j18::p1());
    println!("p2");
    println!("{}", j18::p2());
    println!("J19 -----------------------------------------------------");
    println!("p1");
    println!("{}", j19::p1());
    println!("p2");
    println!("{}", j19::p2());
    println!("J20 -----------------------------------------------------");
    println!("p1");
    println!("{}", j20::p1());
    println!("p2");
    println!("{}", j20::p2());
    println!("J21 -----------------------------------------------------");
    println!("p1");
    println!("{}", j21::p1());
    println!("p2");
    println!("{}", j21::p2());
    println!("J22 -----------------------------------------------------");
    println!("p1");
    println!("{}", j22::p1());
    println!("p2");
    println!("{}", j22::p2());
    println!("J23 -----------------------------------------------------");
    println!("p1");
    println!("{}", j23::p1());
    println!("p2");
    println!("{}", j23::p2());
    AllocationRegistry::disable_tracking();
}