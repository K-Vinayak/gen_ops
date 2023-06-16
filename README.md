# gen_ops

Rust macros for operator overloading of generic types. 

[![build](https://img.shields.io/github/actions/workflow/status/K-Vinayak/gen_ops/rust.yml?logo=github&style=flat-square)](https://github.com/K-Vinayak/gen_ops/actions/workflows/rust.yml)
[![docs](https://img.shields.io/docsrs/gen_ops/latest?style=flat-square)](https://docs.rs/gen_ops/)
[![crate](https://img.shields.io/crates/v/gen_ops?style=flat-square)](https://crates.io/gen_ops)
[![license](https://img.shields.io/crates/l/gen_ops?style=flat-square)](https://github.com/K-Vinayak/gen_ops/blob/main/LICENSE.md)

## Usage

The macros need four statements 

1. (Optional) Generic parameter names
2. Type signature or extended type signature
3. Callable expressions for each operator, and optionally, where clause for each operator
4. (Optional) Where clause for generic parameters

> **Note** 
> All statements end with a semicolon except the where clause.

## Example

```rust
#[derive(Debug, Copy, Clone, PartialEq)]
struct Pair<T>(pub T, pub T);

#[inline]
fn sub_pair<T>(a: &Pair<T>, b: &Pair<T>) -> Pair<T>
where T: Sub<Output=T> + Copy {
    Pair(a.0 - b.0, a.1 - b.1)
}

gen_ops!(
    <T>;                               // Generic parameter names
    types Pair<T>, Pair<T> => Pair<T>; // Type signature

    //add doc strings like below

    /// Add two pairs
    /// 
    /// Returns `Pair`
    for + call |a: &Pair<T>, b: &Pair<T>| {
        Pair(a.0 + b.0, a.1 + b.1)
    };                // Callable expressions for operators
    (where T: Add<Output=T>)

    for - call sub_pair;  // Or use an existing function
    (where T: Sub<Output=T>) //where clause for - operator only

    where T: Copy //Where clause for all impls
);

let a = Pair(2, 3);
let b = Pair(1, 8);

println!("a + b = {:?}", a + b); //a + b = Pair(3, 11)
println!("a - b = {:?}", a - b); //a - b = Pair(1, -5)
```

## gen_ops!

The primary macro for all operators. 

```rust
#[derive(Debug, Copy, Clone, PartialEq)]
struct Pair<T>(pub T, pub T);

gen_ops!(
    <T>;
    types Pair<T>, Pair<T> => Pair<T>;
    for + call |a: &Pair<T>, b: &Pair<T>| Pair(a.0 + b.0, a.1 + b.1);
    (where T: Add<Output=T>)

    for - call |a: &Pair<T>, b: &Pair<T>| Pair(a.0 - b.0, a.1 - b.1);
    (where T: Sub<Output=T>)

    where T: Copy
);

let a = Pair(10, 5);
let b = Pair(8, 9);

println!("a + b = {:?}", a + b); // a + b = Pair(18, 14)
println!("a - b = {:?}", a - b); // a - b = Pair(2, -4)
```

## gen_ops_comm!

Implements commutative operations. 

```rust
#[derive(Debug, Copy, Clone, PartialEq)]
struct Pair<T>(pub T, pub T);

gen_ops_comm!(
    <T>;
    types Pair<T>, i32 => Pair<T>;
    for * call |a: &Pair<T>, b:&i32| Pair(a.0 * *b, a.1 * *b);
    (where T: Mul<i32, Output=T>)

    for & call |a: &Pair<T>, b:&i32| Pair(a.0 & *b, a.1 & *b);
    (where T: BitAnd<i32, Output=T>)

    where T: Copy
);
let a = Pair(12, 3);

println!("a * 5 = {:?}", a * 5); //a * 5 = Pair(60, 15)
println!("5 * a = {:?}", 5 * a); //5 * a = Pair(60, 15)
println!("a & 2 = {:?}", a & 2); //a & 2 = Pair(0, 2)
println!("2 & a = {:?}", 2 & a); //2 & a = Pair(0, 2)
```

## gen_ops_ex!

Implements trait for borrowed types. 

```rust
#[derive(Debug, Copy, Clone, PartialEq)]
struct Pair<T>(pub T, pub T);

gen_ops_ex!(
    <T>;
    types mut Pair<T>, T => Pair<T>;
    for * call |a: &Pair<T>, b:&T| Pair(a.0 * *b, a.1 * *b);
    where T: Mul<Output=T> + Copy
);

let mut a = Pair(12, 3);
{
    let mut b = &mut a;
    println!("&mut a * 2 = {:?}", b * 2);// &mut a * 2 = Pair(24, 6)
}
println!("&a * 2 = {:?}", &a * 2);// &a * 2 = Pair(24, 6)
println!("a * 2 = {:?}", a * 2);// a * 2 = Pair(24, 6)
```

## gen_ops_comm_ex!

Implements commutative operations for borrowed types. 

```rust
#[derive(Debug, Copy, Clone, PartialEq)]
struct Pair<T>(pub T, pub T);

gen_ops_comm_ex!(
    <T>;
    types ref Pair<T>, i32 => Pair<T>;
    for * call |a: &Pair<T>, b:&i32| Pair(a.0 * *b, a.1 * *b);
    where T: Mul<i32, Output=T> + Copy
);

let a = Pair(12, 3);
println!("a * 5 = {:?}", a * 5); //a * 5 = Pair(60, 15)
println!("5 * a = {:?}", 5 * a); //5 * a = Pair(60, 15)
println!("5 * &a = {:?}", 5 * &a); //5 * &a = Pair(60, 15)
println!("&a * 5 = {:?}", &a * 5); //&a * 5 = Pair(60, 15)
```

# Docs
For more details see [docs](https://docs.rs/gen_ops).

# Inspiration

This project is inspired by [auto_ops](https://crates.io/crates/auto_ops)