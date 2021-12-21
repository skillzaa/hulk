mod file_move_info;
mod sub_dirs_for_nav;
use sub_dirs_for_nav::sub_dirs_for_nav;
use crate::nav;
use crate::assets;
use brown::BrownError as Error;
use crate::bro;
use crate::unit;
use std::fs::DirEntry;
use crate::pure;
use crate::app_consts;
use file_move_info::FileMoveInfo;
use comrak::{markdown_to_html, ComrakOptions};

#[derive(Debug)]
pub struct Report {
    pub total_files:usize,
    pub total_data_subfolders:usize,
    pub files_data:Vec<FileMoveInfo>,
  }
impl Report{
    pub fn default()->Self{
        Report{
            total_files: 0,
            total_data_subfolders:0,
            files_data:Vec::new(),
        }
    }
}    

pub fn gen_report()->Result<Report,Error>{
let mut report = Report::default();

let dir_struct = pure::data_dir_struct_clean()?;

report.total_data_subfolders = dir_struct.len();  
//This is report but we need to create site structure since we needs its dirs for nav
clone_site_structure()?;
      
//--Outer loop around dirs
    for dir in dir_struct{
    let sub_dirs_for_nav_res = 
    sub_dirs_for_nav(&dir);
    
    let mut sub_dirs_for_nav:Vec<String>;    
        match sub_dirs_for_nav_res {
        Ok(item)=>{
            sub_dirs_for_nav = item;
        },
        Err(_e)=>{
            sub_dirs_for_nav = Vec::new();
        },
        }
    let navbar = 
    nav::get_nav(&dir,&sub_dirs_for_nav); 
    //------------------------    
    
    let files = get_files(&dir)?;
        //----------------------- 
          //--Inner loop around files
          for file in files {
        //--------------------------
// if file.file_type()        
let mut file_data = FileMoveInfo::default();
file_data.data_path = String::from(&dir);   
file_data.file_ext = bro::get_ext(&file)?;
file_data.site_path = String::from(pure::data_to_site_path_from_string
    (&dir)); 
file_data.nav = navbar.clone();   
file_data.file_name = bro::get_file_name(&file).unwrap();
//--------------------------
              match pure::is_md(&file) {
              true=>{
                file_data.is_md = true;
                let mut content = String::new();
                content.push_str(assets::get_default_header());    
                content.push_str(&file_data.nav);
                let md = get_content(&file);
                let html =
                comrak::markdown_to_html(&md,&ComrakOptions::default());
                content.push_str(&html);
                content.push_str(assets::get_default_footer());
    
                file_data.content = content;    
              },
              false=>{
                file_data.is_md = false;    
                file_data.content = get_content(&file);
              },
            }
        report.total_files = report.total_files +1;     
        report.files_data.push(file_data);    
        }
      }

Ok(report)
}
//------------Report Ends-------------    
fn get_content(file:&DirEntry)->String{
    let file_path = file.path();
    std::fs::
        read_to_string(&file_path).unwrap()
}  
fn clone_site_structure()->Result<bool,Error>{
let site_dir_struct = bro::
clone_dir_structure(
    app_consts::HULK_DATA_DIR,
     app_consts::HULK_SITE_DIR);
     
 match site_dir_struct {
 Ok(_item)=>{Ok(true)},
 Err(_e)=>{panic!("clone_dir_structure");},
 } 
}

fn get_files(dir:&String)->Result<Vec<DirEntry>,Error>{
let files_res = 
bro::get_files(dir);

match files_res {
    Ok(item)=>{Ok(item)},
    Err(e)=>{Err(e)},
    }
}   







#[cfg(test)]
mod tests {
    use super::*;
#[test]
fn report(){
let a = unit::create_demo_data_dir();    
    // let mut h = Report::default();
    let report = gen_report().unwrap();
    println!("{:#?}",report);
let _ = unit::teardown_data();    
}
}
  