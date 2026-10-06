fn main()
{
    let mut s = String :: from ("hello");
    let s1 = &s ;
    let s2 = &s;
    println!("{s1},{s2}");//since the immutable referenced values were used here they are out of the scope now 
    let s3 = &mut s ; // hence s3 could be assigned a mutable ref to s
    println!("{s3}");
    
}