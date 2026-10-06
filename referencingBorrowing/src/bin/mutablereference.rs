fn main ()
{
    let mut s = String :: from("hello");
    let s1 = change(&mut s);
    println!("borrowed value after change {s1}");
    println!("actual value s1 references too {s}");// the variable s1 has borrowed the reference of s which holds the main value so when s1 is changed its giving permission to change/append the value of s since it references to that variable value
}
fn change(string : &mut String) -> &mut String
{
    string.push_str("world");
    string
}