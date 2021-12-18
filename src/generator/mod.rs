mod md;
mod non_md;
use brown::BrownError as Error;
use crate::brown as bro;
use std::fs::DirEntry;
use crate::nav;
use md::md_file;
use non_md::non_md_file;
/*
 Steps : 
  later 1 : gen css file in site folder.
  2 : get the dir structure from data.
  -- there may be a lot of dirs but the operation is flat since all the DIR urls are in one vec. but for each dur there aremany file thus we have 2 loops
    --main loop over folders
    -- inner loop over each file in folder
    2-a: get file header
    2-b: get file content
    2-c: markdown to html the content
    2-d: get the nav bar - using site folder urls
    2-e: create the file
    2-f: place file in site folder   
*/
pub struct Generator {}

impl Generator{
  pub fn new(data_dir_name:String,
    site_dir_name:String)->Self{
      Generator{}
  }
  pub fn gen(&self)->Result<String,Error>{
    todo!();
    }

}//impl
  mod tests {
    use super::*;
  #[test]
  fn data_dir_struct_test(){
  let g = Generator
  ::new("data_test".to_string(),
"site".to_string());   
  let p = g.data_dir_struct().unwrap();
  let resp = [
  "data_test", 
  "data_test/b",
  "data_test/a",
  "data_test/a/a1",
  "data_test/a/a1/a2",
  "data_test/a/a1/a2/a3-b",
  "data_test/a/a1/a2/a3-a"
  ];
  let stng = resp.into_iter()
  .map(|i|i.to_string()).collect::<Vec<String>>();
  assert_eq!(stng,p);
  // println!("{:?}",p);  
  }
  #[test]
  fn site_dir_struct_test(){
    let g = Generator
    ::new("data_test".to_string(),
  "site".to_string());   
    let p = g.site_dir_struct().unwrap();
    let resp = [
    "site", 
    "site/b",
    "site/a",
    "site/a/a1",
    "site/a/a1/a2",
    "site/a/a1/a2/a3-b",
    "site/a/a1/a2/a3-a"
    ];
    let stng = resp.into_iter()
    .map(|i|i.to_string()).collect::<Vec<String>>();
    assert_eq!(stng,p);
    println!("{:?}",p);  
  }

  #[test]
  fn run_test(){
    let g = Generator
    ::new("data_test".to_string(),
  "site".to_string());   
  let res = g.run();
  }
  //----test mod ends
}