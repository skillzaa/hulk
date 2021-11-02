use crate::core::HulkFs;

pub fn init(){
    let b = HulkFs::new();
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

    b.check_n_create_folders(folders_paths_list);

//---- now create files
    //====header
//  b.create_file("./hulkfolder/themes/default/header.html");
//  let header_content = get_header();
//  b.write_to_file("./hulkfolder/themes/default/header.html",header_content ).unwrap();

    //====footer 
//  b.create_file("./hulkfolder/themes/default/footer.html");
//  let footer_content = get_footer();
//  b.write_to_file("./hulkfolder/themes/default/footer.html",footer_content ).unwrap();

 
    //====dark default water.css file 
//  b.create_file("./hulkfolder/themes/default/main.css");
//  let css_content = get_dark_css();
//  b.write_to_file("./hulkfolder/themes/default/main.css",css_content ).unwrap();
//---write the same css file to site folder as well --change later
// b.write_to_file("./site/main.css",css_content ).unwrap();
 println!("initialization completed....!!!!!!");
}//init ends
