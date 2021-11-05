use std::{fs};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs::{DirEntry, File, ReadDir};
use std::io::{Error, ErrorKind};

  pub fn create_file(file_name:&str)->Result<File,Error>{
    let my_file = File::create(file_name);
      my_file
   } 
  pub fn delete_file(file_name:&str)->Result<bool,Error>{
    let path = std::path::Path::new(file_name);
    match path.exists() {
      false => {
        let error = Error::new(ErrorKind::NotFound, "the path could not be found");
        return Err(error);
        },
      true => {
        let result  = fs::remove_file(&path);
        match result {
          Ok(()) => return Ok(true),
          Err(e) => return Err(e),
        }
      }  
    }
  } 
  pub fn create_dir(dir_name:&str)->Result<bool,Error> {
    let complete = String::from("./") + &dir_name;
    let path = std::path::Path::new(&complete);
    let d = fs::create_dir(path);
    match d {
      Ok(()) => return Ok(true),
      Err(e) => Err(e),
    }
  }
  pub fn create_dir_all(dir_name:String)->Result<(),Error> {
    let full_path = String::from("./") + &dir_name;
    let path = std::path::Path::new(&full_path);
    let d = fs::create_dir_all(path);
    d
  }
  pub fn remove_dir(dir_name:&str)->Result<(),Error> {
    let complete = String::from("./") + &dir_name;
    let path = std::path::Path::new(&complete);
    let d = fs::remove_dir(path);
    d
  }
  pub fn remove_dir_all(dir_name:&str)->Result<(),Error> {
    let complete = String::from("./") + &dir_name;
    let path = std::path::Path::new(&complete);
    let d = fs::remove_dir_all(path);
    d
  } 
  pub fn read_dir (dir_name:&str)->Result<ReadDir,Error>{
    let dir_entry = fs::read_dir(dir_name).expect("failed to read directory");
    Ok(dir_entry)
  }
  pub fn get_dir_from_dir (dir_name:&str)->Vec<DirEntry>{
    let all = fs::read_dir(dir_name).unwrap();
      let v = 
    all.map(|x|x.unwrap())
    .filter(|x| (&x.path()).is_dir()).collect();
    v
  }
  pub fn get_files_from_dir (dir_name:&str)->Vec<DirEntry>{
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
  // pub fn get_files_by_ext (dir_name:&str,ext:&str)
  // ->Result<Vec<DirEntry>,Error>{
  //   let all_files = fs::read_dir(dir_name).expect("failed to open directory");
  //   let mut v:Vec<DirEntry> = Vec::new();
  //   for entry in all_files {
  //       let entry = entry.expect("failed to open directory entry");
  //         let path_buf = entry.path();
  //           let pth_ext = path_buf.as_path().extension().unwrap();
  //           let pth_ext_str = pth_ext.to_str().unwrap();  
  //         // let buf_ext = path_buf.extension().expect("get_files_by_ext").to_str().expect("get_files_by_ext");
  //                 if &*pth_ext_str == &*ext{
  //                       v.push(entry);
  //                 }
  //   }
  //   Ok(v)
  // }
  pub fn get_files_by_ext(dir_name:&str,ext:&str)->Option<Vec<DirEntry>>{
    let mut v = Vec::new();
    let all_files = fs::read_dir(dir_name).expect("failed to open directory");
      for direntry in all_files {
        match direntry.unwrap() {
          entry=>{
                    let p = entry.path().as_path();
                    let e  = path_ext(p);
                        match e {
                          Some(e)=>{
                            if e == &*ext{
                                v.push(entry);
                            }
                          },
                          None=> continue,
                        } 
          },
          _=> continue,
        }  
      }  
    Some(v)
  }
  pub fn path_exists( value:&str)->bool{
    let path = std::path::Path::new(value);
    let tf = path.exists();
    tf
  }
  pub fn check_n_create_folders (folders_paths_list:Vec<&str>)->Option<bool>{
    for item in folders_paths_list {
        let data_folder_path = Path::new(item);
        let data_folder_exists = data_folder_path.exists();
        if !data_folder_exists {
            create_dir(item).unwrap();
            println!("folder created:: {}",item.to_string());
        }else {
            println!("folder aleady exists:: {}",item.to_string());            
        }
    }
    Some(true)
  }
  pub fn write_to_file(file_name:&str,content:&str) -> std::io::Result<()> {
  let mut f = std::fs::OpenOptions::new().write(true).open(file_name)?;
  f.write(content.as_bytes())?;
  f.flush()?;
  Ok(())
  }
  
  
  pub fn unwrap_direntry(direntry:Result<DirEntry,Error>)->Option<DirEntry>{
    let unwrapped = direntry;
    match unwrapped {
      Ok(direntry_final)=>{return Some(direntry_final)},
      Err(e) => return None,
    }
  }
  
  pub fn path_ext(path:&Path)->Option<String> {
    let ext_os_str = path.extension();
    match ext_os_str {
      Some(some)=>{
        let ext_str = some.to_str();
                  match ext_str {
                    Some(e_s)=> {
                      let ret = String::from(e_s);
                      return Some(ret);
                    },
                    None=> return None,
                  }
      },
      None=> return None,
    }
  } 
  pub fn direntry_to_path_buf(direntry:Result<DirEntry,Error>)->Option<&'static PathBuf>{
    let direntry = unwrap_direntry(direntry)?;
    let file_path_buf = direntry.path();
    Some(&file_path_buf)
  }
 
