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

      let mut w = Vec::with_capacity(16);
      w.push(1);
      w.push(1);
      w.push(1);
      w.push(1);
      let weight = graph.new_input(w.len() as i32, w.as_ptr());
      graph.matmul(input, weight, root::taso::ActiMode_AC_MODE_NONE);
      graph.optimize(1.0, 100, true);
      println!("{}", graph.total_cost());
    }
}
