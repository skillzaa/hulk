use crate::bro;
use crate::app_consts;
use crate::bro::BrownError;
// there is an error/POINT in brown lib in which it return error of the source folder is empty SO take care of it here and make just the dest dir top level manually.
pub fn create_site_folder(){
let r  = bro::clone_dir_structure(
          app_consts::HULK_DATA_DIR,
           app_consts::HULK_SITE_DIR);

  match r {
  Ok(_item)=>{
    ()
  },
  Err(_e)=>{
    let f = bro::create_dir_brute
    (app_consts::HULK_SITE_DIR);
    //--------match f
            match f {
            Ok(_item)=>{
            ()
            },
            Err(_e)=>{
                panic!("failed to generate site folder");
            },
            }
    },
  }         
}

mod tests {
    use super::*;
#[test]
fn one(){
let _ = bro::remove_dir_brute(app_consts::HULK_SITE_DIR);    

    let r  =create_site_folder();
// check if site folder exists
let success = bro::path_exists(app_consts::HULK_SITE_DIR);    
assert!(success)
// println!("{:?}",r);
}    
}//mod tests end