mod cli_gen;
mod cli_index;
mod cli_init;
mod cli_help;

mod common;
use common::*;
use std::env;
use yansi::Paint;
// use crate::brown;
pub fn cli_gui() {
  print_hulk_top_bar();

  let args: Vec<String> = env::args().collect();

  let arg_one_option = args.get(1);

  match arg_one_option {
    Some(value)=>{
      let v = value.as_str();
        match v {
          "gen" => {
            cli_gen::cli_gen();
            //---- now index
            cli_index::cli_index();          
          },
         "index" => {
          cli_index::cli_index();          
        },
         "init" => {
         cli_init::cli_init();
          },
          // "gui" => gui::gui(),
          "help" => cli_help::cli_help(),
          _ => println!("command not found ==> {}", Paint::red(v)),
        }
    },
    None => {cli_help::cli_help()},
  }
  print_hulk_bot_bar();

}//main
