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