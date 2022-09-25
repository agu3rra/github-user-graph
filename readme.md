# Github User Graph
This crate uses the [GitHub's REST API](https://docs.github.com/en/rest) to obtain the list of followers and other users a given GitHub user follows. It's meant to be used as an exercise and code reference for interations with HTTP API's in Rust using [tokio](https://tokio.rs/) and [reqwest](https://crates.io/crates/reqwest). Additionally it's also a useful exercise on how to create a Rust crate (library). Finally, it could potentially be used in tandem with [neo4j](https://neo4j.com/developer/) in order to programatically generate a graph representing a user and its relationships.

# Install
> cargo install github-user-graph

# Usage
```rust
use github_user_graph::{Github, UserGraph};

let gh = Github::new(
    base_url: "https://github.com/api/v3",
    token: "aTokenWithReadAccessToUsers",
);
let user_graph:UserGraph = gh.get_user("agu3rra")
```
