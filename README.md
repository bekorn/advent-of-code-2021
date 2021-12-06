## Learning Rust with Advent of Code 2021

I have read the first couple of chapters of the rust book and watched some videos but, this is my first hands on experience with the language.
Besides the solutions, I will also try to document interesting things I found about the language.

#### Compilation (TODO)
I don't know how to compile it 😅, _yet_. Curently CLion Rust plugin does it for me.

### Day1
Even the match statement has branches, its compile time state of borrow-or-not is singular.
This makes sense, since a static analysis may consider the worst case.
```rust
let v = vec![1, 2, 3];

let toggle = false;

let _ = match toggle {
    true => v,          //  borrow
    false => v.clone()  //  clone
};

print!("{}", v[0]); // error, 
```

Compiler message:
````
   |
3  |     let v = vec![1, 2, 3];
   |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
...
8  |         true => v,
   |                 - value moved here
...
12 |     print!("{}", v[0]); // error
   |                  ^ value borrowed here after move
````


**However**, the last line still gives the same error even if the toggle is known at compile time, i.e.
```rust
const toggle : bool = false;
```
I'm quite curious if this is because of a limitation or just a thing forgotten in the backlog. 