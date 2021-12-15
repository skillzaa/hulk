use std::fs::DirEntry;

use brown::*; 
use brown::BrownError as Error;

pub fn main() {
 let run_result = run().unwrap();
 println!("{:?}",run_result); 
}//main

fn run()->Result<bool,Error>{
// Step 01: clone_data_to_site
clone_data_to_site()?;
// Step 02: get_dir_struct_clean
let dir_struct_clean = 
get_dir_struct_clean()?;
//---------------------------------
// Step 03: Loop for each sub-dir
  for dir in dir_struct_clean{
  
      let files = get_files(&dir).unwrap();
      single_folder_files(&files);
  }
  Ok(true)
}//run
fn single_folder_files(files:&Vec<DirEntry>){
for file in files {
  
let content = get_content(&file);  
let dest_clean = get_dest_clean(&file);

let a = create_n_write_file(dest_clean,content);  
println!("{:?}",a);
  }
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
fn clone_data_to_site()->Result<Vec<String>,Error>{
clone_dir_structure("data","site")  
}

fn get_dir_struct_clean()->Result<Vec<String>,Error>{
  let mut dir_struct = 
  dir_structure_to_string("data")?;
  //-- MY FIRST
  let dir_struct_clean = 
  dir_struct.iter_mut().
  map(|i|i.replace("./",""))
  .collect::<Vec<String>>();

Ok(dir_struct_clean)
}