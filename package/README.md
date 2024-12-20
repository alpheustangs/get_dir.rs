# Get Dir

An utility to get directory.

This utility searches for a target directory by checking for any directories or files that match the provided input.

## Usage

Get directory by target with the following code:

```rust
use get_dir::{
    Target,
    TargetType,
    get_dir_by_target,
};

get_dir_by_target(Target { 
    name: "src", 
    ty: TargetType::Dir,
});
```

Or get directory by target in reverse with the following code:

```rust
use get_dir::{
    Target,
    TargetType,
    get_dir_by_target_reverse,
};

get_dir_by_target_reverse(Target {
    name: "LICENSE",
    ty: TargetType::File,
});
```

## License

This project is licensed under the terms of the MIT license.
