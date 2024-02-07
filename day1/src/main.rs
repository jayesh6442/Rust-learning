fn main() {
    println!("Hello, world!");
    let a = sum(4, 6);
    println!("{}",a);

    let mut b = 1;
    loop {
        if b==5{
            break;
        }
        println!("{}",b);
        b=b+1;
    }
}


fn sum(a:i32,b:i32)->i32{
    a+b
}