use std::fs::DirEntry;
use crate::*;
pub fn is_md(file:&DirEntry)->bool{
let ext = get_ext(&file);
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
