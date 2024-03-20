use rand::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    println!("y = {:?}", y);
}
