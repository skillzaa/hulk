use super::super::brown;

pub fn init(){
    println!("initialization Hulk folder....");

    let folders_paths_list = vec!
    [ 
    "./data", 
    "./site" ,
    "./site/images", 
    "./hulkfolder" ,
    "./hulkfolder/shortcodes" ,
    ];

    brown::check_n_create_folders(folders_paths_list);
 println!("initialization completed....!!!!!!");
}//init ends
