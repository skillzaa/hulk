use super::super::brown;
use crate::core::{get_default_footer,get_default_header, get_default_nav };

pub fn index(){
    let mut html = String::new();
    html.push_str(get_default_header());
    html.push_str(get_default_nav());
    html.push_str("<h1>Home Page</h1><br/><hr/>");
    
    let ans = brown::get_files_by_ext("./site", "html").expect("failed to get files by extention");
    
      for entry in ans{
        
        let path = entry.path();
        
        let file_name = path.file_name().unwrap();
        let file_name_str = file_name.to_str().map(|s| s.to_string()).unwrap(); 
        //------------ templating ---------------------------
       let ht = format!("<a href=\"./{}\">{}</a> <br/>",file_name_str,file_name_str);
        //--!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        html.push_str(&ht);
      }
      html.push_str(get_default_footer());
  
  //======================================
  brown::create_file("./site/index.html");
  
  brown::write_to_file("./site/index.html",&html ).unwrap();
  
  }