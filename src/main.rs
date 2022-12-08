extern crate core;

mod j8;
mod j7;
mod j6;
mod j5;
mod j4;
mod j3;
mod j2;
mod j1;

use std::alloc::System;
#[allow(unused)]
use tracking_allocator::{
    AllocationGroupId, AllocationGroupToken, AllocationRegistry, AllocationTracker, Allocator,
};

#[global_allocator]
static GLOBAL: tracking_allocator::Allocator<System> =
    tracking_allocator::Allocator::system();

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
    println!("J6 -----------------------------------------------------PROBLEM HEAP ALLOC");
    println!("p1");
    // AllocationRegistry::enable_tracking();
    println!("{}", j6::p1());
    // AllocationRegistry::disable_tracking();
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

}