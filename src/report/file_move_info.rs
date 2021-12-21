#[derive(Debug)]
pub struct FileMoveInfo {
    pub data_path:String,
    pub site_path:String,
    pub file_name:String,
    pub file_ext:String,
    pub nav:String,
    pub content:String,
    pub is_md:bool,
}
// the is_md should not be false in default
// i am using this with default which when not filled give wrong mesages

impl FileMoveInfo{
    pub fn new(
        data_path:String,
        site_path:String,
        file_name:String,
        file_ext:String,
        nav:String,
        content:String,
        is_md:bool,
    )->Self{
        FileMoveInfo {
            data_path,
            site_path,
            file_name,
            file_ext,
            nav,
            content,
            is_md,
        }    
    }
    
}