use brown::*;
use brown::BrownError as Error;
pub struct Generator;

impl Generator {
    pub fn new()->Self{
        Generator
    }
    pub fn default()->Self{
        Generator
    }
    pub fn run()->Result<bool,Error>{
        //Step one delete old site folder
        //PROBLEM THIS RETURN ERROR IF SIR NOT FOUND
        //--CHECK IN BROWN
        let _ = remove_dir_brute("site");
        let dir_struct = 
        clone_dir_structure("data", "site")?;
        println!("{:?}",dir_struct);
        Ok(true)
    }

}