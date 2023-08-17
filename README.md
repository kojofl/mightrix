# mightrix

The mightrix crate exposes matrix types that let continuous memory be used as
if it where a matrix. The dimensions of the matrix is asserted through const
generics. This way the owned variant of the matrix `Stacktrix` can use
a fixed size array on the stack.
