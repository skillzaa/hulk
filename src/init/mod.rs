//use std::io::{Error};
use brown as bro;
use brown::BrownError as Error;
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
bro::create_dir_structure(&folders_paths_list)?;
//======================================== 
println!("initialization completed....!!!!!!");
 Ok(true)
}//init ends
