#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello, world!");
    unsafe {
      let mut dims = Vec::with_capacity(16);
      dims.push(1);
      dims.push(1);
      dims.push(1);
      dims.push(1);
      let mut graph = root::taso::Graph::new();
      root::taso::Graph_Graph(&mut graph);
      // Segfaults here without the preceding line
      let input = graph.new_input(dims.len() as i32, dims.as_ptr());
      graph.run();
      println!("{}", graph.total_cost());
    }
}
