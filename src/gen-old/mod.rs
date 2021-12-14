use std::io::Error;
use crate::site_gen;
use crate::files_gen;
use files_gen::FilesGen;
use brown as bro;

#[derive(Debug)]
struct Binder{
  data_url:String,
  site_url:String,  
}
impl Binder{
    pub fn new(data_url:&String,site_url:&String)->Self{
        Binder {
            data_url: data_url.clone(),
            site_url:site_url.clone(),
        }
    }
}

pub fn gen()->Result<bool,Error>{
let site_ok = site_gen::run();
assert!(site_ok);
let binders = get_binders()?;

for b in binders {
    let dir_files = 
    bro::get_files(&b.data_url)?;
        for df in dir_files {
            let fj = 
            FilesGen::new(df,
                b.site_url);
            let f = fj.get_file()?;
            println!("{:?}",f);
        }
}
//println!("{:#?}",binders);
Ok(true)
// let files_ok = gen_files();
// todo!();
}
/*
Step : 1 get folder struct of data folder
step : 2 convert it into site folder urls
step : 3 place both in same obj
step : 4 loop picking from one and droppping into the other
*/
fn get_binders()->Result<Vec<Binder>,Error>{
let data_struct = bro::tasks::dir_structure_to_string::run("data")?;
let site_struct = bro::tasks::dir_structure_to_string::run("site")?;
let mut binders:Vec<Binder> = Vec::new();
    for n in 0..data_struct.len(){
        let site_url = 
        site_struct[n].clone();
        let data_url = 
        data_struct[n].clone();
        let binder = 
        Binder::new(&data_url,&site_url);
        binders.push(binder);
    }
Ok(binders)
}

mod tests {
    use super::*;
#[test]    
fn one(){
 let b = gen().unwrap();   
}    
}