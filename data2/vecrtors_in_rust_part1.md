
# Vectors in Rust part-1
I have been reading about rust for a while now and the way I can explain vectors in rust is ::
> Vectors are just like arrays i.e they are a linear collection of similar objects But Vectors unlike arrays are allocated on heap and can grow or shrink.

So this tell us that in case of a vector as well as array we need to know the data type that has to be stored in it BUT in case of array we also need to know the total number of items where as incase of a vector we dont need that.

---
From the Rust Book:: chapter 8 :: Common Collections
> Rust’s standard library includes a number of very useful data structures called collections Most other data types represent one specific value, but collections can contain multiple values. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time.
also
>  vectors are indexed by number, starting at zero.
### Among the most important collections are:
- Vectors
- Strings
- Hash maps 

## Vectors in Rust
>  Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.
## Creating a new Vector
```rust
let v: Vec<i32> = Vec::new();
or
let v = vec![1, 2, 3];
```
> Vectors are implemented using generics

## Updating a Vector
```rust
fn main() {
  let mut v:Vec<i32> = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);
println!("{:?}",v);
println!("vector Capacity :: {:?}",v.capacity());
println!("vector Capacity :: {:?}",v.len());
}
```
- Result is capacity = 4 , len = 4
> The internal implementation of vector is such that it is just a struct which holds a pointer to heap allocated data. the length and capacity properties are "impl" on to this structure.
 ## Reading Data from a Vector
 We can read data from a vector directly using a reference or using a get method. Lets check the get method with out using pattern matching (which is not advisable).
```rust
fn main() {
let mut v:Vec<u32> = Vec::new();

v.push(1);
v.push(2);
v.push(3);
v.push(4);
v.push(5);

println!("1:: {:?}",v.get(1));
println!("2:: {:?}",v.get(2));
println!("3:: {:?}",v.get(3));

}
```

 The Result is 
 ```
 1:: Some(2)
2:: Some(3)
3:: Some(4)
```
### lets look at correct way using pattern matching
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
```
The main point to understand is that while using &i32 we get a reference and while using "get" method we get Option<&T>. Thus making the "get" method very safe.
Both of these methods behave very differently incase an index is called that does not exist.

## Side Trip :: Lets look at the above code again from pattern matching point of view
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(t) => println!("The third element is {}", t),
        None => println!("There is no third element."),
    }
}
```
> when we use get we getback an enum type called "option". This enum has 2 variants "Some" and "None". The sum is a method where as None show the presence of nothing.
> Options are commonly paired with pattern matching to query the presence of a value and take action, always accounting for the None case.
> Incase we call an index which does not exist e.g "&v[500]" , this will panic, However while using "get" and "match" it will continue working fine.
---
In the above code argument name of the Some inside match block does not matter. What ever has been returned will be handed over to what ever name you place in the parenthesis after Some (in this case we have "t").

... lets get back to vectors in part-2
---
