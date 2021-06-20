//! # pipe_macros
//! A small macro library that allows you to pipe functions
//! similar to the pipe operator in Elixir and F# (|>), but also
//! with some special syntax sugar.
//!
//! # Syntax / Examples
//!
//! The first argument is a value, all following items are functions
//! that process the output of the previous in order. They are separated
//! by `=>` as operator
//! ```
//! # #![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]
//! #
//! # use pipeline::pipe;
//! # let value = ();
//! # let function1 = std::convert::identity;
//! # let function2 = std::convert::identity;
//! let foo = pipe![value => function1 => function2];
//! /* This is equivalent: */
//! let foo = function2(function1(value));
//! ```
//!
//! All functions must take exactly one argument and will be called like
//! `function(value)` on a syntactic level.
//!
//! Functions may of course be closures. Because all closures in a pipeline
//! must of course take exactly one argument, there is some special syntax sugar.
//! Simply use `_` as the input value and it'll automatically call it:
//!
//! ```
//! # #![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]
//! #
//! # struct Foo;
//! # impl Foo { fn function2(self) -> () {} }
//! # use pipeline::pipe;
//! # let value = ();
//! # let function1 = |_| Foo;
//! let foo = pipe![
//!     value
//!     => function1(_)
//!     => _.function2()
//! ];
//! /* This is equivalent: */
//! let foo = function1(value).function2();
//! /* This is too, but not in general
//!  * (ignore the type annotations, they're the reason we don't expand to this):
//!  */
//! let foo = (|it: Foo| it.function2()) ((|it| function1(it)) (value));
//! ```
//! 
//! The `_` syntax sugar is a plain text macro expansion. This means usual
//! control flow handling applies:
//!
//! ```
//! # #![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]
//! #
//! # use pipeline::pipe;
//! # let value = ();
//! # let fallible_function = |_| std::result::Result::<(), ()>::Ok(());
//! let foo = pipe![
//!     value
//!     => fallible_function(_)?
//! ];
//! # std::result::Result::<(), ()>::Ok(())
//! ```
//!
//! # When to use
//!
//! This is indeed very cool and it is tempting to over-use it. In general,
//! This crate shines in flattening deeply nested calls of the form `baz(bar(foo(val)))`.
//! The strength of Rust syntax lies in postfix function chaining. You should strive
//! to create APIs that are called like `val.foo().bar().baz()`.
//!
//! Strictly speaking this is not exactly what one would call "function composition" or
//! "point free style" in functional programming languages. The reason for that is that
//! an argument is always required; you can't (easily) chain methods together without also
//! calling them right away.
#![deny(missing_docs)]
#![deny(warnings)]

/// Lambda macro
#[cfg(not(feature = "nightly"))]
use proc_macro_hack::proc_macro_hack;

/// See the module level documentation
///
/// The code expands to something like:
///
/// ```ignore
/// {
///   let ret = value;
///   let ret = function1(ret);
///   let ret = ret.function2();
///   ret
/// }
/// ```
///
/// Functions containing no `_` are simply called by appending `(ret);`
/// Functions containing `_` are "called" by substituting `_` with `ret`.
/// If you encounter any edge cases that use any of these two as symbols in
/// a way that breaks the intended semantics, please open an issue.
#[cfg_attr(not(feature = "nightly"), proc_macro_hack)]
pub use pipeline_macro::pipe;
