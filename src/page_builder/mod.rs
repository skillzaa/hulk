use crate::assets::*;
use super::nav::Nav;
use std::fs::DirEntry;
use crate::*;
use comrak::{markdown_to_html, ComrakOptions};

pub struct Page {
    file_path:String,   
}
impl Page{
    pub fn new(file_path:&String)->Self{
        Page {
            file_path:String::from(file_path),
        }
    }
    pub fn run(&self)->bool{
        let header = get_default_header();  
        let navbar = get_navbar(&self.file_path);  
        let content = get_content(&self.file_path);  
        let footer = get_default_footer();  
        true
        // let dest_clean = get_dest_clean_for_md(&file);
        // create_n_write_file(dest_clean,content)  
    }
    
}

//-----------------------------------------
fn get_dest_clean_for_md(file:&DirEntry)->String{
let file_path_string = direntry_to_path(&file).unwrap();        
let dest = file_path_string.replacen("data", "site", 1);
 let d = dest.replace("./","");
 let dd = d.replace("md","html");
 println!("{:?}",dd);
        dd
}
fn get_content(file_path:&String)->String{
let cont = std::fs::read_to_string(&file_path)
.unwrap();
comrak::markdown_to_html(&cont,&ComrakOptions::default())
}

fn get_navbar(path:&String)->String{
  let nb = NavBar::new(path).unwrap();
  let r = nb.gen_navbar();
  r
}