## Learning Rust with Advent of Code 2021

I have read the first couple of chapters of the rust book and watched some videos but, this is my first hands on experience with the language.
Besides the solutions, I will also try to document interesting things I found about the language.

Use `cargo run --bin dayN` to build and run the solution for day N.

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

Edit1: I think this was a wise decision that I can't grasp currently.

### Day 4

There is a borrow checker rule that I don't understand. Here is a minimal scenario:

Given these 2 functions
````rust
fn increase_by_one(elements: &mut [i32]) {
    for element in elements {
        *element += 1;
    }
}

fn first_larger_than(elements: &[i32]) -> Option<&i32> {
    for element in elements {
        if *element > 6 {
            return Some(element);
        }
    }
    return None;
}
````
and these variables
`````rust
let mut arr = [1, 2, 3, 4];
let l: Option<&i32>;
`````

This works
````rust
loop {
    increase_by_one(&mut arr);
    if let Some(val) = first_larger_than(&arr) {
        l = Some(val);
        break;
    }
}

println!("{:?}", l);
````

But this does not
`````rust
for _ in 0..10 {
    increase_by_one(&mut arr);
    if let Some(val) = first_larger_than(&arr) {
        l = Some(val);
        break;
    }
}

println!("{:?}", l);
`````
giving the error message
`````text
error[E0502]: cannot borrow `arr` as mutable because it is also borrowed as immutable
  --> src\main.rs:36:25
   |
36 |         increase_by_one(&mut arr);
   |                         ^^^^^^^^ mutable borrow occurs here
37 |         if let Some(val) = first_larger_than(&arr) {
   |                                              ---- immutable borrow occurs here
...
70 |     println!("{:?}", l);
   |                      - immutable borrow later used here
`````

It is definitely not about the `for x in y` syntax because this version also doesn't compile, giving the same error
`````rust
let mut i = 0;
loop {
    if i < 10 { break; }
    i += 1;

    increase_by_one(&mut arr);
    if let Some(val) = first_larger_than(&arr) {
        l = Some(val);
        break;
    }
}
`````

### Day 5
`type` keyword just creates an alias for the given type, it does not have access to the type's methods.
`````rust
type u32x2_type = (u32, u32);
struct u32x2_struct(u32, u32);

fn main()
{
    let p = u32x2_type(1, 2); // error[E0423]: expected function, found type alias `u32x2_type`
    let p = u32x2_struct(1, 2);
}
`````
the `u32x2_struct` has no methods though, a derive attribute is also needed to make it work like a tuple.
