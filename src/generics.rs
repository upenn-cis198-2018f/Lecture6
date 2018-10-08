// Must import the trait to use it!
use std::io::Read;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;


struct SortedVector<T> {
    sv: Vec<T>,
}

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}


// Parametric Polymorphic.
// Type is basically an argument, type variable
// doesn't affect computation of function.
fn get_first<T>(list: &[T]) -> Option<&T> {
    if list.len() >= 1 {
        Some(& list[0])
    }else {
        None
    }

}

// Traits:
// Traits define the types of things we can do on types.
// Similar to java interfaces. How are they different:
// 1) We can add traits to other types, defined in other modules.
// Example: serde: automatically get serialize and deserialize for our types.
// 2) Statically dispatched... more on this later.
// Notice: Rust is not! Object oriented: no subtyping.

// How is this different from C++ style templates?
// Well:
// https://codegolf.stackexchange.com/questions/1956/generate-the-longest-error-message-in-c
// *Cries in C++*

// Ad-hoc polymorphism.
// Implemnentation of read_to_string depends on the type of R: Read that we have.
// Trait Bound!
fn get_results<R: Read>(reader: &mut R) -> String {
    let mut string = String::new();
    reader.read_to_string(&mut string).expect("Failed to read from source.");
    string
}

// This is Polymorphism
// Other types of polymorphism that you may know: Subtyping or duck typing.

// Ask: What is dynamic dispatch.
// Rust Uses static dispatch. No runtime cost to generics!
// Word of the day monomorphisation

// In newer Rust
fn get_result2(reader: &mut impl Read) -> String {
    let mut string = String::new();
    reader.read_to_string(&mut string).expect("Failed to read from source.");
    string
}


// Ask: Name of the traits that we know.
// Write, Read, Clone, Debug, Display, Serialize, Copy.

// Terminology: Types implement a trait.

// Trait Objects
// What if I want a heterogeneous array?
fn print_a_bunch_of_things(thing: & Debug) {
    println!("{:?}", thing);
}

fn print_my_things() {
    // Something like this would work in Java... Object
    // let v: Vec<impl Debug> = vec![5, 10.2, "hello", None];
    let things: Vec<& Debug> = vec![& 5, & 10.2, & "hello", & Some(5)];

    for v in things {
        print_a_bunch_of_things(v);
    }
}

// What is this &Debug? Why didn't Debug work? Not sized.

// Is debug the type? No! Rust messed up. This is instead a Trait Object.
// Requires dynamic dispatch.
// Instead we should use the following syntax:
fn print_a_bunch_of_things2(thing: & dyn Debug) {
    println!("{:?}", thing);
}

// Multiple trait bounds.
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

// Notice this implementation itself is generic!
impl <T: Debug> Debug for SortedVector<T> {
    fn fmt(&self, f: &mut Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        self.sv.fmt(f)
    }
}

// Either the trait or the type must be defined in this crate.
// Avoids clashes in implementations.


// Traits can be generic also!
// E.g
pub trait Into<T> {
    fn into(self) -> T;
}

impl <P: PartialOrd> Into<Vec<P>> for SortedVector<P> {
    fn into(self) -> Vec<P> {
        unimplemented!();
    }
}

// Even multiple parameters!
trait Graph<N, E> {
    fn has_edge(&self, &N, &N) -> bool;
    // ...
}

// But this has many issues:
// From: https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md
// These are neat though, allow us to define relationships between types: a la
// multi parameter type classes.

// Instead we use associate types.
pub trait Iterator {
    type Item; // Associated type!
    fn next(&mut self) -> Option<Self::Item>; // Use associated type
    //...
}

// See how it's used in collect! Basically magic!
// https://doc.rust-lang.org/src/core/iter/iterator.rs.html#1414
// Collect requires FromIterator:
// https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-FromIterator%3CT%3E
