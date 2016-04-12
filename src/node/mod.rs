use std::hash::{Hash, SipHasher, Hasher};

struct Node
{
    state: Vec<Vec<usize>>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.state.hash(state);
    }
}
