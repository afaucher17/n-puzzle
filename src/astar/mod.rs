use std::collections::BinaryHeap

fn reconstruct_path(cameFrom: &HashMap<Node, Node>, current: &Node) -> Vec<Node>
{
}

fn astar(start: &Node, goal: &Goal, heuristic: &Heuristic) -> Option <Vec<Node>>
{
    let mut closedSet: BinaryHeap<Node>;
    let mut openedSet: BinaryHeap<Node>;
    let mut cameFrom: HashMap<Node, Node>;
    let mut gScore: HashMap<Node, i32>;
    gScore.insert(start, 0);
    openedSet.push(start);
    start.score = heuristic::get_score(start, goal);

    loop
    {
        match openedSet.pop() {
            None => break,
            Some(current) => {
                if current == goal {
                    return Some(reconstruct_path(cameFrom, current))
                }
                closedSet.push(current);
                for neighbour in goal.get_neighbour()
                {
                    if neighbour == current { continue }
                    let tentativeGScore: i32 = 
                }
            }
        }
    }
    None
}
