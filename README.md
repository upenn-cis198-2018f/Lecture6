# Lecture 6: Traits and Generics

## Reminders

- Homework 2 due today!

- Remember to email **before** the deadline to use late days (either the 4 freebies or additional late days) -- so that we make sure to delay grading your assignment.

## Outline of lecture

### Part 1 (March 4)

**Traits:** Traits are like `Copy`, `Clone`, `Debug`, and `Eq`.
Traits define *shared behavior* across different types.

- Implementing a trait for a type

- Overview of standard library traits

### Part 2 (March 11)

**Traits continued:**

- More advanced (`AsRef`, `StructOpt`, `Serde`)

- Defining your own traits

**Generics:**
Generics are like `Vec<T>`.
Generics are how you *generalize data structures and functions*, making them
work for different parameter types.

- Writing generic functions

- Making structs (and their methods) generic

- Using traits to enable truly generic programming
