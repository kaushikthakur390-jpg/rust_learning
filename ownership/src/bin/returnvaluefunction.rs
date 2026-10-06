fn main()
{
    let s1 = give_ownership();
    let s2 = String :: from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
}
fn give_ownership()->String
{
    let string = String :: from("yours");
    string
}
fn takes_and_gives_back(some_string:String)-> String
{
    some_string
}
  
