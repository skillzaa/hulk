use std::fs::DirEntry;
mod generator;
use generator::run;
use brown::*; 
use brown::BrownError as Error;

pub fn main() {
 let run_result = run().unwrap();
 println!("{:?}",run_result); 
}//main
