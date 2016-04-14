use std::hash::{Hash, Hasher};
use std::fmt;

pub struct Node
{
    pub state: Vec<usize>,
    pub len: usize
}

impl Hash for Node
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.state.hash(state);
    }
}

impl fmt::Display for Node 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let padding = format!("{}", self.len * self.len).len();
        write!(f, "{}" ,(0..self.len * self.len)
            .map(|i| format!("{1:00$}{2}", padding, self.state[i], if (i + 1) % self.len == 0 { "\n" } else { " " }))
            .collect::<String>())
    }
}

impl Node
{
    fn swap(&self, sq1: usize, sq2: usize) -> Node
    {
       let mut v = self.state.clone();
       let save = v[sq1];
       v[sq1] = v[sq2];
       v[sq2] = save;
       Node { state: v, len: self.len }
    }

    pub fn get_neighbour(&self) -> Vec<Node>
    {
        let mut res = Vec::with_capacity(4);
        let size = self.len;
        let x = || -> usize
        {
            for i in 0..size*size
            {
                if self.state[i] == 0 { return i; }
            }
            panic!("The empty square is missing");
        }();
        if (x % size) > 0 { res.push(self.swap(x, x - 1)); }
        if x > size { res.push(self.swap(x, x - size)); }
        if (x + 1 % size) > 0 { res.push(self.swap(x, x + 1)); }
        if (x + size) < (size*size) { res.push(self.swap(x, x + size)); }
        res
    }
}
