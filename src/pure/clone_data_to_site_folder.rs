use crate::bro;
use crate::app_consts;
use crate::bro::BrownError as Error;
pub fn clone_data_to_site_folder()->Result<Vec<String>,Error>{
bro::clone_dir_structure(
          app_consts::HULK_DATA_DIR,
           app_consts::HULK_SITE_DIR)
}