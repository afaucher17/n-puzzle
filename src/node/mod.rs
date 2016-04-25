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
		if (x + 1 % size) > 0 { res.push(self.swap(x, x + 1)); }
		if (x + size) < (size*size) { res.push(self.swap(x, x + size)); }
		res
	}

    pub fn get_score(&self, goal: &Goal, h: &Heuristic) -> i32
    {
        h.get_score(self, goal) 
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
        let mut inc = || -> usize {
            let save = if count < (size * size) { count } else { 0 };
            count += 1;
            save
        };
        loop
        {
            for i in left...right { map.insert(count, (top, i)); tab[top * size + i] = inc();}
            top += 1;
            if top > down || left > right { break; }
            for i in top...down { map.insert(count, (i, right)); tab[i * size + right] = inc();}
            right -= 1;
            if top > down || left > right { break; }
            for i in (left...right).rev() { map.insert(count, (down, i)); tab[down * size + i] = inc();}
            down -= 1;
            if top > down || left > right { break; }
            for i in (top...down).rev() { map.insert(count, (i, left)); tab[i * size + left] = inc();}
            left += 1;
            if top > down || left > right { break; }
        }
		Goal { node: Node { state: tab, len:size }, map: map }
	}
}
