# Thread Sleep
This is Test function using thread sleep. We can put a thread to sleep in test functions or in other code.

```rust
use playhead::PlayHead;
use std::{thread, time};

#[test]
fn seq(){
    let mut ph = PlayHead::new();
    ph.play();
    let time_one = ph.time();

    let delay = time::Duration::from_millis(2000);
    thread::sleep(delay);
    
    let time_two = ph.time();
    //==========================
    ph.pause();
    thread::sleep(delay);
    let time_three = ph.time();

    assert!(time_two > time_one);
    assert_eq!(time_two, time_three);
    // println!("Time after sleep=====> {:?}",time_two - time_one);
}//fn
```