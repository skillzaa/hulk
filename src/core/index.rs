use super::Brown;
struct B{}
impl Brown for B{}
impl B {
  fn new()->B{
    B{}
  }
}

pub fn index(){
    let b = B::new();
    println!("Generating Index Files......");

 println!("index generation completed....!!!!!!");
}//init ends
