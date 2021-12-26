use crate::indexer;


pub fn cli_index(){
    let r = indexer::run();
            match r {
              Ok(_r)=>{
              println!("index files generated successfully");
              },
              Err(e)=> println!("{:?}",&e),
            }
}