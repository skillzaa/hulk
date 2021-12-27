use crate::bro;
use crate::app_consts;
use crate::bro::BrownError as Error;
use crate::assets;
pub fn create_demo_data_dir()->Result<bool,Error>{
let _ = teardown_data();
create_dirs()?;
create_files()?;
Ok(true)
}
pub fn demo_data_dir_struct()->Vec<String>{
    let s:Vec<String> = vec!
    [ 
format!("{}",app_consts::HULK_DATA_DIR),
format!("{}/{}",app_consts::HULK_DATA_DIR,"a1"),
format!("{}/{}",app_consts::HULK_DATA_DIR,"a2"),
format!("{}/{}",app_consts::HULK_DATA_DIR,"a3"),
format!("{}/{}",app_consts::HULK_DATA_DIR,"a4"),
format!("{}/{}",app_consts::HULK_DATA_DIR,"a4/a4a"),
format!("{}/{}",app_consts::HULK_DATA_DIR,"a4/a4b"),
];
s
}
fn create_dirs()->Result<bool,Error>{

let dirs = demo_data_dir_struct();
let dirs_str = dirs.iter()
.map(|i|&**i).collect::<Vec<&str>>();

bro::create_dir_structure(&dirs_str)?;
Ok(true)
}
fn create_files()->Result<bool,Error>{

let mock_md_data = assets::demo_md();
let data_dir_struct = demo_data_dir_struct();

    for d in data_dir_struct {
    let p = format!("{}/demo_file.md",d);    
    bro::write_to_file(&p, &mock_md_data)?;
    }
Ok(true)
}
pub fn teardown_site(){
let _ = bro::remove_dir_brute(app_consts::HULK_SITE_DIR);
}
pub fn teardown_data(){
let _ = bro::remove_dir_brute(app_consts::HULK_DATA_DIR);
}
mod tests {
    use super::*;
    #[test]
fn test_unit_module(){
  let t = 
  create_demo_data_dir();
  assert!(t.is_ok());
}
    

}