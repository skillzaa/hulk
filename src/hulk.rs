mod brown;
mod init;
mod index;
use brown::Brown;

struct B{}
impl Brown for B{}
impl B {
  fn new()->B{
    B{}
  }
}

pub fn init(){
 init::init(); 
 
}

pub fn gen(){
  let b = B::new();
  b.move_files();
  index();
}
pub fn help(){
  println!("Help");
  println!("====");
  println!("Please use following commands");
  println!("To create a new project : \"cargo run init\" ");
  println!("To generate a static site : \"cargo run gen\" ");

}
pub fn index(){
  // index::index();
  let b = B::new();
  
  let mut html = String::new();
  html.push_str(b.get_default_header());
  html.push_str(b.get_default_nav());
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
    html.push_str(b.get_default_footer());

//======================================
b.create_file("./site/index.html");

b.write_to_file("./site/index.html",&html ).unwrap();

}