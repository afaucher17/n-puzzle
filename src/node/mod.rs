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
		let (mut pos, mut cpt) = (0, 0);
		let mut inc: i32 = 1;
		for i in 1..size*size
		{
			tab[pos] = i;
			map.insert(i, (pos % size, pos / size));
			if size == cpt + 1 || tab[(pos as i32 + inc) as usize] != 0
			{
				inc = match inc {
					1 => size as i32,
					x if x == size as i32 => -1,
					-1 => -(size as i32),
					x if x == -(size as i32) => 1,
					_ => 0,
				};
				cpt = 1;
			}
			else { cpt += 1; }
			pos = (pos as i32 + inc) as usize;
		}
		Goal { node: Node { state: tab, len:size }, map: map }
	}
}
