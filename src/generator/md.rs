use crate::assets::*;
use std::fs::DirEntry;
use crate::*;

use comrak::{markdown_to_html, ComrakOptions};
// use comrak;
pub fn md_file(file:&DirEntry,navbar:&String)->String{
    let content = get_content(&file,navbar);  
    //let dest_clean = get_dest_clean_for_md(&file);
    content
    // create_n_write_file(dest_clean,content)  
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
//-----Actual Read Content------------------
let p = 
direntry_to_path(&file).unwrap();

let cont = std::fs::
    read_to_string(&p).unwrap();
//-----------------------------------------
let md_to_html = 
comrak::markdown_to_html(&cont,&ComrakOptions::default());
page.push_str(&navbar);
page.push_str(md_to_html.as_str());
page.push_str(get_default_footer());
page
}


fn create_n_write_file(dest_clean:String,content:String)->bool{
  let _b = 
  create_file_brute(dest_clean.as_str())
  .unwrap();
    let _r = write_to_file
    (&dest_clean, &content).unwrap();
  true
}
