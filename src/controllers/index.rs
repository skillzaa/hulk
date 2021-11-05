use super::super::brown;
use crate::core::{get_default_footer,get_default_header, get_default_nav };

pub fn index()->Option<bool>{
    let mut html = String::new();
    html.push_str(get_default_header());
    html.push_str(get_default_nav());
    html.push_str("<h1>Home Page</h1><br/><hr/>");
    
    let ans = brown::get_files_by_ext("./site", "html").expect("failed to get files by extention");
    
      for direntry in ans{
        
       // let path = brown::direntry_to_path_buf(direntry)?;
        
        let file_name = direntry.file_name();
        let file_name_str = file_name.to_str().map(|s| s.to_string()).unwrap(); 
        //------------ templating ---------------------------
        html.push_str("<tr>");
       
        let name = format!("<td><a href=\"./{}\">{}</a> </td>",file_name_str,file_name_str);
       html.push_str(&name);
       
       let link = format!("<td><a href=\"./{}\">{}</a> </td>",file_name_str,file_name_str);
       html.push_str(&link);
       
       html.push_str("</tr>");
       
      }
      html.push_str(get_default_footer());
  
  //======================================
  brown::create_file("./site/index.html");
  
  brown::write_to_file("./site/index.html",&html ).unwrap();
  Some(true)
  }