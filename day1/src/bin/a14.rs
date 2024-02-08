enum Color {
    white,
    black,
    red,
    blue,
}
fn main(){
    color_picker(Color::black);
}

fn color_picker(m_color:Color){
    match m_color {
        Color::red => println!("red"),
        Color::white => println!("white"),
        Color::black => println!("black"),
        Color::blue => println!("blue"),
    }
}
