use std::env;
mod controllers;
mod core;
mod brown;
fn main() {
  // controllers::init();
  // controllers::index();
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
          "gen" => {controllers::gen();controllers::index()},
         "init" => controllers::init(),
          "index" => controllers::index(),
          "help" => controllers::help(),
          _ => println!("Command not found ==> {}", v),
        }
      
    },
    None => {controllers::help()},
  }
  
  println!("================== HULK ENDS ======================= ");
  println!("                                                  ");

}//main
