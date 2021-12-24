// use crate::index_all;
use crate::generator;
use crate::help;
use crate::init;
use crate::app_consts;
use brown::*; 
use brown::BrownError as Error;
use std::env;
use yansi::Paint;
// use crate::brown;
pub fn cli_gui() {
 let gap = "                           "; 
  println! ("{}",gap);
  println! ("{}",gap);
  let bar_top = "================== HULK =======================";
  println!("{}",Paint::green(bar_top));
  println! ("{}",gap);
  println! ("{}",gap);

  let args: Vec<String> = env::args().collect();

  let arg_one_option = args.get(1);

  match arg_one_option {
    Some(value)=>{
      let v = value.as_str();
        match v {
          "gen" => {
            let gen_result = generator::gen();
                      match gen_result {
                        Ok(_r)=>{
                            println!("Markdown to Html generation completed");  
                        },
                        Err(e)=> println!("{:?}",&e),
                      }

                  
                  // let r = index_all::run();
                  // match r {
                  //   Ok(_r)=>{
                  //   println!("index files generated successfully");
                  //   },
                  //   Err(e)=> println!("{:?}",&e),
                  // }
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
          // "gui" => gui::gui(),
          "help" => help::help(),
          _ => println!("command not found ==> {}", Paint::red(v)),
        }
    },
    None => {help::help()},
  }
  
  println! ("{}",gap);
  let bar_bot = "================== HULK ENDS ======================= "; 
  println!("{}",Paint::green(bar_bot));
  println! ("{}",gap);


}//main
