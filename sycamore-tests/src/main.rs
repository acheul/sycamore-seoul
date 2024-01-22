mod app;

use sycamore::prelude::*;
use sycamore_seoul::{Resizer, StyleLength};
use gloo_console::log;

fn main() {
  sycamore::render(app::App)
}