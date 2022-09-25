Notes taken during development to organize the chaos in my head.

# Table of Contents
<!-- TOC -->

- [Table of Contents](#table-of-contents)
- [Reference tutorial](#reference-tutorial)
- [Release profiles](#release-profiles)
- [Documentation](#documentation)
- [lib.rs](#librs)
- [Publishing](#publishing)

<!-- /TOC -->

# Reference tutorial
* Let's Get Rusty: https://www.youtube.com/watch?v=4TI153PIEDQ

# Release profiles
Cargo.toml can be used to customize `[profile.dev]` and `[profile.release]` profiles for optimizing the generated output of our builds.

# Documentation
Automatically generated via regular `//` comments and documentation ones `///`. Markdown supported. To document a file: `//!`. To generate and view:
> cargo doc --open

# lib.rs
Document internal modules as:
```rust
pub mod github {
    
} 
```

Then make the internals easier to consume by:
```rust
pub use self::github::{Github, UserGraph}
```

This shoud make them directly consumeable via the crate name as:
```rust
use github_user_graph::Github

// intead of
use github_user_graph::github::Github
```

# Publishing
1. Create an account on Crates.io;
1. Verify your e-mail;
1. Generate an API token;
1. `cargo login <token>`
1. Add crate metadata info on `Cargo.toml`;
1. `cargo publish`;
1. To stop your crate from being downloadable: `cargo yank --version <a-version>`;
