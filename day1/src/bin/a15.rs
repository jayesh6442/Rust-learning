



struct Grocery{
    stock: i32,
    price: f64,
}

fn main(){
    println!("we are in a15");
    let cereal = Grocery{
        stock: 19,
        price: 2.99,
    };
    println!("stock {}",cereal.stock);
    println!("price {}",cereal.price)
}