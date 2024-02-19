mod app;

use sycamore::prelude::*;
use sycamore_seoul::*;
use hashbrown::HashMap;

fn main() {
  sycamore::render(app::App)
}

fn new_id(list: &Vec<usize>) -> usize {
  let len = list.len();
  (1..len+1).rev().find(|x| !list.contains(x)).unwrap_or(0)
}