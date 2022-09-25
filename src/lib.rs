//! github_user_graph
//! 
//! `github_user_graph` is a crate to facilitate the consumption of Github's users REST API endpoint from Rust and aid into building a graph representing a network of a user's followers and other users that the the input user follows.

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
