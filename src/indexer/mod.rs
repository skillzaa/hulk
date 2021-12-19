use crate::assets::*;
use crate::bro;
use brown::BrownError as Error;

use std::fs::DirEntry;
use std::path::Path;
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
  pub fn run(&self)->Result<bool,Error>{      
  let mut html = index_page_start_html();

    let files = self.get_files()?;
    
      for file in files {
          match self.flat_loop(&file) {
          Ok(item)=>{
              html.push_str(&item);            
          },
          Err(_e)=>{},
          }
      }
      //======================================
        html.push_str(get_default_footer());
        self.create_index_file()?;
        self.write_index_file(&html)?;
        Ok(true)

    }
    fn write_index_file(&self,html:&String)->Result<bool,Error>{
          bro::write_to_file(&self.index_file_path.as_str(), &html)?;
          Ok(true)
    }  
    fn create_index_file(&self)->Result<bool,Error>{
          
      bro::create_file_brute(&self.index_file_path)?;
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
}//Indexer Ends here 

fn index_page_start_html()->String{
  let mut html = String::new();
  html.push_str(get_default_header());
  html.push_str(get_default_nav());
  html.push_str("<h1>Home Page</h1><br/><hr/>");
  html.push_str("<table>");
  html.push_str("<tr><td>File Name</td></tr>");
  html
}

//-----------------------------------
//-----------------------------------
//------------TESTS------------------
//-----------------------------------
//-----------------------------------
mod tests {
use std::ops::Index;

use super::*;
#[test]
fn create_indexer(){
  let i = 
  Indexer::new("site/exploration".to_string()).unwrap();
  // we can check non-pub items also
  assert_eq!(i.dir_path,"site/exploration");
  assert_eq!(i.index_file_path,"site/exploration/index.html");
}
#[test]
fn get_files_test(){
  let i = 
  Indexer::new("site/exploration".to_string()).unwrap();
  let files = i.get_files();
println!("{:?}",files);
}
#[test]
fn first(){
let indexer = Indexer::new("site".to_string())
.unwrap();
let run = indexer.run().unwrap();
// println!("{:?}",run);
assert!(run);
}
#[test]
fn site_exploration_test(){
let indexer =  Indexer::new("site/exploration".to_string()).unwrap();
// let files = 
// indexer.get_files().unwrap();
let run = indexer.run();
assert!(run.is_ok());

}
#[test]
fn site_a2_test(){
let indexer =  Indexer::new("site/a2".to_string()).unwrap();
// let files = 
// indexer.get_files().unwrap();
let run = indexer.run();
assert!(run.is_ok());

}
#[test]
fn create_index_file_test(){
  let indexer =  Indexer::new("site/a2".to_string()).unwrap();
  // assert!(indexer.is_ok);
  let s 
  =indexer.create_index_file();
assert!(s.is_ok());
}
}