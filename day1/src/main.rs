fn main() {
    //Display the massage to the user.
    println!("Hello, world!");
    let a = sum(4, 6);
    println!("{}",a);

    let mut b = 1;
    loop {              //for loop
        if b==5{        // go till the b  not equal to 5
            break;      //break if value meet the condition
        }
        println!("{}",b);
        b=b+1;
    }
    let mut c = 0;
    while c!=5 {            //while loop
        println!("{}",c);   
        c = c + 1;
        
    }

    
}


fn sum(a:i32,b:i32)->i32{
    a+b
}