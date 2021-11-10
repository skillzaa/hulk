use crate::core::{get_default_footer,get_default_header,get_default_nav,get_dark_css};
use comrak::{markdown_to_html, ComrakOptions};
use std::fs::{self, DirEntry};
use std::path::Path;
use brown as bro;
use std::io::{Error};


pub fn gen()->Result<bool,Error>{

let all_md = bro::get_files_by_ext("data", "md")?;
     
    for md in all_md {
      let result = static_loop(&md);

      match result {
        Ok(_r)=>{
          let file_name = bro::get_file_name(&md)?;
          println!("......... generate html for {}",&file_name);
        },
        Err(_e)=>{
          let file_name = bro::get_file_name(&md)?; 
          println!("failed to generate html for {}",&file_name);
          continue;
        },
      }  
    } 
    gen_dark_css();  
   Ok(true)
  }
fn static_loop(dir_entry:&DirEntry)->Result<bool,Error> {
        let file_name = bro::get_file_name(&dir_entry)?;
        let content = get_page(&dir_entry)?;
        write_file(&file_name, &content)?;
        Ok(true)
}
fn write_file(file_name:&str,content:&String)->Result<bool,Error>{
  let mut write_path_str = String::from("./site/");
  write_path_str.push_str(file_name);
  write_path_str.push_str(".html");
  let write_path = Path::new(&write_path_str);
  fs::write(write_path, content)?;
  Ok(true)
}
fn get_page(dir_entry:&DirEntry)->Result<String,Error>{
 //--Build the Page..
 let mut page = String::new();
 page.push_str(get_default_header());
 page.push_str(get_default_nav());
 let md_to_html = md_to_html(dir_entry)?;
 page.push_str(md_to_html.as_str());
 page.push_str(get_default_footer());
  Ok(page)
}
fn md_to_html(dir_entry:&DirEntry)->Result<String,Error>{
  let md = fs::read_to_string(&dir_entry.path().as_path())?;
  let html = markdown_to_html(&md, &ComrakOptions::default());
   Ok(html)    
}

pub fn gen_dark_css(){
  let write_path_css = Path::new("./site/main.css");
  let _ = bro::create_file(write_path_css.to_str().unwrap());
  let css = get_dark_css();
    fs::write(write_path_css, css).unwrap();
    println!("Dark Theme css generated..");

}