//You do not need to link every mod to top level one 
use crate::assets::*;
use super::navbar::NavBar;
use std::fs::DirEntry;
use crate::*;

use comrak::{markdown_to_html, ComrakOptions};
// use comrak;
pub fn md_files(file:&DirEntry,navbar:&String)->bool{
    // let file_name = get_file_name(&file).unwrap();
    // println!("Non md file:: {}",file_name);
    let content = get_content(&file,navbar);  
    let dest_clean = get_dest_clean_for_md(&file);
    create_n_write_file(dest_clean,content)  
}
fn get_dest_clean_for_md(file:&DirEntry)->String{
let file_path_string = direntry_to_path(&file).unwrap();        
let dest = file_path_string.replacen("data", "site", 1);
 let d = dest.replace("./","");
 let dd = d.replace("md","html");
 println!("{:?}",dd);
        dd
}
fn get_content(file:&DirEntry,navbar:&String)->String{
let mut page = String::new();
page.push_str(get_default_header());
let p = 
direntry_to_path(&file).unwrap();
let cont = std::fs::
    read_to_string(&p).unwrap();
let md_to_html = 
comrak::markdown_to_html(&cont,&ComrakOptions::default());
page.push_str(&navbar);
page.push_str(md_to_html.as_str());
page.push_str(get_default_footer());
page
}
// fn get_content(file:&DirEntry)->String{
//   let header =  get_default_header();
//   let footer =  get_default_footer();
//   let file_path = file.path();
//   let cont = std::fs::
//     read_to_string(&file_path).unwrap();
//   let content = 
//   comrak::markdown_to_html(md, &comarkOptions);  
//     let final_content = 
//     format!("{}{}{}", header,content,footer);
//     final_content
// }
fn create_n_write_file(dest_clean:String,content:String)->bool{
  let _b = 
  create_file_brute(dest_clean.as_str())
  .unwrap();
    let _r = write_to_file
    (&dest_clean, &content).unwrap();
  true
}
fn get_navbar(path:&String)->String{
  let nb = NavBar::new(path).unwrap();
  let r = nb.gen_navbar();
  r
}