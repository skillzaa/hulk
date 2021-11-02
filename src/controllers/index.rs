use crate::core::HulkFs;
use crate::core::{get_default_footer,get_default_header, get_default_nav };

pub fn index(){
    // index::index();
    let b = HulkFs::new();
    
    let mut html = String::new();
    html.push_str(get_default_header());
    html.push_str(get_default_nav());
    html.push_str("<h1>Home Page</h1><br/><hr/>");
    
    let ans = b.get_files_by_ext("./site", "html");
      for entry in ans{
        let path = entry.path();
        let path_str = path.as_path().to_str().map(|s| s.to_string()).unwrap(); 
        let file_name = path.file_name().unwrap();
        let file_name_str = file_name.to_str().map(|s| s.to_string()).unwrap(); 
        //------------ templating ---------------------------
       let ht = format!("<a href=\"./{}\">{}</a> <br/>",file_name_str,file_name_str);
        //--!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        html.push_str(&ht);
      }
      html.push_str(get_default_footer());
  
  //======================================
  b.create_file("./site/index.html");
  
  b.write_to_file("./site/index.html",&html ).unwrap();
  
  }