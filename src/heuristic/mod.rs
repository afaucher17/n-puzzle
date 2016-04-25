use std::ascii::AsciiExt;

use node::{Node, Goal};

pub enum Heuristic
{
    Manhattan,
    Linear,
    // Your new heuristic
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

    fn linear(node: &Node, goal: &Goal) -> i32
    {
        let mut score: i32 = 0;

        for (line, chunk) in node.state.chunks(node.len).enumerate()
        {
            for (i, el1) in chunk.iter().enumerate()
            {
                for (j, el2) in chunk[i + 1 .. chunk.len()].iter().enumerate()
                {
                    if let (Some(&(x, y)), Some(&(x2, y2))) = (goal.map.get(&el1), goal.map.get(&el2))
                    {
                        if y == line && y2 == line && (i as i32 - (i + j + 1) as i32) * (x as i32 - x2 as i32) < 0 { score += 2; }
                    }
                }
            }
        }
        for col in 0..node.len
        {
            let filtered: Vec<_> = node.state.iter().enumerate()
                                            .filter(|&(index, _)| index % node.len == col)
                                            .map(|(_, v)| v)
                                            .collect();
            for (i, el1) in filtered.iter().enumerate()
            {
                for (j, el2) in filtered[i + 1 .. filtered.len()].iter().enumerate()
                {
                    if let (Some(&(x, y)), Some(&(x2, y2))) = (goal.map.get(&el1), goal.map.get(&el2))
                    {
                        if x == col &&
                            x2 == col &&
                                (i as i32 - (i + j + 1) as i32) * (y as i32 - y2 as i32) < 0
                                { score += 2; }
                    }
                }
            }
        }
        println!("Linear: {}", score);
        score
    }

    pub fn str_to_heuristic(s: &str) -> Option<Heuristic>
    {
        
        match s.to_ascii_lowercase().as_ref() {
            "manhattan" => Some(Heuristic::Manhattan),
            "linear" => Some(Heuristic::Linear),
            // Your new heuristic string => heuristic enum
            _ => None
        }
    }

    pub fn get_score(&self, node: &Node, goal: &Goal) -> i32
    {
        match *self {
            Heuristic::Manhattan => Heuristic::manhattan(node, goal),
            Heuristic::Linear => Heuristic::manhattan(node, goal) + Heuristic::linear(node, goal),
            // Your new heuristic enum => heuristic function
        }
    }
}
