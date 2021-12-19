use crate::assets::get_dark_css;
use crate::brown as bro;
use crate::app_consts;

pub fn create_css()->bool{
let css = get_dark_css();
let css_file_path = format!("{}/{}",
    app_consts::HULK_SITE_DIR,    
app_consts::CSS_FILE_NAME);

let css_file = 
bro::create_file_brute(&css_file_path.to_string());
match css_file {
Ok(item)=>{
    
    let w = 
    bro::write_to_file(&css_file_path,
         &css.to_string());
return true;
},
Err(_e)=>{return false;},
}

}

mod tests{
    use super::*;
#[test]
fn one(){
  let r = create_css();
  assert!(r);  
}    
}