use std::fs::DirEntry;
use crate::*;

pub fn non_md_files(file:&DirEntry)->bool{
    // let file_name = get_file_name(&file).unwrap();
    // println!("Non md file:: {}",file_name);
    let content = get_content(&file);  
    let dest_clean = get_dest_clean(&file);
    create_n_write_file(dest_clean,content)  
}

fn get_dest_clean(file:&DirEntry)->String{
let file_path_string = direntry_to_path(&file).unwrap();        
let dest = file_path_string.replacen("data", "site", 1);
 let d = dest.replace("./","");
        d
}
fn get_content(file:&DirEntry)->String{
  let file_path = file.path();
  std::fs::
    read_to_string(&file_path).unwrap()
 
}
fn create_n_write_file(dest_clean:String,content:String)->bool{
  let _b = 
  create_file_brute(dest_clean.as_str())
  .unwrap();
    let _r = write_to_file
    (&dest_clean, &content).unwrap();
  true
}