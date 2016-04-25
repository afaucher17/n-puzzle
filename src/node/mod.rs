use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::fmt;
use heuristic::Heuristic;

pub struct Node
{
    pub state: Vec<usize>,
    pub len: usize,
}

pub struct Goal
{
    pub node: Node,
    pub map: HashMap<usize, (usize, usize)>,
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
               .map(|i| format!("{1:00$}{2}", padding, self.state[i]
                                , if (i + 1) % self.len == 0 { "\n" }
                                else { " " }))
               .collect::<String>())
    }
}

impl fmt::Display for Goal
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.node)
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
        if ((x + 1) % size) > 0 { res.push(self.swap(x, x + 1)); }
        if (x + size) < (size*size) { res.push(self.swap(x, x + size)); }
        res
    }

    pub fn get_score(&self, goal: &Goal, h: &Heuristic) -> i32
    {
        h.get_score(self, goal) 
    }

    fn get_linear(&self) -> Vec<usize>
    {
        let mut linear: Vec<_> = Vec::new();
        let size: usize = self.len;
        let (mut top, mut down, mut left, mut right) = (0, size - 1, 0, size - 1);
        loop
        {
            for i in left...right { linear.push(self.state[top * size + i]); }
            top += 1;
            if top > down || left > right { break; }
            for i in top...down { linear.push(self.state[i * size + right]); }
            right -= 1;
            if top > down || left > right { break; }
            for i in (left...right).rev() { linear.push(self.state[down * size + i]); }
            down -= 1;
            if top > down || left > right { break; }
            for i in (top...down).rev() { linear.push(self.state[i * size + left]); }
            left += 1;
            if top > down || left > right { break; }
        }
        linear
    }

    pub fn is_solvable(&self) -> bool
    {
        let linear: Vec<usize> = self.get_linear().into_iter().filter(|&x| x != 0).collect();
        let mut row: usize = 0;
        let mut inversions: usize = 0;
        let odd = { |&x| x % 2 != 0 };
        let even = { |&x| x % 2 == 0 };
        for (i, el1) in linear.iter().enumerate()
        {
            if *el1 == 0 { row = i / self.len; continue; }
            for el2 in (linear[i + 1 .. linear.len()]).iter()
            {
                if *el2 != 0 && *el1 > *el2 { inversions += 1; }
            }
        }
        (odd(&self.len) && even(&inversions)) || (even(&(self.len)) && (odd(&row) == even(&inversions))) 
    }
}

impl Goal
{
    pub fn new (size: usize) -> Goal
    {
        let mut tab = vec![0; size*size];
        let mut map = HashMap::new();
        let (mut top, mut down, mut left, mut right) = (0, size - 1, 0, size - 1);
        let mut count: usize = 1;
        let mut inc = |mut cpt: usize| -> usize {
            let save = if cpt < (size * size) { cpt } else { 0 };
            cpt += 1;
            save
        };
        loop
        {
            for i in left...right { map.insert(count, (top, i)); tab[top * size + i] = inc(count);}
            top += 1;
            if top > down || left > right { break; }
            for i in top...down { map.insert(count, (i, right)); tab[i * size + right] = inc(count);}
            right -= 1;
            if top > down || left > right { break; }
            for i in (left...right).rev() { map.insert(count, (down, i)); tab[down * size + i] = inc(count);}
            down -= 1;
            if top > down || left > right { break; }
            for i in (top...down).rev() { map.insert(count, (i, left)); tab[i * size + left] = inc(count);}
            left += 1;
            if top > down || left > right { break; }
        }
        Goal { node: Node { state: tab, len:size }, map: map }
    }
}
