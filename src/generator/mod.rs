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
        let old_site = remove_dir_brute("site")?;
        
        Ok(true)
    }

}