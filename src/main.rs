use std::env;
mod hulk;

fn main() {
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
          "gen" => {hulk::gen();hulk::index()},
          "init" => hulk::init(),
          // "index" => hulk::index(),
          "help" => hulk::help(),
          _ => println!("Command not found ==> {}", v),
        }
      
    },
    None => {hulk::help()},
  }
  
  println!("================== HULK ENDS ======================= ");

}//main
