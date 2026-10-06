//if one mutable reference for a value exists already you cannot give another mutable reference to it 
fn main ()
{
    let mut s = String :: from("hello");
    let s1 = &mut s ;
    let s2 = &mut s ;
    println!("{s1},{s2}");
}
