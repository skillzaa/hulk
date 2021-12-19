mod file_move_info;
use crate::nav;
use brown::BrownError as Error;
use crate::brown as bro;
use std::fs::DirEntry;
use crate::pure;
use crate::app_consts;
use file_move_info::FileMoveInfo;
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
let mut hunter = Report::default();
let dir_struct = pure::data_dir_struct_clean()?;
hunter.total_data_subfolders = dir_struct.len();  
let site_dir_struct = bro::
      clone_dir_structure(
          app_consts::HULK_DATA_DIR,
           app_consts::HULK_SITE_DIR);
           
       match site_dir_struct {
       Ok(item)=>{},
       Err(_e)=>{panic!("clone_dir_structure");},
       }    
      //--Outer loop around dirs
      for dir in dir_struct{
        // let navbar = nav::get_nav(&dir); 
        //-----get files
        let files_res = bro::get_files(&dir);
        let files:Vec<DirEntry>;    
        match files_res {
            Ok(item)=>{
                files = item;
            },
            Err(_e)=>{continue;},
            }
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
file_data.nav = nav::get_nav(&file_data.site_path);   
file_data.file_name = bro::get_file_name(&file).unwrap();
// file_data.content = get_content(&file);
        //--------------------------
              match pure::is_md(&file) {
              true=>{
                file_data.is_md = true;    
              },
              false=>{
                file_data.is_md = false;    
              },
            }
        hunter.total_files = hunter.total_files +1;     
        hunter.files_data.push(file_data);    
        }
      }
      Ok(hunter)
    }
fn get_content(file:&DirEntry)->String{
    let file_path = file.path();
    std::fs::
        read_to_string(&file_path).unwrap()
}  
    #[cfg(test)]
mod tests {
    use super::*;
#[test]
fn report(){
    // let mut h = Report::default();
    let report = gen_report().unwrap();
    println!("{:#?}",report);
}
}
  