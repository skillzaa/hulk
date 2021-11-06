mod static_assets;
pub use static_assets::*;

use std::{fs};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs::{DirEntry, File, ReadDir};
use std::io::{Error, ErrorKind};

// pub fn get_dir_from_dir (dir_name:&str)->Vec<DirEntry>{
//     let all = fs::read_dir(dir_name).unwrap();
//       let v = 
//     all.map(|x|x.unwrap())
//     .filter(|x| (&x.path()).is_dir()).collect();
//     v
// }
pub fn get_files_from_dir(dir_name:&str)->Option<Vec<DirEntry>>{
    let all = fs::read_dir(dir_name);
    match all {
       Ok(some)=>{
        let v = 
        some.map(|x|x.unwrap())
        .filter(|x| (&x.path()).is_file()).collect();
        Some(v)
       },
       Err(e)=> return None, 
    }  
}

pub fn get_files_by_ext(dir_name:&str,ext:&str)->Option<Vec<DirEntry>>{
    let files = get_files_from_dir(dir_name)?;
    let mut f = Vec::new();
    for file in files{
        let tf = compare_ext(&file, ext)?;
        if tf {f.push(file)}
    }
    Some(f)
}
pub fn compare_ext (dir_entry:&DirEntry,ext:&str)->Option<bool>{
    let path_buf = dir_entry.path();
    let path = path_buf.as_path();
    let ext_os_str = path.extension()?;
    let ext_str = ext_os_str.to_str()?;
    if &*ext == ext_str  {
        return Some(true);
    } else {
        return None;
    }
}
