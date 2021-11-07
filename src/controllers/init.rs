use std::io::{Error};
use brown::Hdir;

pub fn init()->Result<bool,Error>{
    let hdir = Hdir::new()?;
    println!("initialization Hulk folder....");

    let folders_paths_list = vec!
    [ 
    "data", 
    "site" ,
    "site/images", 
    "hulkfolder" ,
    "hulkfolder/templates" ,
    "hulkfolder/config" ,
    ];

    for folder in folders_paths_list {
        let result = hdir.create_dir(folder);
        match result {
            Ok(f)=>{println!("folder created :: {}",folder)},
            Err(_e)=>{
                println!("failed to create folder :: {} , the folder may already exist",folder);
                continue;
            },
        }
    }
 println!("initialization completed....!!!!!!");
 Ok(true)
}//init ends
