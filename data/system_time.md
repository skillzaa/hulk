# System Time / Time Stamp
```rust
use std::time::SystemTime;

fn main() {
let time_now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
println!("Finally the time stamp::{:?} ",time_now);

}
```