use node::Node;

pub fn create_goal(size: usize) -> Node
{
    let mut tab = vec![0; size*size];
    let (mut pos, mut cpt) = (0, 0);
    let mut inc: i32 = 1;
    for i in 1..size*size
    {
        tab[pos] = i;
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
    Node { state: tab, len: size }
}
