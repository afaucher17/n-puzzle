use node::{Node, Goal};

pub enum Heuristic
{
    Manhattan,
}

impl Heuristic {
    fn manhattan(node: &Node, goal: &Goal) -> i32
    {
        let mut score: i32 = 0;
        for i in 0..node.len * node.len
        {
            let square = node.state[i];
            if let Some(&(x, y)) = goal.map.get(&square) {
                score += if square == 0 { 0 }
                else { 
                    (x as i32 - (i % node.len) as i32).abs()
                    + (y as i32 - (i / node.len) as i32).abs()
                }
            }
        }
        score
    }

    pub fn get_score(&self, node: &Node, goal: &Goal) -> i32
    {
        match *self {
            Heuristic::Manhattan => Heuristic::manhattan(node, goal)
        }
    }
}
