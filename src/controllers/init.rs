use super::super::brown;

pub fn init(){
    println!("initialization Hulk folder....");

    let folders_paths_list = vec!
    [ 
    "./data", 
    "./site" ,
    // "./site/assets", 
    // "./site/css", 
    "./site/images", 
    // "./hulkfolder" ,
    // "./hulkfolder/themes" ,
    // "./hulkfolder/themes/default" ,
    // "./hulkfolder/themes/light" ,
    // "./hulkfolder/templates" ,
    ];

    //let folder_exists = brown::path_exists(value)
    brown::check_n_create_folders(folders_paths_list);
 println!("initialization completed....!!!!!!");
}//init ends
