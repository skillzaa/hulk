use crate::bro;
use crate::app_consts;
use crate::bro::BrownError;
// there is an error/POINT in brown lib in which it return error of the source folder is empty SO take care of it here and make just the dest dir top level manually.
pub fn clone_data_to_site_folder()
->Result<Vec<String>,BrownError>{
let r = bro::clone_dir_structure(
          app_consts::HULK_DATA_DIR,
           app_consts::HULK_SITE_DIR);
match r {
Ok(item)=>{
return Ok(item);
},
Err(e)=>{
    if e == BrownError::DirEmpty {
        let  v:Vec<String> = vec![app_consts::HULK_SITE_DIR.to_string()];
        
        let f = bro::create_dir_brute(app_consts::HULK_SITE_DIR)?;
        
        match f {
            true=>{return Ok(v)},
            false=>{return Err(e)},
        }

    }else{
        panic!("failed to clone the data folder into site folder");
    }
},
}          
}