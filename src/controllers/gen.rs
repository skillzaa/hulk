use crate::core::{get_default_footer,get_default_header,get_default_nav,get_dark_css};
use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
use std::path::Path;
use super::super::brown;

pub fn gen(){


    let header = get_default_header();
    let footer = get_default_footer();
  
    let all_files = brown::get_files_from_dir("./data");
   //===== the loop ===== 
    for entry in all_files.iter(){
    
      let path = entry.path();
      
      let md = fs::read_to_string(&path).unwrap();
    
      let html = markdown_to_html(&md.to_string(), &ComrakOptions::default());
  
      let mut page = String::from(header);
          page.push_str(get_default_nav());
          page.push_str(&html);
          page.push_str(&footer);
      //==============================
        let file_name = path.file_name().unwrap();
        let file_name_str = file_name.to_str().map(|s| s.to_string()).unwrap();
        let len = file_name_str.len();
        let ext = &file_name_str[len-3..];
        let file_wo_ext = &file_name_str[0..len-3];
        
        if let ".md" = &*ext {
        //    println!("ext::{:?}",ext);
            let mut s = String::from("./site/");
            s.push_str(file_wo_ext);
            s.push_str(".html");
            // println!("S::{:?}",s);
  
            let write_path = Path::new(&s);
            fs::write(write_path, page).unwrap();
          }
  
      //==============================
    }
    let write_path_css = Path::new("./site/main.css");
    brown::create_file(write_path_css.to_str().unwrap());
    let css = get_dark_css();
            fs::write(write_path_css, css).unwrap();
      println!("Markdown to Html conversion complete......");
  
    
  }
