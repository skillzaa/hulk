use crate::bro::BrownError as Error;
use crate::bro;
use crate::app_consts;
pub fn data_dir_struct_clean()->Result<Vec<String>,Error>{
    let mut dir_struct = 
    bro::dir_structure_to_string(
        app_consts::HULK_DATA_DIR
    )?;
    //-- MY FIRST
    let dir_struct_clean = 
    dir_struct.iter_mut().
    map(|i|i.replace("./",""))
    .collect::<Vec<String>>();
  
    Ok(dir_struct_clean)
    }
