use brown::*; 
use brown::BrownError as Error;

pub fn main() {
 let run_result = run().unwrap();
 println!("{:?}",run_result); 
}//main

fn run()->Result<bool,Error>{
// Step 01: clone_data_to_site
clone_data_to_site()?;
// Step 02: get_dir_struct_clean
let dir_struct_clean = 
get_dir_struct_clean()?;
//---------------------------------
// Step 03: Loop for each sub-dir

  for dir in dir_struct_clean{
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
      let r = write_to_file
      (&dest_clean, &content).unwrap();
      println!("{:?}",r);
              
      }
  }
  Ok(true)
}//run

fn clone_data_to_site()->Result<Vec<String>,Error>{
clone_dir_structure("data","site")  
}

fn get_dir_struct_clean()->Result<Vec<String>,Error>{
  let mut dir_struct = 
  dir_structure_to_string("data")?;
  //-- MY FIRST
  let dir_struct_clean = 
  dir_struct.iter_mut().
  map(|i|i.replace("./",""))
  .collect::<Vec<String>>();

Ok(dir_struct_clean)
}