mod cli_gui;
mod generator;
mod assets;
mod report;
mod pure;
mod nav;
mod indexer;
mod help;
mod init;
mod unit;
// mod gui;
mod app_consts;
use brown as bro;
use cli_gui::cli_gui;


fn main(){
  cli_gui();
}