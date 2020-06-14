#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use rand::prelude::*;
use root::taso::*;

unsafe fn new_w(graph: &mut Graph, mut v: Vec<i32>, rng: &mut impl Rng) -> *mut Tensor {
      let vol = v.iter().product();
      let data: Vec<f32> = (0..vol).map(|_| rng.gen()).collect();
      graph.new_weight(v.len() as i32, v.as_ptr(), data.as_ptr())
}

unsafe fn new_i(graph: &mut Graph, v: Vec<i32>) -> *mut Tensor {
      graph.new_input(v.len() as i32, v.as_ptr())
}

fn main() {
    unsafe {
      let mut rng = rand::thread_rng();

      let mut graph = Graph::new();
      Graph_Graph(&mut graph);

      let input = new_i(&mut graph, vec![1, 128, 56, 56]);
      let w1 = new_w(&mut graph, vec![128, 128, 3, 3], &mut rng);
      let w2 = new_w(&mut graph, vec![128, 128, 1, 1], &mut rng);
      let w3 = new_w(&mut graph, vec![128, 128, 3, 3], &mut rng);

      let left = graph.conv2d1(input, w1, 1, 1, PaddingMode_PD_MODE_SAME, ActiMode_AC_MODE_RELU);
      let mid = graph.conv2d1(left, w3, 1, 1, PaddingMode_PD_MODE_SAME, ActiMode_AC_MODE_NONE);
      let right = graph.conv2d1(input, w2, 1, 1, PaddingMode_PD_MODE_SAME, ActiMode_AC_MODE_RELU);
      let output = graph.element(OpType_OP_EW_ADD, mid, right);
      graph.relu(output, false);

      graph.run();
      println!("{}", graph.total_cost());
    }
}
