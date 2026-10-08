fn main()
{
    let string = String :: from("hello world");
    let word = first_word(&string[0..6]);
    let word = first_word(&string[..]);
let word = first_word(&string);
let string_literal = "hello world";
let word = first_word(&string_literal[0..6]);
let word = first_word(&string_literal[..]);
let word = first_word(&string_literal);
}
fn first_word (s: &str) -> &str {
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }
    &s[..]
}