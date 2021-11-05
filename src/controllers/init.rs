use brown::Brown;

struct Bro { }
impl Bro {
    fn new()->Bro{
        Bro {}
    }
}
impl Brown for Bro {}

pub fn init(){
    let bro = Bro::new();
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

    bro.check_n_create_folders(folders_paths_list);
 println!("initialization completed....!!!!!!");
}//init ends
