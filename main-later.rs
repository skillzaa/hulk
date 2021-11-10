use std::env;
mod controllers;
mod index;
mod init;
mod gen;
mod core;
use brown; //for controllers main.rs does not need it 

fn main() {
  // gen::gen();
  return;
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
            let gen_result = gen::gen();
                      match gen_result {
                        Ok(_r)=>{
                            println!("Markdown to Html generation completed..");
                            
                        },
                        Err(e)=> println!("{:?}",&e),
                      }
            //let _ = index::index(); //waoooo!!!!!!!
          },
         "init" => {//init::init()
        
          },
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
