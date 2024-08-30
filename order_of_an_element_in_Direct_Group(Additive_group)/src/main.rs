fn main() {
    let a=1;
    let b=1;
    let zn=3;
    let zm=9;
    println!("The order of ({a},{b}) of direct product z{zn}*z{zm} = {:?}",find_order_of_an_element_in_direct_product((a,b),zn,zm));
    

}
/*
consider two additive groups
Zn,Zm
if you got the direct product of ZnZm
find the direct products of a certain element say (a,b) mod n,m respectively

Zn=3
Zm=9
(1,1)---->find how many times we need to add 1 to get the identity 0
*/
// 1,2,3
fn find_order_of_an_element_in_direct_product((a,b):(u32,u32),n:u32,m:u32)->u32{
    let order_a=find_order_of_individual_element(a,n);
    let order_b=find_order_of_individual_element(b,m);
   lcm(order_a,order_b)

}

fn find_order_of_individual_element( a:u32,modulo:u32)->u32{
    let mut counter=1;
    let mut res=a%modulo;
    while res!=0{
        res=(res+a)%modulo;
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