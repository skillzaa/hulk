use crate::bro;
use crate::bro::BrownError as Error;
use crate::app_consts;
use crate::unit;
pub fn sub_dirs_for_nav(dir:&String)->Result<Vec<String>,Error>{
let sub_dirs = bro::get_dirs(dir)?;
let paths = bro::
direntry_to_path_all(&sub_dirs, false)?;

let mut last_segments:Vec<String> = Vec::new();

for p in paths {
  let r = get_last_segment(&p);
  last_segments.push(r);  
}
Ok(last_segments)    
}
fn get_last_segment(path_String:&String)->String{
let split = path_String.split('/');
let a = split.last();
    match a {
    Some(item)=>{ item.to_string() },
    None=>{ "".to_string() },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
#[test]
fn basic(){
let a = unit::create_demo_data_dir().unwrap();    
let s = 
sub_dirs_for_nav
(&app_consts::HULK_DATA_DIR.to_string())
.unwrap();
// println!("{:?}",s);
let expected:Vec<String> = vec![
"a2".to_string(),
"a1".to_string(),
"a3".to_string(),
"a4".to_string(),
   ];
assert_eq!(s,expected);
let z = unit::teardown_data();
}
#[test]
fn get_last_segment_test(){
let p = String::from("aa/bb/cc");
let s = get_last_segment(&p);
println!("{:?}",s);
assert_eq!(s,"cc");
}


}