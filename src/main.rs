#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello, world!");
    unsafe {
      let mut graph = root::taso::Graph::new();
      // let mut graph = root::taso::Graph::new();
      println!("Got here");
      println!("{}", graph.total_cost());
    }
}
