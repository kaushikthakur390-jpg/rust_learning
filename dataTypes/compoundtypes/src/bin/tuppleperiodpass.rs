fn main()
{
    let tup :(i32,f64,u32) = (500,6.4,1);
    let five_hundred = tup.0; //can be referenced to by using period(.) followed by the index of the tupple the index always starts with 0 
    let six_point_four = tup.1;
    let one = tup.2;
println!("the values of tupple are {five_hundred},{six_point_four},{one}");
}