mod flat;
use flat::*;
mod non_md;
mod md;
use non_md::non_md_files;
use md::md_files;
use brown::*; 
use brown::BrownError as Error;

pub fn run()->Result<bool,Error>{
// Step 01: clone_data_to_site
clone_data_to_site()?;
// Step 02: get_dir_struct_clean
let dir_struct_clean = 
get_dir_struct_clean()?;
//---------------------------------
// Step 03: Loop for each sub-dir
  for dir in dir_struct_clean{
  let files = get_files(&dir).unwrap();
  //--------------------------
  for file in files {
    match is_md(&file) {
    true=>{
        md_files(&file);
    },
    false=>{
        non_md_files(&file);
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
  dir_structure_to_string("data")?;
  //-- MY FIRST
  let dir_struct_clean = 
  dir_struct.iter_mut().
  map(|i|i.replace("./",""))
  .collect::<Vec<String>>();

Ok(dir_struct_clean)
}