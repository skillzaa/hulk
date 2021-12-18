use crate::assets::get_dark_css;
use crate::brown as bro;
use crate::app_consts;

pub fn create_css()->bool{
let css = get_dark_css();
let css_file = 
bro::create_file_brute("main.css");
match css_file {
Ok(item)=>{
    let path = format!("{}/{}",
    app_consts::HULK_SITE_DIR,    
app_consts::CSS_FILE_NAME);
    let w = 
    bro::write_to_file(&path, &css.to_string());
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