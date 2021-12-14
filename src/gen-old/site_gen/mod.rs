mod dirs_gen;
use dirs_gen::Generator;

pub fn run()->bool{
  let g = 
  Generator::new("data",
  "site");
  let f = g.run();
  println!("{:#?}",&f);
  true
}

mod tests{
use super::*;
#[test] 
fn run_test(){
  super::run();
}  
}