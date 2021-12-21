
use brown::BrownError as Error;
use crate::bro;
use crate::report::{gen_report};
use crate::pure;
use crate::app_consts;

pub fn gen()->Result<bool,Error>{
//-->>>>>>>>>>>>>Gen Begin<<<<<<<<<<
//--Step 00 delete old site folder 
bro::remove_dir_brute(app_consts::HULK_SITE_DIR)?;

//--Step 01 crate site forlder  
pure::create_site_folder()?;

//--Step  002 create main css file  
pure::create_css();

//--Step 03 get report from Hunter 
let report = gen_report()?;

//\\\\\\\\\\\-- THE LOOP --\\\\\\\\\\\\\\
for info in report.files_data {
  let file_name = get_file_name(&info.site_path,&info.file_name); 

  //-----write files to disk
  bro::write_to_file
  (&file_name, &info.content)?;

}  
//\\\\\\\\\\\-- THE LOOP ENDS --\\\\\\\\\\\
Ok(true)
}//gen ends

fn get_file_name(path:&String,file_name:&String)->String{
  let file_name_raw = 
  format!("{}/{}.html",path,file_name);
  let file_name = 
  file_name_raw.replace(' ', "_");  
  file_name
}

mod tests {
    use super::*;
  #[test]
  fn run_test(){
  let res = gen();
  assert!(res.is_ok());
  }
  //----test mod ends
}