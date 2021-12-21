use crate::bro;
use crate::bro::BrownError as Error;
mod get_nav;
use get_nav::get_nav;
mod nav;

use std::fs::DirEntry;
use crate::pure;
use crate::unit;
use crate::app_consts;
#[derive(Debug,PartialEq, Eq)]
pub struct HulkFile {
    pub data_dir_path:String,
    pub site_dir_path:String,
    pub file_path:String,
    pub file_name:String,
    pub file_ext:String,
    pub nav:String,
    pub content:String,
    pub is_md:bool,
}

impl HulkFile{
pub fn new(dir_name_string:&String,file:&DirEntry)->Result<Self,Error>{
//--> Step 00 : Path must exist    
//--> Step 01 : data_dir_path    
let data_dir_path = String::from(dir_name_string);   
//--> Step 02 : Site Path    
let site_dir_path = String::from(
pure::data_to_site_path_from_string
(&dir_name_string)); 
//--> Step 03 : Ext    
let file_ext = bro::get_ext(&file)?;
//--> Step 04 : File Name    
let file_name = bro::get_file_name(&file).unwrap();
//--> Step 05 : File Name    
let file_path = format!("{}/{}.{}",site_dir_path,file_name,file_ext);
//--> Step 06 : File Name    
let is_md = pure::is_md(&file);

//======= finally
let nav = get_nav();   
let content = String::new();   
//==================================
        Ok( HulkFile {
            data_dir_path,
            site_dir_path,
            file_path,
            file_name,
            file_ext,
            nav,
            content,
            is_md,
        } )

}//new fn
}

mod tests {
    use super::*;
#[test]
fn basic(){
  let _ = unit::create_demo_data_dir();
  let dirs = 
  pure::data_dir_struct_clean()
  .unwrap();

  for dir in dirs{
      let files = bro::get_files(&dir).unwrap();
      for file in files {
          let s = HulkFile
          ::new(&dir, &file)
          .unwrap();
          println!("{:#?}",s);
      }
  }
  
  
}

}
