use brown::BrownError as Error;
use crate::bro;
use crate::hulk_file::HulkFile;
use crate::pure;
use crate::app_consts;

pub fn gen()->Result<bool,Error>{
//-->>>>>>>>>>>>>Gen Begin<<<<<<<<<<
//--Step 00 delete old site folder 
let _ = bro::remove_dir_brute(app_consts::HULK_SITE_DIR);
//--Step 01 clone/CREATE data folder structure  
//--if this fail the gen process will break
pure::clone_data_to_site_folder();
//--Step  002 create main css file  
//pure::create_css();
//\\\\\\\\\\\-- THE LOOP --\\\\\\\\\\\\\\
let dirs = 
pure::data_dir_struct_clean()?;

  for dir in dirs{
      let files_res = bro::get_files(&dir);
      
      match files_res {
      Ok(files)=>{
        for file in files {
          let hulk_file = HulkFile
          ::new(&dir, &file)?;
          
          let _ = bro::write_to_file(
            hulk_file.get_file_write_path().as_str(), 
            &hulk_file.get_content());
      }
        
      },
      Err(_e)=>{continue;},
      }

      
}
//\\\\\\\\\\\-- THE LOOP ENDS --\\\\\\\\\\\
Ok(true)
}//gen ends



#[cfg(test)]
mod tests {
use super::*;
use crate::unit;
use crate::generator;
#[test]
//I have created a demo data folder- from that I created a site folder and checked that all the files are there 
fn basic(){
  let x = unit::create_demo_data_dir().unwrap();
  assert!(x);
  let y = generator::gen().unwrap();
  assert!(y);
  //---- now check if each file exists or not
  //------------- this is the main gen test
  let site_dir_struct = bro::dir_structure_to_string(app_consts::HULK_SITE_DIR);
  assert!(site_dir_struct.is_ok());
  let site_dir = site_dir_struct.unwrap();
  // -- one file in eacj folder
  for d in site_dir{
    let files = bro::
    get_files_by_ext(&d.as_str(),"html").unwrap();
    let file_name = files[0].file_name();
    assert_eq!(file_name,"demo_file.html");
    println!("{:#?}",files[0].file_name());
    assert_eq!(files.len(),1);
  }
  //----------------------------------------------
  bro::remove_dir_brute(app_consts::HULK_DATA_DIR);
  bro::remove_dir_brute(app_consts::HULK_SITE_DIR);
}
#[test]
fn organic(){
  
  let y = generator::gen().unwrap();
  assert!(y);
 
}
}
