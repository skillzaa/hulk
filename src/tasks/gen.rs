use std::io::Error;
use crate::site_gen;
pub fn gen()->Result<bool,Error>{
let site_ok = site_gen::run();
assert!(site_ok);
let files_ok = gen_files();
todo!();
}
/*
Step : 1 get folder struct of data folder
step : 2 convert it into site folder urls
step : 3 place both in same obj
step : 4 loop picking from one and droppping into the other
*/
fn gen_files(){
    
}


mod tests {
use super::*;
#[test]
fn first(){
    let c = gen().unwrap();
}




}