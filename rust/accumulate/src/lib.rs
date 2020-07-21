/// What should the type of _function be?
pub fn map<A, B, F: FnMut(A) -> B>(input: Vec<A>, mut function: F) -> Vec<B> {
    let mut new_map = Vec::with_capacity(input.len());
    for inp in input {
        new_map.push(function(inp));
    }
    new_map
}
