use std::fs;
use crate::core::*;
use brown::Hdir;
use std::io::{Error};
use std::path::Path;


pub fn index()->Result<bool,Error>{
  
  let hdir = Hdir::new()?;  
  //delteold index if exists
  let _ = hdir.remove_file("./site/index.html");  
  
  let mut html = String::new();
    html.push_str(get_default_header());
    html.push_str(get_default_nav());
    html.push_str("<h1>Home Page</h1><br/><hr/>");
    html.push_str("<table>");
    html.push_str("<tr><td>File Name</td></tr>");
    let files = hdir.get_files_by_ext("site", "html")?;
    
      for file in files {
        let path_buf = file.path();
        //------------ templating ---------------------------
        html.push_str("<tr><td>");
        
        let pth = path_buf.as_path();
        let file_name = get_file_name(pth);
        let file_name_human = replace_char(&file_name, '_', ' ');
        
        let link = path_buf.as_path().file_name().unwrap().to_str().unwrap();

              
        let  anchor= format!("<a href= \"{}\">{}</a>",link,&file_name_human);
        
        html.push_str(&anchor);
        html.push_str("</td></tr>");
            
      }
      //======================================
           
        html.push_str(get_default_footer());
        //println!("{}",&html);
        //This is safe file creation we need create in any case
        fs::File::create("./site/index.html")?;
        //hdir.create_file("./site/index.html")?;
        let res = std::fs::write("./site/index.html", &html);
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