use crate::generator;


pub fn cli_gen(){
    let gen_result = generator::gen();
    match gen_result {
      Ok(_r)=>{
          println!("Markdown to Html generation completed");  
      },
      Err(e)=> println!("{:?}",&e),
    }
}