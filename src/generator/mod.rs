use brown::BrownError as Error;
use crate::bro;
use crate::hulk_file::HulkFile;
use crate::pure;
use crate::app_consts;

pub fn gen()->Result<bool,Error>{
//-->>>>>>>>>>>>>Gen Begin<<<<<<<<<<
//--Step 00 delete old site folder 
let _ = bro::remove_dir_brute(app_consts::HULK_SITE_DIR);
//--Step 01 clone data folder structure  
pure::clone_data_to_site_folder()?;

//--Step  002 create main css file  
pure::create_css();
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


mod tests {
    use super::*;
  #[test]
  fn run_test(){
  let res = gen();
  assert!(res.is_ok());
  }
  //----test mod ends
}