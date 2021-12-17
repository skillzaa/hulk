# The Rust Book Ch 10 Traits Summary

### Definitions / Explanation
1. **#myview** Trait is just like **interface** in other languages with some super powers. Just like an interface its main / basic job is to present one or more function signatures. This interface is then implemented by various Types. 
1. A trait can also contain default implementation of some or all of the methods it presents. The user can accept the default implementation or can over-ride it. The methods for which the trait does not provide default implementation the user has to provide.
1. Once a type / user implements a trait it has to provide implementation for those methods for which the trait has not provided a default implementation.