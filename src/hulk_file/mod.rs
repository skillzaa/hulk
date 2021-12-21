use crate::bro;
use crate::bro::BrownError as Error;
use std::fs::DirEntry;
use crate::pure;
use crate::app_consts;
#[derive(Debug)]
pub struct HulkFile {
    pub data_path:String,
    pub site_path:String,
    pub file_name:String,
    pub file_ext:String,
    pub nav:String,
    pub content:String,
    pub is_md:bool,
}

impl HulkFile{
    pub fn new(dir_name_string:&String,file:&DirEntry)->Result<Self,Error>{
//--> Step 00 : Path must exist    

//--> Step 01 : data_path    
let data_path = String::from(dir_name_string);   

//--> Step 02 : Site Path    
let site_path = String::from(
pure::data_to_site_path_from_string
(&dir_name_string)); 

//--> Step 03 : Ext    
let file_ext = bro::get_ext(&file)?;

//--> Step 04 : File Name    
let file_name = bro::get_file_name(&file).unwrap();

//--> Step 05 : File Name    
let is_md = pure::is_md(&file);

//======= finally
let nav = String::new();   
let content = String::new();   
//==================================
        Ok( HulkFile {
            data_path,
            site_path,
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
let main_dir = 
pure::data_dir_struct_clean().unwrap();

for dir in main_dir{
    let files = 
    bro::get_files(&dir).unwrap();
        for file in files {
        let f = HulkFile
            ::new(&dir, &file);
        println!("{:#?}",f);    
        }
}
}

}