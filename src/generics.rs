/*
    Generics in Rust

    "Generic" means across different types!
    Most commonly, two things in Rust are generic: structs and functions.
*/

use std::fmt::Debug;

/*
    Generic Structs
*/

pub struct SortedVector<T> {
    sv: Vec<T>,
}

/*
    INTERNAL DETAILS: what does this <T> really mean?

    - C++ templates

      https://codegolf.stackexchange.com/questions/1956/generate-the
      -longest-error-message-in-c
      *Cries in C++*

    - Zero-cost abstraction
*/

// Syntax for an impl block
impl<T> SortedVector<T> {
    /* What should we implement for SortedVector? */
}

// Can we generalize our AddressBook example?

// D: address book data
// F1, F2: address book fields
pub struct AddressBookGen<F1, F2, D> {
    pub by_field1: HashMap<N, T>,
    pub by_field2: HashMap<A, Vec<T>>,
}

/*
    Generic Functions
*/

pub fn get_first<T>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        None
    } else {
        Some(&list[0])
    }
}

/*
    Generic Trait Bounds
*/

impl<T: Debug> Debug for SortedVector<T> {
    fn fmt(&self, f: &mut Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        self.sv.fmt(f)
    }
}

// Either the trait or the type must be defined in this crate.
// Avoids clashes in implementations.

impl<P: PartialOrd> Into<Vec<P>> for SortedVector<P> {
    fn into(self) -> Vec<P> {
        unimplemented!();
    }
}

// Even multiple parameters!
trait Graph<N, E> {
    fn has_edge(&self, &N, &N) -> bool;
    // ...
}

/*
    Iterators!
    Iterators in Rust are powerful, and encapsulated by the
    Iterator **trait*, not a type.

    Q: why is Iterator a trait and not a type?
*/

// For iterators we use "associate types."
pub trait Iterator {
    type Item; // Associated type!
    fn next(&mut self) -> Option<Self::Item>;
}

// See how it's used in collect! Basically magic!
// https://doc.rust-lang.org/src/core/iter/iterator.rs.html#1414
// Collect requires FromIterator:
// https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-FromIterator%3CT%3E
