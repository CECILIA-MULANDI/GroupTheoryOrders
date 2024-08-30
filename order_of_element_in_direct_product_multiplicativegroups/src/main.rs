fn main() {
    let a=16;
    let zn=23;
    let b=43;
    let zm=52;
    println!("The order of ({a},{b}) in (z{zn}*z(zm) = {:?}",find_order_of_elements_in_direct_productgroups((a,b),zn,zm));
}

/*
Zm*Zn

*/
fn find_order_of_elements_in_direct_productgroups((a,b):(u32,u32),n:u32,m:u32)->u32{
    let order_a=find_order_of_individual_element(a,n);
    let order_b=find_order_of_individual_element(b,m);
    lcm(order_a,order_b)
}
fn find_order_of_individual_element(a:u32,modulo:u32)->u32{
    let mut counter=1;
    let mut res=a%modulo;

    while res!=1{
        res=(res*a)%modulo;
        counter+=1;
    }
    counter
}
fn gcd(mut a:u32,mut b:u32)->u32{
    while b!=0{
        let temp=b;
        b=a%b;
        a=temp;
    }
    a
}
fn lcm(a:u32,b:u32)->u32{
   (a*b)/gcd(a,b)
}