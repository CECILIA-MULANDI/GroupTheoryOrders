fn main() {
    let n=8;
    let m=7;
    let a=1;
    let b=3;
    let c=1;
    let d=4;
    println!("The elements in the group {} are:{:?}",n,get_elements_of_a_group(n));
    println!("The elements in the group {} are:{:?}",m,get_elements_of_a_group(m));
    println!("\nThe elements of the direct product z{}.z{} are:{:?}",n,m,get_direct_product_of_two_groups(n,m));
    println!("The inverse of ({},{}) is:{:?}",a,b,get_inverse((a,b),n,m));
    println!("The sum of ({},{}),({},{}) is:{:?}",a,b,c,d,sum_mod((a,b),(c,d),n,m));
}
fn get_elements_of_a_group(n:u32)->Vec<u32>{
    (0..n).collect()
}
fn get_direct_product_of_two_groups(n:u32,m:u32)->Vec<(u32,u32)>{
    let zn_elements=get_elements_of_a_group(n);
    let zm_elements=get_elements_of_a_group(m);
    let mut znzm_elements=Vec::new();
    for i in &zn_elements{
        for j in &zm_elements{
           
            znzm_elements.push((*i,*j));
           
        }
    }
    znzm_elements
}

fn get_inverse((a,b):(u32,u32),n:u32,m:u32)->(u32,u32){
    ((n-a)%n,(m-b)%m)
}
/*
GIVEN:
(1,3)(1,4) find the sum

 */
fn sum_mod((a,b):(u32,u32),(c,d):(u32,u32),n:u32,m:u32)->(u32,u32){
    ((a+c)%n,(b+d)%m)
}