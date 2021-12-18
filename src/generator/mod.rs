mod md;
mod non_md;
use brown::BrownError as Error;
use crate::brown as bro;
use md::md_file;
use non_md::non_md_file;
use crate::report::{gen_report,Report};
use crate::pure;
use crate::app_consts;
/*
 Steps : 
  later 1 : gen css file in site folder.
  2 : get the dir structure from data.
  -- there may be a lot of dirs but the operation is flat since all the DIR urls are in one vec. but for each dur there aremany file thus we have 2 loops
    --main loop over folders
    
*/
pub struct Generator {}

impl Generator{
  pub fn new(data_dir_name:String,
    site_dir_name:String)->Self{
      Generator{}
  }
pub fn gen(&self)->Result<String,Error>{
//>>>>>>>>>>>>>Gen Begin<<<<<<<<<<
//--Step  get report from Hunter 
//--Step  crate site forlder  
//--Step  get data forlder struct  
//--Step  crate main css file  
//--Step  run the 2 main loops for dirs and files
let report = gen_report()?;
let site_dir_created = pure::create_site_folder()?;
let data_dir_Struct = bro::
dir_structure_to_string(
  app_consts::HULK_DATA_DIR)?;
//---->--->---> First loop
for dir in data_dir_Struct {
  let files = bro::get_files(&dir)?;
  //---->--->---> second loop  
  for file in files{
    //----- dhamal 
  }
//---->--->---> second loop ends  
}  
//---->--->---> First loop ends  
todo!();
//>>>>>>>>>>>>>Gen Ends<<<<<<<<<<<
}//gen ends

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