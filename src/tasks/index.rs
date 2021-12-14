use std::fs;
use crate::assets::*;
use brown as bro;
use brown::BrownError as Error;

// use std::io::{Error};
use std::path::Path;


pub fn index()->Result<bool,Error>{
  
  //delteold index if exists
  let _ = bro::remove_file("./site/index.html");  
  
  let mut html = String::new();
    html.push_str(get_default_header());
    html.push_str(get_default_nav());
    html.push_str("<h1>Home Page</h1><br/><hr/>");
    html.push_str("<table>");
    html.push_str("<tr><td>File Name</td></tr>");
    let files = bro::get_files_by_ext("site", "html")?;
    
      for file in files {
        //------------ templating ---------------------------
        html.push_str("<tr><td>");
        
        let file_name = bro::get_file_name(&file)?;
        
        let file_name_readable = 
        replace_char(&file_name, '_', ' ');
        let mut link = String::from(&file_name);
        link.push_str(".html");
                      
        let  anchor= format!("<a href= \"{}\">{}</a>",&link,&file_name_readable);
        
        html.push_str(&anchor);
        html.push_str("</td></tr>");
            
      }
      //======================================
           
        html.push_str(get_default_footer());
        //println!("{}",&html);
        //This is safe file creation we need create in any case
        bro::create_file_brute("./site/index.html")?;
        //bro::create_file("./site/index.html")?;
        let res = 
        bro::write_to_file("./site/index.html", &html);
        match res {
          Ok(_r)=> return Ok(true),
          Err(e)=> return Err(e),
        }
              
}      
   

  fn get_file_name(path:&Path)->String{
    let file_name = path.file_stem().unwrap();
    let file_name_str = file_name.to_str().map(|s| s.to_string()).unwrap();
    file_name_str
  }
  fn replace_char(sample:&String,strip:char,rep:char)->String{
    let mut r = String::from("");
    for char in sample.chars() {
      if char == strip {
        r.push(rep);
      }else {
        r.push(char);
      }
    }
  r
  }