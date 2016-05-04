use std::collections::BinaryHeap;
use std::collections::HashMap;

use std::i32::MAX;

use node::Node;
use node::Goal;

use heuristic::Heuristic;

pub fn reconstruct_path(came_from: &HashMap<Node, Node>, start: &Node) -> Vec<Node>
{
    let mut total_path = vec![start.clone()];
    let mut current = start;
    loop
    {
        if !came_from.contains_key(current) { break }
        current = came_from.get(current).unwrap();
        total_path.push(current.clone())
    }
    total_path
}

pub fn astar(start: &mut Node, goal: &Goal, heuristic: &Heuristic) -> Option <Vec<Node>>
{
    let mut closed_set : Vec<Node> = Vec::new();
    let mut opened_set : BinaryHeap<Node> = BinaryHeap::new();
    let mut came_from : HashMap<Node, Node> = HashMap::new();
    let mut g_score : HashMap<Node, i32> = HashMap::new();
    start.score = heuristic.get_score(start, goal);
    opened_set.push(start.clone());
    g_score.insert(start.clone(), 0);

    loop
    {
        match opened_set.pop() {
            None => break,
            Some(current) => {
                if current == goal.node {
                    return Some(reconstruct_path(&came_from, &current))
                }
                closed_set.push(current.clone());
                for mut neighbour in current.get_neighbour()
                {
                    if let Some(_) = closed_set.iter().find(|&x| x == &neighbour) { continue }
                    let ref infinity = MAX;
                    let neighbour_score: i32 = *(g_score.get(&neighbour).unwrap_or_else(|| infinity));
                    let tentative_g_score: i32 = (g_score.get(&current).unwrap()) + 1;
                    match (opened_set.iter().find(|&x| x == &neighbour), tentative_g_score >= neighbour_score) 
                    {
                        (None, _) | (Some(_), false) => 
                        {
                            neighbour.score = tentative_g_score + heuristic.get_score(&neighbour, goal);
                            came_from.insert(neighbour.clone(), current.clone());
                            g_score.insert(neighbour.clone(), tentative_g_score);
                            if let None = opened_set.iter().find(|&x| x == &neighbour) 
                            {
                                opened_set.push(neighbour);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    None
}
