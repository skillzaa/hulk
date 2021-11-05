
# Vectors in Rust part-2
Lets look at the following code. This will not compile.
```rust
fn main() {
   let mut v:Vec<i32> = vec![1,2,3,4,5];
   //==== Immutable borrow 
   let first = &v[0];
   //==== Mutable borrow
   v.push(6);
   println!("first{}",first);
}
```    
> We can not have a mutable borrow and a non mutable borrow in the same scope.Adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.
---
## Iterating over the Values in a Vector
We can do it like:
```rust
 let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
```
OR
```rust
 let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
  }
```        
## Using an Enum to Store Multiple Types
```rust
  enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```    