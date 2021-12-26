use crate::init;


pub fn cli_init(){
    let init_result = init::init();
    match init_result {
      Ok(_r)=>{
          println!("Project initialized....");
      },
      Err(e)=> println!("{:?}",&e),
    } 
}