use std::env;
mod controllers;
mod index;
mod gen;
mod core;
use brown::Hdir; //for controllers main.rs does not need it 

fn main() {
  // index::index();
  println!("                                                  ");
  println!("                                                  ");
  println!("================== HULK ======================= ");
  println!("                                                  ");
  println!("                                                  ");

  let args: Vec<String> = env::args().collect();

  let arg_one_option = args.get(1);

  match arg_one_option {
    Some(value)=>{
      let v = value.as_str();
        match v {
          "gen" => {
            let _ = gen::gen();
            let _ = index::index(); //waoooo!!!!!!!
          },
         "init" => controllers::init(),
          "index" => {
                      let r = index::index();
                      match r {
                        Ok(_r)=>{
                            println!("index generated successfully");
                            
                        },
                        Err(e)=> println!("{:?}",&e),
                      }
                    },
          "help" => controllers::help(),
          _ => println!("Command not found ==> {}", v),
        }
      
    },
    None => {controllers::help()},
  }
  
  println!("                                                  ");
  println!("================== HULK ENDS ======================= ");
  println!("                                                  ");

}//main
