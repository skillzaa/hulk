mod generator;
mod assets;
use brown::*; 
use brown::BrownError as Error;

use std::env;
mod tasks;
mod util;
use yansi::Paint;

use tasks::{help,index,init};
mod gui;
// use util::
use brown; 

fn main() {
  // Generator::run();
  // return;
  //init::init();
  println!("                                                  ");
  println!("                                                  ");
  let bar_top = "================== HULK =======================";
  println!("{}",Paint::green(bar_top));
  println!("                                                  ");
  println!("                                                  ");

  let args: Vec<String> = env::args().collect();

  let arg_one_option = args.get(1);

  match arg_one_option {
    Some(value)=>{
      let v = value.as_str();
        match v {
          "gen" => {
            let gen_result = generator::run();
                      match gen_result {
                        Ok(_r)=>{
                            println!("Markdown to Html generation completed..");
                            
                        },
                        Err(e)=> println!("{:?}",&e),
                      }
                  let r = index::index();
                  match r {
                    Ok(_r)=>{
                        println!("index generated successfully");
                        
                    },
                    Err(e)=> println!("{:?}",&e),
                  }
          },
         "init" => {
          let init_result = init::init();
                      match init_result {
                        Ok(_r)=>{
                            println!("Project initialized....");
                            
                        },
                        Err(e)=> println!("{:?}",&e),
                      } 
          },
          "gui" => gui::gui(),
          "help" => help::help(),
          _ => println!("Command not found ==> {}", Paint::red(v)),
        }
      
    },
    None => {help::help()},
  }
  
  println!("                                                  ");
  let bar_bot = "================== HULK ENDS ======================= "; 
  println!("{}",Paint::green(bar_bot));
  println!("                                                  ");

}//main
