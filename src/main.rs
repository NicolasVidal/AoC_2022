mod j1;
mod j2;
mod j3;

fn main() {
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
}
