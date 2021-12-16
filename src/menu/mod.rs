use std::io::BufRead;
// use qndr;
use crate::brown::*;
use crate::BrownError as Error;

pub fn get_menu(dir_path:String)->Result<String,Error>{
let mut navbar = String::new();
navbar.push_str(navbar_start());

let sub_dirs = 
get_dirs(&dir_path).unwrap();

for sd in sub_dirs {
    let file_name = 
    direntry_to_path(&sd)?;
    
    let last_segment = get_last_segment(&file_name);
    let f = format!("<a href='./{}/index.html'>{}</a>",&file_name,&last_segment);
    navbar.push_str(&f);      
}
navbar.push_str(navbar_end());
Ok(navbar)
}
fn get_last_segment(url:&String)->String{
//  let is_last_seperator =    
 let mut last = String::new();   
 for i in url.chars(){
    if i == '/' {
        last = String::new(); //dont use "let" here
    }else {
        last.push(i);
    }
 }
last
}
fn navbar_start()->&'static str{
    let r = r#"<header id='header'><nav class='links' style='--items: 1;'>"#;
    r    
    }
fn navbar_end()->&'static str{
    let r = r#"</nav></header>"#;
    r
}
    


mod tests{
    use super::*;
#[test]
fn one(){
    let sd = get_menu("site".to_string());
    println!("{:?}",sd);
}
#[test]
fn get_last_segment_test(){
    let l = 
    get_last_segment(&"aa/bb/cc/dd".to_string());
    // println!("{:?}",l);
    assert_eq!(l,"dd");
}

}