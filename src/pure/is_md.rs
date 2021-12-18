use crate::brown as bro;
use std::fs::DirEntry;

pub fn is_md(file:&DirEntry)->bool{
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
    