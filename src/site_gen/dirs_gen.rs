use crate::assets::{get_default_footer,get_default_header,get_default_nav,get_dark_css};
use std::io::Error;
use brown as bro;
#[derive(Debug)]
pub struct Generator{
    data_folder_name:&'static str,
    site_folder_name:&'static str,
}
impl Generator{
  pub fn new(data_folder_name:&'static str,
    site_folder_name:&'static str)->Self{
      Generator{
        data_folder_name,
        site_folder_name,
      }
    }
  pub fn remove_site_folder(&self)->bool{
    let _ =brown::remove_dir_brute("site");
    true
  }  
  pub fn run(&self)->Result<bool,Error>{
    let site_struct:Vec<String> = self.copy_site_struct()?;
    self.create_site_struct
    (site_struct)
  }
  pub fn copy_site_struct(&self)->Result<Vec<String>,Error>{
    let site_struct = bro::tasks::copy_dir_structure::run("data", "site");
    match site_struct {
    Ok(item)=>{
      return Ok(item);
      },
      Err(_e)=>{return Err(_e);
      },
    }
  }
  pub fn create_site_struct(&self,dir_struct:Vec<String>)->Result<bool,Error>{
    let dir_struct_str = 
    dir_struct.iter().map(|s|&**s)
    .collect();
    bro::tasks::create_dir_structure::
    run(&dir_struct_str)
  }
}
mod tests {
use super::*;
#[test]
fn test_remove_site(){
let g = get_new();
let f = g.run();
println!("{:#?}",&f);
}

}


fn get_new()->Generator{
  let x = Generator::new(
    "data",
    "site",
  );
  x
}