    fn main() {
    
        let modulo=8;
        
    for i in 1..modulo{
        let res=order_of_an_element(i,modulo);
        match res{
            Some(res)=>println!("The order of {} mod {} ={:?}",i,modulo,res),
            None=>(),
        }
    }
        
        
    }
    fn gcd(mut a:i32,mut b:i32)->i32{
        while b != 0{
            let temp=b;
            b=a%b;
            a=temp;

        }
        a
    }

    fn order_of_an_element(a:i32,modulo:i32)->Option<i32>{
    
        let mut count=1;
        let mut res=a%modulo;
        if gcd(a,modulo) !=1{
            return None;
        }
       
        while res != 1{
            res=(res*a)%modulo;
           
            count+=1
        }
        Some(count)

    }