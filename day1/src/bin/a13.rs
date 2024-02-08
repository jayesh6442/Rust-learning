enum Direction  {
    Up,
    Left,
    Right,
    Down,
}

fn main(){
    let go = Direction::Right;
    match go{
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
        Direction::Up => println!("go up"),
        Direction::Down => println!("go down"),
        
    }
}