use fltk::{prelude::*, enums::*, *};

use fltk_flow::Flow;

pub fn card()->Flow{

    let mut flow = Flow::default_fill();
// -----------------------------------------   
    //============== level zero
    let hdg = heading();
    let hdg_sep = sep();
    //============== level one
    let mut change_data_btn = change_data_btn("change data");
    let inp = input::Input::default().with_size(150, 30);
    let mut explain = get_label(0, 0, 16,
        "You can changed the name of your <Data> folder.");
    
    //--this is the seperator
    
    let mut sep = sep();
    flow.rule(&hdg,"^/<");
    flow.rule(&hdg_sep,"=<^");

    flow.rule(&change_data_btn, "^<");
    flow.rule(&inp, "^<=>");
    flow.rule(&explain, "^<=>");
    flow.rule(&sep, "=<^");
    
    //============== level two

    
// -----------------------------------------   
flow.end();
// -----------------------------------------   
change_data_btn.set_callback(move|_b|{
    let msg = String::from(inp.value());
    let response = event_handler(msg);
    dialog::message(200, 200, response.as_str());

});
flow
}
fn heading()->frame::Frame{
let mut f = frame::Frame::
new(0,0,10,10,"Hulk Setting");
// f.set_label("testing");
f.set_label_size(32);
f
}
fn get_label(w:i32,h:i32,size:i32,title:&'static str)->frame::Frame{
let mut f = frame::Frame::
new(0,0,w,h,title);
// f.set_label("testing");
f.set_label_size(size);
f
}
fn change_data_btn(title:&str)->button::Button{
    let mut b = button::Button::default().with_size(100, 30).with_label(title);
    b  
}
fn sep()->frame::Frame{
let mut f = frame::Frame::default().with_size(10, 1);
f.set_color(Color::Black);
f.set_frame(FrameType::FlatBox);
f
}


fn event_handler(msg:String)->String{
    println!("{:?}",msg);  
    let len = msg.len();
    if len < 5 {
        "all quit on western front".to_string()
    }else {
        "finally some action".to_string()
    }
}