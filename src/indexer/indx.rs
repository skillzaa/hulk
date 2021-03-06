use crate::assets;
use crate::bro;
use crate::nav;

use crate::unit;
use crate::app_consts;
use brown::BrownError as Error;
use std::fs::DirEntry;
/// Indexer is given the path of any folder and it will generate an index of all the (html) files there.
#[derive(Debug)]
pub struct Indexer{
dir_path:String,
index_file_path:String,
}

impl Indexer{
/// The path should exist
pub fn new(dir_path:String)->Option<Self>{
        match bro::path_exists(&dir_path) {
        true=>{
          let index_file_path = format!("{}/index.html",&dir_path);
                Some(Indexer {
                        dir_path,
                        index_file_path,
                    })
        },
        false=>{return None},
        }
    }
fn get_files(&self)->Result<Vec<DirEntry>,Error>{
    let f = bro::get_files_by_ext(&self.dir_path, "html")?;
        Ok(f)
    }
pub fn run(&self)->Result<i32,Error>{ 
  let files_res = self.get_files();
  match files_res {
  Ok(files)=>{
    let _ = self.has_files(files);
    Ok(1)
  },
  Err(_e)=>{
    self.create_empty_index();
    Ok(0)      
  },
  }
}     

fn has_files(&self,files:Vec<DirEntry>)->Result<i32,Error>{      
  let mut html = self.index_page_start_html();
  let mut counter = 0;    
    
      for file in files {
          match self.flat_loop(&file) {
          Ok(item)=>{
              html.push_str(&item);
              counter += 1;            
          },
          Err(_e)=>{},
          }
      }
      //======================================
        html.push_str(assets::get_default_footer());
        bro::create_file_brute(&self.index_file_path)
        ?;
        self.write_index_file(&html)?;
        Ok(counter)
}
fn write_index_file(&self,html:&String)->Result<bool,Error>{
      bro::write_to_file(&self.index_file_path.as_str(), &html)?;
      Ok(true)
}  
  
fn flat_loop(&self,file:&DirEntry)->Result<String,Error>{
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
        Ok(html)
    }
//--------------------------------------------    
fn index_page_start_html(&self)->String{
  let mut html = String::new();
  html.push_str(assets::get_default_header());
  let x = format!("{}",self.dir_path.as_str());
  let n = nav::get_nav(&x);
  html.push_str(&n);
  html.push_str("<h1>Index Page</h1><br/><hr/>");
  html.push_str("<table>");
  html.push_str("<tr><td>File Name</td></tr>");
  html
}
fn create_empty_index(&self){
  let mut s = String::new();
  let h = assets::get_default_header();
  s.push_str(&h.to_string());
  s.push_str("<a href='/index.html'>Home</a>");
  s.push_str("<h3>No files found...</h3>");
  s.push_str(assets::get_default_footer());
   let _ = bro::create_file_brute(&self.index_file_path);
   let _ = self.write_index_file(&s);
}
}//Indexer Ends here 


mod tests {
  use super::*;
#[test]
fn one(){
  let idx = 
  Indexer::new("site/inshallah".to_string())
  .unwrap();
let x = idx.run();
println!("{:?}",x);
}  
}
