# Traits in Rust

To understand traits keep in mind that the concept of **trait** is many things in Rust. We will take it slowly and look at all its uses.

### What is a Trait (First Basic definition)
- A trait in Rust is basically a collections of methods for an imaginary type self. This collection can contain just the method signatures or can also provide implementation for some or all of its methods. These implementations provided by the trait are called **default implementation**.
- Any type can then implement this trait and will get all these methods. The implementing type must provide its own implementations for the methods in trait. Those methods inside the trait which come with default implementation can be overridden or accepted.
- In traditional Object Oriented programming we share behaviour mainly by inheretance i.e an object will inhereted all the methods from   

Traits in Rust language are used for three main purposes.

1. Sharing Behaviour
2. Code reuse using Static Dispatch by trait bounds
6. Code reuse using Dynamic Dispatch by  trait objects
---
we have
    1. concrete data types
    1. generic data types
    1. trait objects
[More on trai objects: The Book](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)    


## Generics
- For generics we use angled brackets. With in these brackets we can mention the type of the type and then the traits that it support

```rust
fn print_number<T: Debug>(number: T) { // <T: Debug> is the important part
    println!("Here is your number: {:?}", number);
}
```