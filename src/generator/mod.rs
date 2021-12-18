mod md;
mod non_md;
use brown::BrownError as Error;
use crate::brown as bro;
use std::fs::DirEntry;
use crate::nav;
use md::md_file;
use non_md::non_md_file;
/*
 Steps : 
  later 1 : gen css file in site folder.
  2 : get the dir structure from data.
  -- there may be a lot of dirs but the operation is flat since all the DIR urls are in one vec. but for each dur there aremany file thus we have 2 loops
    --main loop over folders
    -- inner loop over each file in folder
    2-a: get file header
    2-b: get file content
    2-c: markdown to html the content
    2-d: get the nav bar - using site folder urls
    2-e: create the file
    2-f: place file in site folder   
*/
pub struct Generator {
  data_dir_name:String,
  site_dir_name:String,
}
impl Generator{
  pub fn new(data_dir_name:String,
    site_dir_name:String)->Self{
      Generator{
      data_dir_name,
      site_dir_name,
    }
  }
  pub fn data_dir_struct(&self)->Result<Vec<String>,Error>{
  let mut dir_struct = 
  bro::dir_structure_to_string(&self.data_dir_name)?;
  //-- MY FIRST
  let dir_struct_clean = 
  dir_struct.iter_mut().
  map(|i|i.replace("./",""))
  .collect::<Vec<String>>();

  Ok(dir_struct_clean)
  }
  pub fn site_dir_struct(&self)->Result<Vec<String>,Error>{
    let data_struct = self.data_dir_struct()?;
    let output:Vec<String> = data_struct.into_iter()
    .map(|i|i.replacen(
      &self.data_dir_name, 
      &self.site_dir_name,
      1))
      .collect::<Vec<String>>();
    Ok(output)
  }
  // This is just both the loops inner and outer
  pub fn run(&self)->Result<String,Error>{
    let dir_struct = self.data_dir_struct()?;
    //--Outer loop around dirs
    for dir in dir_struct{
      // let navbar = get_nav(); 
      let navbar = nav::fake(); 
      //-----get files
      let files = bro::get_dirs(&dir)?;
      //----------------------- 
        //--Inner loop around files
        for file in files {
            match is_md(&file) {
            true=>{
                // let final_file = md_file(&file,&navbar);
                let content = get_content(&file);
                let dest = get_dest_clean(&file);
              create_n_write_file(dest, content);
            },
            false=>{
              // let content = get_content(&file);
              let content = "bla bla bla".to_string();
              let dest = get_dest_clean(&file);
              let _r = create_n_write_file(dest, content);
              // return Ok(final_file);
            },
          }
      }
    }
      Ok("ok".to_string())
    }

}//impl


fn create_n_write_file(dest:String,content:String)->bool{
  let _b = 
  bro::create_file_brute(dest.as_str())
  .unwrap();
    let _r = bro::write_to_file
    (&dest, &content).unwrap();
  true
}
fn get_dest_clean(file:&DirEntry)->String{
  let file_path_string = 
  bro::direntry_to_path(&file).unwrap();        
  let dest = file_path_string.replacen("data_test", "site", 1);
  let d = dest.replace("./","");
          d
}
fn is_md(file:&DirEntry)->bool{
let ext = bro::get_ext(&file);
        match ext {
        Ok(item)=>{
            if item == "md"{
            true
            }else {
            false
            }
        },
        Err(_e)=>{false},
        }
}
fn get_content(file:&DirEntry)->String{
  let file_path = file.path();
  std::fs::
    read_to_string(&file_path).unwrap()
}
  mod tests {
    use super::*;
  #[test]
  fn data_dir_struct_test(){
  let g = Generator
  ::new("data_test".to_string(),
"site".to_string());   
  let p = g.data_dir_struct().unwrap();
  let resp = [
  "data_test", 
  "data_test/b",
  "data_test/a",
  "data_test/a/a1",
  "data_test/a/a1/a2",
  "data_test/a/a1/a2/a3-b",
  "data_test/a/a1/a2/a3-a"
  ];
  let stng = resp.into_iter()
  .map(|i|i.to_string()).collect::<Vec<String>>();
  assert_eq!(stng,p);
  // println!("{:?}",p);  
  }
  #[test]
  fn site_dir_struct_test(){
    let g = Generator
    ::new("data_test".to_string(),
  "site".to_string());   
    let p = g.site_dir_struct().unwrap();
    let resp = [
    "site", 
    "site/b",
    "site/a",
    "site/a/a1",
    "site/a/a1/a2",
    "site/a/a1/a2/a3-b",
    "site/a/a1/a2/a3-a"
    ];
    let stng = resp.into_iter()
    .map(|i|i.to_string()).collect::<Vec<String>>();
    assert_eq!(stng,p);
    println!("{:?}",p);  
  }

  #[test]
  fn run_test(){
    let g = Generator
    ::new("data_test".to_string(),
  "site".to_string());   
  let res = g.run();
  }
  //----test mod ends
}