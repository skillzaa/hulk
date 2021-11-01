use std::fs;
use std::fs::{File,DirEntry};
use comrak::{markdown_to_html, ComrakOptions};
use std::path::Path;
use std::io::Write;
use std::ffi::{OsStr, OsString};
pub trait Brown {
  fn create_file(&self,file_name:&str)->File{
    // let file_name = "test_doc.txt";
    let my_file = File::create(file_name).expect("creation failed");
    my_file
  } 
  fn create_dir(&self,dir_name:&str)->std::io::Result<()> {
    let path = String::from("./") + &dir_name;
  
    let d = fs::create_dir(path)?;
    Ok(d)
  }
  fn create_dir_all(&self,dir_name:String)->std::io::Result<()> {
    //prepend the ./
    let path = String::from("./") + &dir_name;
    println!("path {}",path);
    let d = fs::create_dir_all(path)?;
    Ok(d)
  } 
  fn read_dir (&self,dir_name:&str)->Vec<DirEntry>{
    let mut v:Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(dir_name).unwrap() {
      let entry = entry.unwrap();
      // let path = entry.path();  
              v.push(entry);
    }
    v
  }
  fn get_dir_from_dir (&self,dir_name:&str)->Vec<DirEntry>{
    let mut v:Vec<DirEntry> = Vec::new();
        for entry in fs::read_dir(dir_name).unwrap() {
          let entry = entry.unwrap();
          let path = entry.path();
                if path.is_dir(){
                  v.push(entry);
                }
        }
    v
  }
  fn get_files_from_dir (&self,dir_name:&str)->Vec<DirEntry>{
    let mut v:Vec<DirEntry> = Vec::new();
        for entry in fs::read_dir(dir_name).unwrap() {
          let entry = entry.unwrap();
          let path = entry.path();
                if path.is_file(){
                  v.push(entry);
                }
        }
    v
  }
  fn move_files(&self){
    let header = self.get_default_header();
    let footer = self.get_default_footer();
  
    // let footer = fs::read_to_string("./hulkfolder/themes/default/footer.html").unwrap();
  
    let all_files = &self.get_files_from_dir("./data");
   //===== the loop ===== 
    for entry in all_files.iter(){
    
      let path = entry.path();
      
      let md = fs::read_to_string(&path).unwrap();
    
      let html = markdown_to_html(&md.to_string(), &ComrakOptions::default());
  
      let mut page = String::from(header);
          page.push_str(self.get_default_nav());
          page.push_str(&html);
          page.push_str(&footer);
      //==============================
        //let path_str = path.to_str().map(|s| s.to_string()).unwrap();
        let file_name = path.file_name().unwrap();
        let file_name_str = file_name.to_str().map(|s| s.to_string()).unwrap();
        let len = file_name_str.len();
        let ext = &file_name_str[len-3..];
        let file_wo_ext = &file_name_str[0..len-3];
        
        //println!("file_name_str::{:?}",file_name_str);
        
        if let ".md" = &*ext {
        //    println!("ext::{:?}",ext);
            let mut s = String::from("./site/");
            s.push_str(file_wo_ext);
            s.push_str(".html");
            // println!("S::{:?}",s);
  
            let write_path = Path::new(&s);
            fs::write(write_path, page).unwrap();
          }
  
      //==============================
    }
    let write_path_css = Path::new("./site/main.css");
    self.create_file(write_path_css.to_str().unwrap());
    let css = self.get_dark_css();
            fs::write(write_path_css, css).unwrap();
      println!("All files moved......");
  
    
  }
  fn check_n_create_folders (&self,folders_paths_list:Vec<&str>)->Option<bool>{
    
    for item in &folders_paths_list {
        let data_folder_path = Path::new(item);
        let data_folder_exists = data_folder_path.exists();
        if !data_folder_exists {
            self.create_dir(item).unwrap();
            println!("folder created:: {}",item.to_string());
        }else {
            println!("folder aleady exists:: {}",item.to_string());            
        }
    }
    Some(true)
}
  fn write_to_file(&self,file_name:&str,content:&str) -> std::io::Result<()> {
  let mut f = std::fs::OpenOptions::new().write(true).open(file_name)?;
  f.write(content.as_bytes())?;
  
  f.flush()?;
  Ok(())
  }
  fn get_files_by_ext (&self,dir_name:&str,ext:&str)->Vec<DirEntry>{
    let mut v:Vec<DirEntry> = Vec::new();
        for entry in fs::read_dir(dir_name).unwrap() {
          let entry = entry.unwrap();
          let path = entry.path();
                if path.is_file(){
                  let e = path.extension().unwrap();
                  let e_str = e.to_str().unwrap();
                  if e_str == ext                    {
                    v.push(entry)
                  }
                }
        }
    v
  }
  fn get_default_header(&self)->&'static str{
    let a = r#"
    <!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Hulk</title>    
<link rel="stylesheet" href="./main.css">
</head>
<body>
    "#;
    a
  }
  fn get_default_footer(&self)->&'static str{
    let a = r#"
    </body>
</html>
    "#;
    a
  }
  fn get_default_nav(&self)->&'static str{
    let a = r#"
    <header id="header">
	<nav class="links" style="--items: 1;">
		<a href="./index.html">Home</a>
		<span class="line"></span>
	</nav>
</header>
    "#;
    a
  }
  fn get_header(&self)-> & 'static str {
    let header = r#"
    <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hulk</title>
    
    <link rel="stylesheet" href="./main.css">
</head>
<body>
    
    "#;
    header
}
  fn get_footer(&self)->& 'static str {
let footer = r#"
</body>
</html>
"#;
footer
}
  fn get_dark_css(&self)->& 'static str{
  let dark = r#"
  @charset "UTF-8";
  body {
    font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
    line-height: 1.4;
    max-width: 800px;
    margin: 20px auto;
    padding: 0 10px;
    color: #dbdbdb;
    background: #202b38;
    text-rendering: optimizeLegibility;
  }

  button, input, textarea {
    transition: background-color 0.1s linear, border-color 0.1s linear, color 0.1s linear, box-shadow 0.1s linear, transform 0.1s ease;
  }

  h1 {
    font-size: 2.2em;
    margin-top: 0;
  }

  h1,
  h2,
  h3,
  h4,
  h5,
  h6 {
    margin-bottom: 12px;
  }

  h1,
  h2,
  h3,
  h4,
  h5,
  h6,
  strong {
    color: #ffffff;
  }

  h1,
  h2,
  h3,
  h4,
  h5,
  h6,
  b,
  strong,
  th {
    font-weight: 600;
  }

  blockquote {
    border-left: 4px solid #0096bfab;
    margin: 1.5em 0em;
    padding: 0.5em 1em;
    font-style: italic;
  }

  blockquote > footer {
    margin-top: 10px;
    font-style: normal;
  }

  blockquote cite {
    font-style: normal;
  }

  address {
    font-style: normal;
  }

  a[href^='mailto']::before {
    content: '📧 ';
  }

  a[href^='tel']::before {
    content: '📞 ';
  }

  a[href^='sms']::before {
    content: '💬 ';
  }

  button,
  input[type='submit'],
  input[type='button'],
  input[type='checkbox'] {
    cursor: pointer;
  }

  input:not([type='checkbox']):not([type='radio']),
  select {
    display: block;
  }

  input, select, button, textarea {
    color: #ffffff;
    background-color: #161f27;
    font-family: inherit;
    font-size: inherit;
    margin-right: 6px;
    margin-bottom: 6px;
    padding: 10px;
    border: none;
    border-radius: 6px;
    outline: none;
  }

  input:not([type='checkbox']):not([type='radio']),
  select, button, textarea {
    -webkit-appearance: none;
  }

  textarea {
    margin-right: 0;
    width: 100%;
    box-sizing: border-box;
    resize: vertical;
  }

  button, input[type='submit'], input[type='button'] {
    padding-right: 30px;
    padding-left: 30px;
  }

  button:hover,
  input[type='submit']:hover,
  input[type='button']:hover {
    background: #324759;
  }

  input:focus,
  select:focus,
  button:focus,
  textarea:focus {
    box-shadow: 0 0 0 2px #0096bfab;
  }

  input[type='checkbox']:active,
  input[type='radio']:active,
  input[type='submit']:active,
  input[type='button']:active,
  button:active {
    transform: translateY(2px);
  }

  input:disabled,
  select:disabled,
  button:disabled,
  textarea:disabled {
    cursor: not-allowed;
    opacity: .5;
  }

  ::-webkit-input-placeholder {
    color: #a9a9a9;
  }

  :-ms-input-placeholder {
    color: #a9a9a9;
  }

  ::-ms-input-placeholder {
    color: #a9a9a9;
  }

  ::placeholder {
    color: #a9a9a9;
  }

  a {
    text-decoration: none;
    color: #41adff;
  }

  a:hover {
    text-decoration: underline;
  }

  code, kbd {
    background: #161f27;
    color: #ffbe85;
    padding: 5px;
    border-radius: 6px;
  }

  pre > code {
    padding: 10px;
    display: block;
    overflow-x: auto;
  }

  img {
    max-width: 100%;
  }

  hr {
    border: none;
    border-top: 1px solid #dbdbdb;
  }

  table {
    border-collapse: collapse;
    margin-bottom: 10px;
    width: 100%;
  }

  td, th {
    padding: 6px;
    text-align: left;
  }

  th {
    border-bottom: 1px solid #dbdbdb;
  }

  tbody tr:nth-child(even) {
    background-color: #161f27;
  }

  ::-webkit-scrollbar {
    height: 10px;
    width: 10px;
  }

  ::-webkit-scrollbar-track {
    background: #161f27;
    border-radius: 6px;
  }

  ::-webkit-scrollbar-thumb {
    background: #324759;
    border-radius: 6px;
  }

  ::-webkit-scrollbar-thumb:hover {
    background: #415c73;
  }



  "#;
  dark
  }

}//final bracket
 

struct B{}
impl Brown for B{}

impl B {
  fn new()->B{
    B {}
  }
}
