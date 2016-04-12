use std::hash::{Hash, Hasher};

struct Node
{
    state: Vec<Vec<usize>>,
    len: usize
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.state.hash(state);
    }
}

impl Node
{
    fn swap(&self, sq1: (usize, usize), sq2: (usize, usize)) -> Node
    {
       let mut v = self.state.clone();
       let (x1, y1) = sq1;
       let (x2, y2) = sq2;
       let save = v[x1][y1];
       v[x1][y1] = v[x2][y2];
       v[x2][y2] = save;
       Node { state: v, len: self.len }
    }

    fn get_neighbour(&self) -> Vec<Node>
    {
        let mut res = Vec::with_capacity(4);
        let (x, y) = || -> (usize, usize)
        {
            for i in 0..self.len
            {
                for j in 0..self.state[i].len()
                {
                    if self.state[i][j] == 0 { return (i, j); }
                }
            }
            panic!("Call Batman");
        }();
        if x > 0 { res.push(self.swap((x, y), (x - 1, y))); }
        if y > 0 { res.push(self.swap((x, y), (x, y - 1))); }
        if x < self.len { res.push(self.swap((x, y), (x + 1, y))); }
        if y < self.len { res.push(self.swap((x, y), (x, y + 1))); }
        res
    }
}
