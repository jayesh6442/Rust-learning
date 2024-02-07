fn main(){
    let  mut i = 5;
    loop{
        println!("{}",i);
        i-=1;
        if i==0 {
            break;
        }
    }
    println!("printing is done");
}