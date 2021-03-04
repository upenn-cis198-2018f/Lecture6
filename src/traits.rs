/*
    Traits in Rust

    A running example using familiar concepts: address book
    - Database of information about people
    - Fields in the database: struct
    - Whole database: HashMap
*/

use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Person {
    name: String,
    age: u8,
    phone: [u8; 10], // fixed array of 10 digits
    favorite_color: String,
}

pub struct AddressBook {
    // should not really be public, normally would want to hide implementation
    // details in the API for your data structure
    pub by_name: HashMap<String, Person>,
    pub by_age: HashMap<u8, Vec<Person>>,
}
impl AddressBook {
    pub fn new() -> Self {
        Self {
            by_name: HashMap::new(),
            by_age: HashMap::new(),
        }
    }
    pub fn add_person(&mut self, person: Person) {
        self.by_name.insert(person.name.clone(), person.clone());
        // Should be using the entry API
        // But I'm just illustrating here
        self.by_age.insert(person.age, Vec::new());
        self.by_age.get_mut(&person.age).unwrap().push(person);
    }
}

/*
    ***** QUIZ *****

    1. What will fail to compile the above code?
    A: we needed to derive Clone and use .clone()

    2. What would happen now if we write a test and do
        assert_eq!(Person1, Person2)
       ?
    A: We needed to derive Debug and PartialEq
*/

#[test]
fn test_assert_eq_person() {
    let person1 = Person {
        name: "caleb".to_owned(),
        age: 26,
        phone: [5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
        favorite_color: "Purple".to_owned(),
    };
    let person2 = person1.clone();
    assert_eq!(person1, person2);
}

/*
    ***** Part 1 *****
    Implementing a trait for a type
*/

/*
    Clone and Copy

    Clone is for .clone() to copy all your data from one place in memory
    to another.

    Copy is for automatically cloning on function calls and other places
    so you don't have to worry about it.

    Q: Should we implement Copy for Person?
    A: Probably no, Person has String fields which are dynamically growable,
       so we don't really want to automatically copy large chunks of memory
       without making that explicit and making the user think about it.
       Copy is mainly used for integer, char, etc. data types.

    Q: Should we implement Copy for AddressBook?
    A: Even worse, definitely no.
*/

/*
    So far: to implement a trait, we have seen the #[derive(...)]
    syntax.

    What if we want a different implementation than the default provided
    by #[derive(...)]?

    #[derive(...)] is just magic that calls a macro to generate some code
    to implement your trait automatically in some canonical or "obvious" way.
    If you want something less obvious or different than the default,
    you can write that code yourself.
*/

// Let's take Eq as an example, what would be the automatically generated Eq
// code?
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.age == other.age
            && self.phone == other.phone
            && self.favorite_color == other.favorite_color
    }
}

// But maybe we want to say two people are equal if they have the same name?
// impl PartialEq for Person {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name
//     }
// }

// In general, the way you implement a trait:
// - First you look up the documentation or the source code for the trait
//   which will have a list of methods that you need to implement.
//   For example, Clone requires that you implement
//       fn clone(&self) -> Self
//   PartialEq requries that you implement
//       fn eq(&self, other: &Self) -> bool
//   and so on.
//   Some traits have more than just one function.

/*
    ***** Part 2 *****
    More standard library traits
*/

/*
    Debug and Display
    The displaying traits in Rust
        Debug: "{:?}"
        Display: "{}"
    The reason they're different? The idea is to keep
    straight different purposes: Debug should be
    "print this out for a developer to take a look at and understand"
    Display should be "print this out to the end user of the application"
*/

// Let's try implementing display for our Person type
// (we could also do it for AddressBook)
// Brief note:
//    Display and Debug make use of certain types specific to the fmt
//    module, so you need fmt::Formatter and fmt::Result in particular.
//    you can think of fmt::Result as just a customization of Result<(), String>
impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Person with name {}, age {}, other details omitted",
            self.name, self.age,
        )
    }
}

#[test]
fn test_display_person() {
    let caleb = Person {
        name: "caleb".to_owned(),
        age: 26,
        phone: [5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
        favorite_color: "Purple".to_owned(),
    };
    println!("{}", caleb);
    // Useful trick:
    // Test output is suppressed for passing tests :(
    // if you want to see the output from a test,
    // try causing the test to fail.
    // assert!(false);
}

/*
    From and Into

    Exactly what it sounds like: going from one type to another.

    Important note: If you implement From, then Into is derived
*/

const DEFAULT_PHONE: [u8; 10] = [5, 5, 5, 5, 5, 5, 5, 5, 5, 5];

impl From<(String, u8)> for Person {
    // Convert a String to a Person
    fn from((name, age): (String, u8)) -> Self {
        Person {
            name,
            age,
            phone: DEFAULT_PHONE,
            favorite_color: "Unknown".to_string(),
        }
    }
}

#[test]
fn test_from_into() {
    let caleb = Person::from(("caleb".to_owned(), 26));
    assert_eq!(caleb.phone, DEFAULT_PHONE);

    // Into is automatically implemented by the std library
    // if you implement From
    let caleb2: Person = ("caleb".to_owned(), 26).into();
    assert_eq!(caleb, caleb2);

    // Note: from is an associated function (doesn't take self),
    // so called with ::from
    // into is a method, so called with .into().
}

/*
    Default

    Rust uses Default for objects which can be safely initialized automatically
    to some default value.

    If you implement Default, you are saying "it's fine to not initialize me, I
    will just be the default value in that case"

    Good example of NOT deriving a trait: by not implementing Default
    for Person, we indicate to users of our data structure or library
    that they should always initialize a Person. ==> compiler error
    if you try to use Person with an API that requires Default.

    But Default does make sense for AddressBook, because you can have
    an empty address book and that's fine.
*/

impl Default for AddressBook {
    fn default() -> Self {
        Self {
            by_name: Default::default(),
            by_age: Default::default(),
        }
        // ^^ HashMap implements Default!
    }
}

#[test]
fn test_address_book_default() {
    // People usually do this
    let a1: AddressBook = Default::default();
    // This should work too
    let a2 = AddressBook::default();
    // Could do this, need to first implement Debug and PartialEq, just
    // like we did for Person
    // assert_eq!(a1, a2);
}

/*
    FromStr: can-fail conversion

    for "Parsing" -- any time you want to parse your data from a String,
    you want to implement FromStr

    Q: Why don't I just implement From for String?

    impl From<String> for Person {
        ...
    }

    Parsing can fail!
    We would prefer not to panic on failure, and to return
    a Result error.
*/

// impl FromStr for Person {
//     // New we haven't seen -- specify a type as part of the trait
//     // Called an "associated type"
//     // It's actually internally similar to something we have seen:
//     // recall From syntax:
//     // From<(String, u8)>
//     // (String, u8) is basically the same thing -- a type that you
//     // specify along with implementing the trait.
//     type Err = String;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         // Complex parsing logic here
//         // Parsing can sometimes be annoying
//         // &str API has a bunch of useful functions, particularly
//         // matching patterns
//         // https://doc.rust-lang.org/std/primitive.str.html
//         // Rough pseudocode:
//         // - Check if the string starts with 'Person'
//         // - Then split the remainder of the string by a separator
//         //   character ',', by calling .split()
//         // - For each part, try to parse it as the corresponding
//         //   field of Person
//         // - For each line that fails, use the ? operator:
//         //       let name = split_parts[2].parse()?;
//         //   That forwards the error case to return from the function
//         //   immediately.
//         // In the end you would end up with something where you
//         // can call "Person(caleb, 26, ...)".parse()
//         // to get a person object.
//     }
// }

/*
    Common operations:
    std::ops::{Index, IndexMut, Add, Mul}

    Index, IndexMut, Add, Mul

    These correspond to specific syntax:
    Index, IndexMut:
        []
    Add:
        +
    Mul:
        *
    Others, e.g. AddAssign for +=

    For our AddressBook type, of these,
    it seems like Index and IndexMut make sense, not necessarily
    the others
*/

// Maybe we want to directly access a Person entry by name
// impl Index<str> for AddressBook {
//     fn index(&self, idx: &str) -> &Person {
//         self.by_name[idx]
//     }
// }

// if I have an AddressBook a
// I can use the syntax println!("{}", a["caleb"])
// rather than using another method.

/*
    *******************************
    End of part 1 -- will continue next time.
    *******************************
*/

/*
    Others:
    - AsRef
    - Borrow
    - Read / Write
    - Iterator
*/

/*
    Not technically in the standard library, but so widespread and
    idiomatic that they are standard:

    StructOpt
    https://crates.io/crates/structopt
    - StructOpt trait: if you want to parse your data from the command line

    Serde
    https://crates.io/crates/serde
    - Serialize and Deserialize traits
*/

/*
    ***** Part 3 *****
    Defining your own traits
*/
