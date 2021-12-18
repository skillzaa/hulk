use crate::brown as bro;
use std::fs::DirEntry;
use crate::BrownError as Error;
/// I am trying 2 techniques 
///     : Fake data , TDD create fn with output and then insert code.
/// remove all the operations and just create oure componenets and later provide io etc as a layer 
/// A nav is specific for each Dir, one nav for one dir will be used for all its files including index.
/// every dir nav has path upto that dir and then different segments each corresponding to one sub_dir
pub fn get_nav(dir_path:&String)->String{
let sub_dirs = 
bro::get_dirs(&dir_path);

    match  sub_dirs {
    Ok(items)=>{
        let mut nav = String::from("<header id='header'><nav class='links' style='--items: 1;'>");
        for sd in items{
        let sd_str = bro::direntry_to_path(&sd).unwrap();    
        // let sd_str = String::from(&sd.path().as_path());    
        let link = 
        format!("<a href='./{}/{}/index.html'>{}</a>",dir_path,sd_str,sd_str);    
        nav.push_str(&link);
        }
        nav.push_str(&"</nav></header>");
        return nav
    },
    Err(_e)=>{return "no sub dirs".to_string();},
    }


}
// fn get_sub_dirs(){

// }
pub fn fake()->String{
    let static_data = r#"<header id='header'><nav class='links' style='--items: 1;'><a href='./site/aa/bb/cc/index.html'>cc</a><a href='./site/aa/bb/dd/index.html'>dd</a><a href='./site/aa/bb/ee/index.html'>ee</a><a href='./site/aa/bb/ff/index.html'>ff</a></nav></header>"#;
    static_data.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
#[test]
fn excellent_test_no_confusion(){
// let dir_path = String::from("site/aa/bb");    
// let mut sub_dirs:Vec<String> = Vec::new();
// sub_dirs.push("cc".to_string());
// sub_dirs.push("dd".to_string());
// sub_dirs.push("ee".to_string());
// sub_dirs.push("ff".to_string());
// let r = get_nav(&dir_path);
// let static_data = r#"<header id='header'><nav class='links' style='--items: 1;'><a href='./site/aa/bb/cc/index.html'>cc</a><a href='./site/aa/bb/dd/index.html'>dd</a><a href='./site/aa/bb/ee/index.html'>ee</a><a href='./site/aa/bb/ff/index.html'>ff</a></nav></header>"#;
// assert_eq!(r.as_str(),static_data);
}
//--mod tests
}