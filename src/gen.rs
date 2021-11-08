use crate::core::{get_default_footer,get_default_header,get_default_nav,get_dark_css};
use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
use std::path::Path;
use brown::Hdir;
use std::io::{Error};


pub fn gen()->Result<bool,Error>{
  let hdir = Hdir::new()?;
  
let header = get_default_header();
let footer = get_default_footer();

let all_md = hdir.get_files_by_ext("data", "md");
      match all_md {
        Ok(mds)=>{
          for md in mds {
              let path = md.path();
              let md_str = fs::read_to_string(&path.as_path());
              match md_str {
                Ok(md_str_ok)=>{
                  let html = markdown_to_html(&md_str_ok, &ComrakOptions::default());
                  //--Build the Page..
                  let mut page = String::from(header);
                  page.push_str(get_default_nav());
                  page.push_str(&html);
                  page.push_str(&footer);
                  let file_name = get_file_name(&path.as_path());
                  
                  let mut write_path_str = String::from("./site/");
                  write_path_str.push_str(&file_name);
                  write_path_str.push_str(".html");
                  
                  let write_path = Path::new(&write_path_str);
                  fs::write(write_path, page).unwrap();
  
                },
                Err(_e)=> continue,
              }
          }
        },
        Err(e)=> return Err(e),
      }
        //Example code
        // let len = file_name_str.len();
        // let ext = &file_name_str[len-3..];
        // let file_wo_ext = &file_name_str[0..len-3];

// --- Writing css this does not need to be in loop    
    let write_path_css = Path::new("./site/main.css");
    let _ = hdir.create_file(write_path_css.to_str().unwrap());
    let css = get_dark_css();
      fs::write(write_path_css, css).unwrap();
      println!("Markdown to Html conversion complete......");
  
   Ok(true)
  }

  fn get_file_name(path:&Path)->String{
    let file_name = path.file_name().unwrap();
    let file_name_str = file_name.to_str().map(|s| s.to_string()).unwrap();
    file_name_str
  }