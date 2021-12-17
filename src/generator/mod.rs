mod flat;
use flat::*;
mod non_md;
use crate::nav::get_nav;
use crate::assets::get_dark_css;
mod md;
use md::md_files;
use non_md::non_md_files;
use brown::*; 
use brown::BrownError as Error;


pub fn run()->Result<bool,Error>{
// Step 00: Add css file
let _ = create_file_brute("site/main.css"); 
let _ = write_to_file("site/main.css", 
&get_dark_css().to_string()); 
// Step 01: clone_data_to_site
clone_data_to_site()?;
// Step 02: get_dir_struct_clean
let dir_struct_clean = 
get_dir_struct_clean()?;
//---------------------------------
// Step 03: Loop for each sub-dir
  for dir in dir_struct_clean{
    //----fake data-----
    let sd = vec!["cc","dd","ee"];
    let mutated:Vec<String>  = sd.iter()
    .map(|s| String::from(*s)).collect();
    //------------------------
  let navbar = get_nav(&dir,&mutated);  
  let files = get_files(&dir).unwrap();
  //--------------------------
  for file in files {
    match is_md(&file) {
    true=>{
        md_files(&file ,&navbar);
    },
    false=>{
        non_md_files(&file,&navbar);
    },
    }
  }    
  //--------------------------    
  }
  Ok(true)
}//run

fn clone_data_to_site()->Result<Vec<String>,Error>{
clone_dir_structure("data","site")  
}
fn get_dir_struct_clean()->Result<Vec<String>,Error>{
  let mut dir_struct = 
  dir_structure_to_string("site")?;
  //-- MY FIRST
  let dir_struct_clean = 
  dir_struct.iter_mut().
  map(|i|i.replace("./",""))
  .collect::<Vec<String>>();

Ok(dir_struct_clean)
}

mod tests {
  use super::*;
#[test]
fn run_test(){
  run();
}
}