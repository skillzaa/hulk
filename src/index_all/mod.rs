use crate::bro;
use crate::app_consts;
use crate::bro::BrownError as Error;
use crate::indexer::Indexer;

pub fn run()->Result<bool,Error>{
let site_struct = 
bro::dir_structure_to_string(app_consts::HULK_SITE_DIR)?;
let clean:Vec<String> = site_struct.iter()
.map(|i|i.replace("./","")).collect(); 
//--------------------------------------------
//--------------------------------------------
    for d in clean {
        let i = Indexer::new(d);
        match i {
        Some(ii)=>{
         let _ = ii.run();
        },
        None=>{continue;},
        }        
    }
//--------------------------------------------
//--------------------------------------------
Ok(true)
}

mod tests {
use super::*;
#[test]
fn basic(){
//    run();
}
}