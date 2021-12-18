use crate::app_consts;
pub fn data_to_site_path_from_string(data_path:&String)->String{
    
    let dest = data_path
    .replacen(
        app_consts::HULK_DATA_DIR,
         app_consts::HULK_SITE_DIR,
          1);

    let d = dest.replace("./","");
            d
}