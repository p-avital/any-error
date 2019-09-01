# Any Error: When you just want to use `?` in peace
Ever wanted to do like the cool kids and use `?` to handle your errors,
but Rust keeps bugging you that your error types can't be cast into your return Error type?
Me too, so I made this crate, with 3 levels of "Just let me use `?` damnit!".

## SomeError
This 0-sized type looses ALL information about the encountered Err:
it simply drops the content of the error.  
Really, using `Result<T, SomeError>` is the same as using `Option<T>`, only you return a `Result`,
which has the cool "unused_must_use" lint.

## AnyError
This error type really just boxes the error into a `Box<dyn Any>`,
and lets you downcast it when you need to recover the error.

If you need something more complex than "Some Error happened",
and all your error types implement `Any` (which most types do),
I recommend using this one.

## FormattedError
This type lets you keep a bit more information about the error, at least at runtime:
it simply formats it with `core::fmt::Debug`, saves the result of the formatting, and drops the error.

Probably the type I recommend using the least, considering that it's costlier than the others,
and still looses a lot of information.  