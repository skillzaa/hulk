use crate::assets::{get_default_footer,get_default_header,get_default_nav,get_dark_css};
use comrak::{markdown_to_html, ComrakOptions};
use brown as bro;
#[derive(Debug)]
pub struct Generator{
  data_folder_name:String,
  site_folder_name:String,
}
impl Generator{
  pub fn new(data_folder_name:String,
    site_folder_name:String)->Self{
      Generator{
        data_folder_name,
        site_folder_name,
      }
    }
  pub fn remove_site_folder(&self)->bool{
    let _ =brown::remove_dir_brute("site");
    true
  }  
  pub fn create_site_dir(&self){
    let dir_struct = bro::tasks::copy_dir_structure::run("data", "site").unwrap();
    // println!("{:?}",&dir_struct);
    let dir_struct_str:Vec<&str> = dir_struct
    .iter().map(|s| &**s).collect();
    let site_struct = bro::tasks::
    create_dir_structure::run(&dir_struct_str);
    assert!(site_struct.is_ok());

  }
}



mod tests {
use super::*;
#[test]
fn test_remove_site(){
let g = get_new();
println!("{:#?}",&g);
}

}


fn get_new()->Generator{
  let x = Generator::new(
    "data".to_string(),
    "site".to_string(),
  );
  x
}