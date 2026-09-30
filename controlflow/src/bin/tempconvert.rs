fn main()
{
let celsius = 25.0;
let farenheit = tempconvert(celsius);
println!("{}c = {}f",celsius,farenheit)
}
fn tempconvert(n:f64)->f64
{
    (n*9.0/5.0) + 32.0 
}