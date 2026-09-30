fn main()
{
    let mut s = String :: from("hello");
    s.push_str(" there");//push_str is used to append a literal to strings
    println!("{}",s);
}