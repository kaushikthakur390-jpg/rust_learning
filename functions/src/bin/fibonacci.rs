fn main()
{
    let n = 10 ;
    println!("fibonacci({}) = {}",n,fibonacci(n));
}
fn fibonacci(n:u32)->u64 
{
    if n == 0 
    {return 0;}
    let mut a = 0;
    let mut b = 1;
    for number in 1..n 
    {
        let next = a + b ;
        a = b;
        b = next ;
    }
    b
}


