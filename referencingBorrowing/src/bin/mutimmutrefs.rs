//you cannot assign mutable and immutable refs both to the same value
fn main()
{
    let mut s = String ::from ("hello");
    let s1 = &s; /no issue
    let s2 = &s; / no issue
    let s3 = &mut s; /big issue
        println!("{s1},{s2},{s3}")
}