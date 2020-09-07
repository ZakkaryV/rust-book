# Chapter 10 → Generic Types, Traits and Lifetimes

## Module scope
   ◦ Unrelated to this chapter but nonetheless relevant: 
      ◦ Modules are brought into scope with `mod some_file_name.rs` where some_file_name.rs is a sibling file
      ◦ *Re-exports* are accomplished via `pub use some_file_name::{Stuff}`
   ◦ Only `main.rs` (bin) and `lib.rs` must explicitly bring modules into scope
      ◦ Other modules are free to import eachother with the `use some_file_name` syntax, `crate::*` syntax or `super::*`

## Generic Types
> fn largest<T>(list, &[T]) → T
   ◦ Generics are stand-ins for concrete types like i32, String etc. 
   ◦ Potential concrete types passed as type arguments implement traits that provide methods, but the compiler must be informed explicitly of the traits implemented by `T`
   ◦ Generics can be implemented in Structs, Enums, Fumust be informed explicitly of the traits implemented by 
   ◦ Implementations can be made separate for structs with generics, ie
      > struct Point<T> { x: T, y: T }
      > impl<T> Point<T> { fn x(&self) → &T }
      > impl Point<i32> { fn n(&self) → i32 }
   ◦ This impl block only applies to `Point<i32>`, only this type will have an `n` method
   ◦ Generics must not necessarily be passed as arguments: they can be inferred
      > let pt = Point { x: 10.5, y: 20.0 }
      >                     ^^^^^ f32 is inferred here
      >                           the compiler will complain if a different type attempts to initialize a subsequent field
   ◦ `n` types can be specified, but more than 2 is not advised as the code will become difficult to read
      > Point<T, U> // these letters, conventionally
   ◦ “Monomorphization” → A compiler technique where generic types are converted into concrete types at compile time. 
      ◦ This is a “Zero cost abstraction”
      ◦ Rust exhausts every path a generic is used, creating separate definitions, ie
         > Some(10.0) // becomes...
         > Option_f32::Some(10.0) // at runtime
      ◦ This greatly increases runtime performance... but XXX what language would `not` do this?

## Traits
   ◦ Similar to a feature known in other languages as “interfaces”. They are an abstract way of sharing functionality across various types.
   ◦ Methods are bound to a Trait declaration with no function body, *just the signature* 
      ◦ Structs that are implementors of the trait are tasked with creating a function body. 
      ◦ The compiler just enforces that implementors abide by the signature contract
   ◦ The *Orphan rule* is one of many *coherence programs* in Rust; it applies to traits. 
      ◦ Types can only implement traits in their local implementations, ie module *A* exports a *T*, module *B* cannot implement traits for *T* because his parent *A* is missing
   ◦ Traits can have “default implementations” where the method body is actually specified along with the type signature.
         ▸ To use the default impl simply omit the method definitions, eg
         > impl Summarize for Tweet { // nothing }
   ◦ Traits as parameters
      ◦ `impl` syntax can be used to specify that a fn property is not a concrete type, but one that implements a given trait, ie
      > fn notify(some_type: &impl SomeTrait) → any;
      >                      ^^^^^^^^^^^^^^^
      ◦ A limitation to keep in mind is that the compiler can only verify methods of the trait
   ◦ Trait bounds
      ◦ Generic types can specify trait `bounds`, which is to say that they can any concrete `T` but must have certain trait(s)
      ◦ Traits bounds are specified in `<>` along with the generic type, and any number of traits can be passed, delimited with `+`
      > fn notify<T: SomeTrait + AnotherTrait + AndAnotherTrait>(item, &mut T) → T;
   ◦ `where` syntax
      ◦ Types with many trait bounds can be hard to read. Income `where syntax`
      > fn notify<T, U>(item: &T) → String
      >   where T: Display + Clone + AnyOtherTrait
      >         U: Summary
      > { }
   ◦ Functions that return types that implement certain traits
      ◦ Return types must not necessarily be a concrete:
      > fn returns_displayable() → impl fmt::Display {
      >   Tweet {
      >     ...
      >   }
      > }
      ◦ A caveat is that you can only return *a single type*. This is some limitation on the “implementation of impl Trait syntax”
