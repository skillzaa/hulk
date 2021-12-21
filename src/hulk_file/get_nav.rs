use super::nav;
use crate::bro;
use crate::app_consts;
use crate::unit;
use crate::bro::BrownError as Error;
//Remember dir_path is from data folder and you have to return dir_path of site folder- even the site folder may not exist at the moment but we know its struct is just like data folder except the top level folder name;
pub fn get_nav(dir_path:&String)->String{
let mut sub_dirs:Vec<String> = Vec::new(); 

let sub_dirs_res = 
bro::get_dirs(dir_path);
        match sub_dirs_res {
        Ok(item)=>{
            let paths = item.into_iter()
            .map(|i|bro::direntry_to_path(&i).unwrap()).collect::<Vec<String>>();

            for path in paths{
                let components = std::path::Path::new(&path).components();
                let last = components.last();
                let ll = last.unwrap();
                let lll = ll.as_os_str();
                let llll = lll.to_str().map(|s| s.to_string()).unwrap();
                //println!("{:?}",llll);
                sub_dirs.push(llll);
                (&dir_path, &sub_dirs);
            }
// here is the hidden problem --change the dir_path in to site dir path before sending
let site_dir_path = dir_path.replace(
    app_consts::HULK_DATA_DIR,
    app_consts::HULK_SITE_DIR,
);
        return nav::nav(&site_dir_path,&sub_dirs)
        },
        Err(_e)=>{ return "no sub dirs".to_string();},
        }
}


#[cfg(test)]
mod tests {
use super::*;

#[test]
fn basic(){
 let _ = unit::create_demo_data_dir();
let d = get_nav(&"data".to_string());
// println!("{:?}",d);

let static_data = "<header id='header'><nav class='links' style='--items: 1;'><a href='./site/a2/index.html'>a2</a><a href='./site/a1/index.html'>a1</a><a href='./site/a3/index.html'>a3</a><a href='./site/a4/index.html'>a4</a></nav></header>";

assert_eq!(d,static_data);
//  let _ = unit::teardown_data();
}
}