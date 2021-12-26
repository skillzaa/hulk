
use yansi::Paint;

const gap:&str = "                           "; 

pub fn print_hulk_top_bar(){
    println! ("{}",gap);
    println! ("{}",gap);
    let bar_top = "================== HULK =======================";
    println!("{}",Paint::green(bar_top));
    println! ("{}",gap);
    println! ("{}",gap);
}
pub fn print_hulk_bot_bar(){
    println! ("{}",gap);
    let bar_bot = "================== HULK ENDS ======================= "; 
    println!("{}",Paint::green(bar_bot));
    println! ("{}",gap);
  
}