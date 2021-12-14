use brown::*; 
use brown::BrownError as Error;

pub fn main() {
  let clone = 
  clone_dir_structure("data","site");
  println!("{:?}",clone);
  
  //-->Step one delete old site folder
  //remove_dir_brute("site").unwrap();
  let mut dir_struct = 
  dir_structure_to_string("data").unwrap();
  //-- MY FIRST
  let dir_struct_ok = 
  dir_struct.iter_mut().
  map(|i|i.replace("./",""))
  .collect::<Vec<String>>();

  // println!("{:?}",dir_struct_ok); 
  //---------------------------------
  for dir in dir_struct_ok{
    let d = create_dir_brute(&dir.as_str())
    .unwrap();
    println!("{:?}",d);
      let files = get_files(&dir).unwrap();
      for file in files {
        
        let file_path = file.path();
        // println!("{:?}",file_path);
        
        let content = std::fs::
        read_to_string(&file_path).unwrap();
        
        let file_path_string = direntry_to_path(&file).unwrap();
            
        let dest = file_path_string.replacen("data", "site", 1);
          let dest_clean = dest.replace("./","");
          // println!("{:?}",dest_clean);
            
            // let dest_path = std::path::Path::
            //   new(&dest);
             
             // write_to_file(&dest, &content)
              //.unwrap();
        // let r = std::fs::write(&dest, &content);
      // let b = create_file(&dest_clean.as_str())
      // .unwrap();
      let p = std::path::Path::new(&dest_clean);
      // println!("{:?}",p);
  let b = 
  create_file_brute(dest_clean.as_str())
  .unwrap();
      // let r = write_to_file
      // (&dest_clean, &content).unwrap();
      println!("{:?}",b);
              
      }
  }
}//main

fn create_file(){
create_file_brute("site/abc.md");  
}