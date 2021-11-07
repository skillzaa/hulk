use std::fs;
use crate::core::*;
use brown::Hdir;
use std::io::{Error};
use std::path::Path;


pub fn index()->Result<bool,Error>{
  let hdir = Hdir::new()?;  
  let mut html = String::new();
    html.push_str(get_default_header());
    html.push_str(get_default_nav());
    html.push_str("<h1>Home Page</h1><br/><hr/>");
    html.push_str("<table>");
    let files = hdir.get_files_by_ext("./site", "html")?;
    
      for file in files {
        let path_buf = file.path();
        let file_name = get_file_name(path_buf.as_path());
        //------------ templating ---------------------------
              html.push_str("<tr>");
            
              let name = format!("<td><a href=\"./{:?}\">{}</a> </td>",&file.path(),&file_name);
             html.push_str(&name);
            
             let link = format!("<td>{}</td>",file_name);
             html.push_str(&link);
            
             html.push_str("</tr>");
            
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
    let file_name = path.file_name().unwrap();
    let file_name_str = file_name.to_str().map(|s| s.to_string()).unwrap();
    file_name_str
  }