# Create a File

```rust
fn create_file(&self,file_name:&str)->File{
    // let file_name = "test_doc.txt";
    let my_file = File::create(file_name).expect("creation failed");
    my_file
  }
```  