fn main(){
    let x = sum(6, 6);
    disp(x);

}

fn sum(a:i32,b:i32)->i32{
    a+b
}

fn disp(result:i32){
    println!("{}",result)
}