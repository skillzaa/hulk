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
    --main loop over folders.
*/
pub struct Generator {}

impl Generator{
  pub fn new(data_dir_name:String,
    site_dir_name:String)->Self{
      Generator{}
  }
pub fn gen(&self)->Result<String,Error>{
//-->>>>>>>>>>>>>Gen Begin<<<<<<<<<<
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
for info in report.files_data {
    let file_name_raw = 
      format!("{}/{}.{}",info.site_path,info.file_name,info.file_ext);
      let file_name = 
      file_name_raw.replace(' ', "_");  
      // println!("{:?}",file_name);
      //-----write files to disk
let f = bro::write_to_file
(&file_name, &"ddd".to_string());
}  
//---->--->---> First loop ends  
Ok("fff".to_string())
//---->>>>>>>>>>>>>Gen Ends<<<<<<<<<<<
}//gen ends

}//impl ends
  mod tests {
    use super::*;
  #[test]
  fn run_test(){
    let g = Generator
    ::new("data_test".to_string(),
  "site".to_string());   
  let res = g.gen().unwrap();
  }
  //----test mod ends
}