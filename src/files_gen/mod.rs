use crate::assets::{get_default_footer,get_default_header,get_default_nav,get_dark_css};
use comrak::{markdown_to_html, ComrakOptions};
use std::fs::{DirEntry};
use bro::navbar::NavBar;
use std::path::Path;
use brown as bro;
use std::io::{Error};
use std::fs;
// use nav

#[derive(Debug)]
pub struct FilesGen{
  file:DirEntry,
  url: String, 
}
impl FilesGen{
  pub fn new(file:DirEntry, url:String)->Self{
    FilesGen {file,url,}
  }
  pub fn run(&self,dir_entry:&DirEntry)->Result<bool,Error> {
    let file_name = bro::get_file_name(&dir_entry)?;
    let content = 
    self.get_page()?;
    write_file(&file_name, &content)?;
    Ok(true)
  }
  pub fn get_file(&self)->Result<String,Error> {
    let file_name = 
    bro::get_file_name(&self.file)?;
    let content = 
    self.get_page()?;
    Ok(content)
  }
  fn get_nav<'a>(&'static self)->Result<String,Error>{
    let s: &'static str = &self.url.as_str();
    // print_me(&owned_string);
    let n = 
    NavBar::new(s);
    match n {
    Ok(item)=>{
    Ok(item.gen_navbar())
    },
    Err(_e)=>{Err(_e)},
    }
  }
  fn get_page(&self)->Result<String,Error>{
    //--Build the Page..
    let mut page = String::new();
    page.push_str(get_default_header());
    let n = self.get_nav();
      match n {
      Ok(item)=>{
        page.push_str(&item);
      },
      Err(_e)=>{},
      }
    let md_to_html = md_to_html(&self.file)?;
    page.push_str(md_to_html.as_str());
    page.push_str(get_default_footer());
    Ok(page)
    }
}

//-------------------------------
fn write_file(file_name:&str,content:&String)->Result<bool,Error>{
let mut write_path_str = String::from("./site/");
write_path_str.push_str(file_name);
write_path_str.push_str(".html");
let write_path = Path::new(&write_path_str);
fs::write(write_path, content)?;
Ok(true)
}
//................................
fn md_to_html(dir_entry:&DirEntry)->Result<String,Error>{
let md = fs::read_to_string(&dir_entry.path().as_path())?;
let html = markdown_to_html(&md, &ComrakOptions::default());
Ok(html)    
}
fn gen_dark_css(){
let write_path_css = Path::new("./site/main.css");
let _ = bro::create_file(write_path_css.to_str().unwrap());
let css = get_dark_css();
fs::write(write_path_css, css).unwrap();
println!("Dark Theme css generated..");

}


#[cfg(test)]
mod tests{
use super::*;

#[test]
fn first(){
  let files = bro::get_files("data").unwrap();
  let url = "data/snippets";
  // let file = files.first().unwrap().to_owned();
    for file in files {
      let f = FilesGen::new(file,url.to_string());
      let n = f.get_page().unwrap();
      println!("===>{:?}",n);

    }  
}


}//mod