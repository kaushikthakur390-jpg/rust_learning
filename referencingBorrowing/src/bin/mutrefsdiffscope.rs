//you can have multiple mutable refs as long as they are not within same scope
fn main()
{
    let mut s = String :: from ("hello");
    {
        let s1 = &mut s ;
            println!("{s1}");
    }//s1 is out of scope here now 
    let s2 = &mut s ;
    println!("{s2}");
}