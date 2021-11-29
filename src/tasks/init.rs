use std::io::{Error};
use brown as bro;

pub fn init()->Result<bool,Error>{
    println!("initialization Hulk folder....");
    let folders_paths_list = vec!
    [ 
        "site" ,
        "site/images", 
        "hulkfolder" ,
        "hulkfolder/templates" ,
        "hulkfolder/config" ,
        "data", 
    ];
//====================== ONE LINE MAGIC
bro::tasks::create_dir_structure::
run(&folders_paths_list)?;
//======================================== 
println!("initialization completed....!!!!!!");
 Ok(true)
}//init ends
