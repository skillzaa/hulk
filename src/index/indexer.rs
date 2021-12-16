use crate::assets::*;
use crate::brown as bro;
use brown::BrownError as Error;

use std::fs::DirEntry;
use std::path::Path;
/// Indexer is given the path of any folder and it will generate an index of all the (html) files there.
pub struct Indexer{
dir_path:String,
}

impl Indexer{
    pub fn new(dir_path:String)->Self{
        Indexer {
            dir_path
        }
    }
    fn get_files(&self)->Result<Vec<DirEntry>,Error>{
    let f = bro::get_files_by_ext("site", "html")?;
        Ok(f)
    }
    pub fn run(&self)->Result<bool,Error>{      
  //delteold index if exists
  let _ = bro::remove_file("site/index.html");  
  let mut html = index_page_start_html();

    let files = self.get_files()?;
    
      for file in files {
          let i = self.flat_loop(&file);
        html.push_str(&i);            
      }
      //======================================
        html.push_str(get_default_footer());
        create_n_write_file(&html)?;
        Ok(true)

    }
    fn flat_loop(&self,file:&DirEntry)->String{
        let mut html = String::new();
        html.push_str("<tr><td>");
        let file_name = bro::get_file_name(&file)?;
        
        let file_name_readable = 
        file_name.replace( '_', " ");
        let mut link = String::from(&file_name);
        link.push_str(".html");
                        
        let  anchor= format!("<a href= \"{}\">{}</a>",&link,&file_name_readable);
        
        html.push_str(&anchor);
        html.push_str("</td></tr>");
        html
    }
//--------------------------------------------    
}//Indexer Ends here 
fn create_n_write_file(html:&String)->Result<bool,Error>{
    bro::create_file_brute("site/index.html")?;
        //bro::create_file("./site/index.html")?;
        let res = 
        bro::write_to_file("site/index.html", &html);
        match res {
          Ok(_r)=> return Ok(true),
          Err(e)=> return Err(e),
        }
}  
fn index_page_start_html()->String{
  let mut html = String::new();
  html.push_str(get_default_header());
  html.push_str(get_default_nav());
  html.push_str("<h1>Home Page</h1><br/><hr/>");
  html.push_str("<table>");
  html.push_str("<tr><td>File Name</td></tr>");
  html
}
