use std::env;
mod tasks;
use tasks::gen;
use tasks::help;
use tasks::index;
use tasks::init;
mod assets;
use brown; //for controllers main.rs does not need it 

fn main() {
  //init::init();
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
          "help" => help::help(),
          _ => println!("Command not found ==> {}", v),
        }
      
    },
    None => {help::help()},
  }
  
  println!("                                                  ");
  println!("================== HULK ENDS ======================= ");
  println!("                                                  ");

}//main
