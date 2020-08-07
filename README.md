# `yes-or-no`

A macro that generates an enum with the variants `Yes` and `No`.

```rust
use yes_or_no::yes_or_no;

yes_or_no!(EnableColor);

assert_eq!(EnableColor::from(true), EnableColor::Yes);
assert_eq!(EnableColor::from(false), EnableColor::No);
assert!(EnableColor::Yes.yes());
assert!(EnableColor::No.no());
```
