# ReSTD

ReSTD (for **re**implementation **st**an**d**ard) is a project made to reimplemente all of the standard library of the rust programming language AND the core lib of the language.

## Goal

The goal is to offer an alternative implementation to that of the default compiler (in an idea similar to [`async-std`](https://docs.rs/async-std/latest/async_std/) or [`tokio`](https://docs.rs/tokio/latest/tokio/) in the case of asynchronous) but also and above all to offer an alternative to [`core`](https://doc.rust-lang.org/stable/core/) and [`alloc`](https://doc.rust-lang.org/stable/alloc/).

The project was initially started by curriosity and fun, and the goal is now to reproduce as faithfully as possible the API given by the Rust API and, as a first step, to be able to replace the std/core and alloc in any project with the implementations given by ReSTD. In addition, the documentation will be designed to be as precise and transparent as possible and to provide as much explanation as possible of the written implementations.
