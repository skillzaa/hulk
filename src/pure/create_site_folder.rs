use crate::brown as bro;
use crate::app_consts;
use crate::BrownError as Error;

pub fn create_site_folder()->Result<Vec<String>,Error>{
bro::clone_dir_structure(
          app_consts::HULK_DATA_DIR,
           app_consts::HULK_SITE_DIR)
}