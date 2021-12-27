use crate::bro;
mod indx;

use indx::Indexer;
use crate::app_consts;
use crate::bro::BrownError as Error;

pub fn run()->Result<bool,Error>{
let site_struct = 
bro::dir_structure_to_string(app_consts::HULK_SITE_DIR)?;
let clean:Vec<String> = site_struct.iter()
.map(|i|i.replace("./","")).collect(); 
//--------------------------------------------
//--------------------------------------------
    for d in clean {
        let i = Indexer::new(d);
        match i {
        Some(ii)=>{
         let _ = ii.run();
        },
        None=>{continue;},
        }        
    }
//--------------------------------------------
//--------------------------------------------
Ok(true)
}


#[cfg(test)]
mod tests {
use super::*;
use crate::unit;
use crate::generator;
#[test]
fn basic(){
  let x = unit::create_demo_data_dir().unwrap();
  assert!(x);
  let y = generator::gen().unwrap();
  assert!(y);
  //-- now run indexer
  let i = run();

  let site_dir_struct = bro::dir_structure_to_string(app_consts::HULK_SITE_DIR);
  
  assert!(site_dir_struct.is_ok());
  
  let site_dir = site_dir_struct.unwrap();
  //-!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
  for d in site_dir{
      let files = bro::
      get_files_by_ext(&d.as_str(),"html").unwrap();
      //-- 2 files in each folder - one demo_file.html and other index.html
    assert_eq!(files.len(),2);

    //---now check for an index.html IN EVERY FOLDE
    let mut has_index_file = false;
    for f in files {
        let file_name = f.file_name();
        // println!("{:?}",file_name);
        if file_name == "index.html" {
            has_index_file = true;
        }
    }
    // every folder must have has_index_file = true
    assert!(has_index_file);
  } //checking each folder ends here
    //-!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
  bro::remove_dir_brute(app_consts::HULK_DATA_DIR);
  bro::remove_dir_brute(app_consts::HULK_SITE_DIR);
}
}
