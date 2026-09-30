fn main ()
{
    let mut s = String :: from("hello");
    s.push_str(" there");
    println!("{}",s);
    println!("s="scopeout(s));
}

    fn scopeout()
    {
        println!("{}",s);
    }
