// THIS IS THE BEST MODULE THAT I HAVE WRITTEN IN MY LIFE 22-dec-2021
/// A nav is specific for each Dir, one nav for one dir will be used for all its files including index.
/// Every dir nav has path upto that dir and then different segments each corresponding to one sub_dir
// Write pure code or dont waste time.
pub fn nav(dir_path:&String,sub_dirs:&Vec<String>)->String{

let mut nav = String::from("<header id='header'><nav class='links' style='--items: 1;'><a href='/index.html'>Home</a>");

for sd in sub_dirs{
let link = 
format!("<a href='./{}/index.html'>{}</a>",sd,sd);    
    nav.push_str(&link);
}

nav.push_str(&"</nav></header>");
return nav   
}

// This is well formatted return string but this has spaces so can not be used for testing
pub fn fake()->String{
let static_data = r#"
<header id='header'>
    <nav class='links' style='--items: 1;'>
        <a href='./aa/bb/cc/index.html'>cc</a>
        <a href='./aa/bb/dd/index.html'>dd</a>
        <a href='./aa/bb/ee/index.html'>ee</a>
        <a href='./aa/bb/ff/index.html'>ff</a>
    </nav>
</header>"#;
    static_data.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
#[test]
fn excellent_test_no_confusion(){
let dir_path = String::from("site/aa/bb");    
let mut sub_dirs:Vec<String> = Vec::new();
sub_dirs.push("cc".to_string());
sub_dirs.push("dd".to_string());
sub_dirs.push("ee".to_string());
sub_dirs.push("ff".to_string());
let r = nav(&dir_path,&sub_dirs);
println!("{:?}",r);
 let static_data = r#"<header id='header'><nav class='links' style='--items: 1;'><a href='/index.html'>Home</a><a href='./cc/index.html'>cc</a><a href='./dd/index.html'>dd</a><a href='./ee/index.html'>ee</a><a href='./ff/index.html'>ff</a></nav></header>"#;
assert_eq!(r.as_str(),static_data);
}

//--mod tests
}