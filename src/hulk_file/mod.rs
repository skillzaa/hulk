use crate::bro;
use crate::bro::BrownError as Error;
mod get_nav;
use get_nav::get_nav;
mod nav;
use crate::assets::*;
use comrak::{markdown_to_html, ComrakOptions};

use std::fs::DirEntry;
use crate::pure;
use crate::unit;
use crate::app_consts;

#[derive(Debug,PartialEq, Eq)]
pub struct HulkFile {
    pub data_dir_path:String,
    pub site_dir_path:String,
    pub file_read_path:String,
    pub file_name:String,
    pub file_ext:String,
    pub nav:String,
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

//--> Step 05 : File read path    
let file_read_path = format!("{}/{}.{}",data_dir_path,file_name,file_ext);
//--> Step 06 : File Name    
let is_md = pure::is_md(&file);
//--> Step 07 : File write path    

//======= finally
let nav = get_nav(&dir_name_string);   

//==================================
        Ok( HulkFile {
            data_dir_path,
            site_dir_path,
            file_read_path,
            file_name,
            file_ext,
            nav,
            is_md,
        } )

}//new fn
pub fn get_file_write_path(&self)->String{
 if self.is_md {
    format!("{}/{}.html",self.site_dir_path,self.file_name)
 }else {
    format!("{}/{}.{}",self.site_dir_path,self.file_name,self.file_ext)
 }
}
pub fn get_content(&self)->String{
    if self.is_md {
        self.get_md_content()
    }else {
        self.get_non_md_content()
    }
}

fn get_md_content(&self)->String{
let mut page = String::new();
page.push_str(get_default_header());
//-----Actual Read Content------------------
let raw_content = std::fs::
    read_to_string(&self.file_read_path).unwrap();
//-----------------------------------------
let md_to_html = 
comrak::markdown_to_html(&raw_content,&ComrakOptions::default());
page.push_str(&self.nav);
page.push_str(md_to_html.as_str());
page.push_str(get_default_footer());
page
}
fn get_non_md_content(&self)->String{
    let mut page = String::new();
// page.push_str(get_default_header());
//-----Actual Read Content------------------
let content = std::fs::
    read_to_string(&self.file_read_path).unwrap();

page.push_str(&content);
// page.push_str(get_default_footer());
page
}
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
        //   let cont = s.get_content();
        //   println!("{:#?}",cont);
      }
  }
  
  
}

}
