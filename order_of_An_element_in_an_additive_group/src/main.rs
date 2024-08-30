fn main() {
    let a:i32=6;
    let modulo:i32 = 10;
    println!("order of {} mod {} is: {:?}",a,modulo,order_of_an_element(a,modulo));
    println!("order of {} mod {} is: {:?}",a,modulo,get_order_of_an_element(a,modulo));
}

fn order_of_an_element(a:i32,modulo:i32)->i32{
    let mut count=1;
    let mut res=a%modulo;
    while res !=0{
           res=(res+a)%modulo;
           count+=1;
        
    }
    count
    
}

// formula:
/*
order of a group=n/gcd(a,n)
where for example n is G(n,+)...
eg G(10,+). Find the order of 6
10/gcd(10,6)

*/
fn get_order_of_an_element(mut a:i32,modulo:i32)->i32{
    // find gcd
    let mut b=modulo;
    while b !=0{
        let temp=b;
        b=a%b;
        a=temp;
    }
    let gcd =a;
    let res=modulo/gcd;
    res
}
