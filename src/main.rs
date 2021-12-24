mod cli_gui;
mod generator;
mod assets;
mod hulk_file;
mod pure;
mod indexer;
mod help;
mod init;
mod unit;
// mod gui;
mod app_consts;
use brown as bro;


fn main(){
  //--we can place it here as well
  use cli_gui::cli_gui;
  cli_gui();
  println!("ok");
}