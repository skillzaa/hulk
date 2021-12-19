mod md;
mod non_md;
use brown::BrownError as Error;
use crate::brown as bro;
use md::md_file;
use non_md::non_md_file;
use crate::report::{gen_report,Report};
use crate::pure;
use crate::app_consts;

pub fn gen()->Result<bool,Error>{
//-->>>>>>>>>>>>>Gen Begin<<<<<<<<<<

//--Step  00.1 crate main css file  
pure::create_css();

//--Step 01 delete old site folder 
bro::remove_dir_brute(app_consts::HULK_SITE_DIR)?;

//--Step 02 get report from Hunter 
let report = gen_report()?;

//--Step 03 crate site forlder  
let site_dir_created = pure::create_site_folder()?;

// let data_dir_Struct = bro::
// dir_structure_to_string(
//   app_consts::HULK_DATA_DIR)?;

//\\\\\\\\\\\-- THE LOOP --\\\\\\\\\\\\\\
for info in report.files_data {
  let file_name_raw = 
  format!("{}/{}.html",info.site_path,info.file_name);
  let file_name = 
  file_name_raw.replace(' ', "_");  
  
//-----write files to disk
  bro::write_to_file
  (&file_name, &info.content)?;

}  
//\\\\\\\\\\\-- THE LOOP ENDS --\\\\\\\\\\\
Ok(true)
}//gen ends

  mod tests {
    use super::*;
  #[test]
  fn run_test(){
  let res = gen().unwrap();
  }
  //----test mod ends
}