#![allow(unused_mut)]
mod card;
use card::card;
use fltk::{prelude::*, enums::*, *};
// use fltk_flow::Flow;

pub fn gui() {
let a = app::App::default().with_scheme(app::Scheme::Gtk);

let mut win = window::Window::default().with_size(640, 480);
 let mut flow = card();
// -----------------------------------------   
win.end();win.resizable(&flow);
win.show();a.run().unwrap();
}
