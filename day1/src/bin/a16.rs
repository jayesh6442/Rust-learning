
enum Flavors {
    Sparkling,
    Sweet,
    Fruty
}
struct Drink{
    flavor:Flavors,
    fluid_oz: f64,
}
fn main(){
        let sweet = Drink{
            flavor:Flavors::Sweet,
            fluid_oz: 6.0
        };
}
fn printDrink(drink: Drink){
    match drink.flavor  {
        Flavors::Sparkling => println!("sparkling"),
        Flavors::Sweet => println!("sweet"),
        Flavors::Fruty => println!("frut"),
    }
    println!("oz {}",drink.fluid_oz);
}