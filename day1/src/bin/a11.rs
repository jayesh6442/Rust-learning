fn main(){
    let mut a = 1;
    loop {
        println!("{}",a);
        a+=1;
        if a==6 {
            break;
        }
    }
    println!("the loop is ended ");
}