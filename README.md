# rev_slice

A simple alternative to negative indexing on rust slices

Ever wanted `a[-1]` in Rust?  With this crate, you can do that as `a.rev()[0]`.

Also provided are the other slice methods on the reversed slice, and you can
`.rev()` again to get the original slice back.  So you can, for example,
remove the last element of a slice with `a.rev()[1..].rev()`.
