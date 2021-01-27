use nom::IResult;
use nom::bytes::complete::{is_a, is_not};
use nom::character::is_digit;
use nom::bytes::complete::tag;
use nom::named;
use nom::map;

#[derive(Debug, PartialEq)]
enum Token {
    Semicolon,
    Space,
    Comma,
    Equal,
    ParenthesisLeft,
    ParenthesisRight,
    Number(f64),
    Identifier(String),
}

/*
type TokenStream = Vec<Token>;

// vrací tuple, první hodnota je zbytek stringu, druhá hodnota je ta naparsovaná
// https://docs.rs/nom/6.1.0/nom/combinator/fn.iterator.html

pub fn is_char_digit(chr: char) -> bool {
    return chr.is_ascii() && is_digit(chr as u8)
}

fn match_space(input: &str) -> Option<(Token, &str)>
{
    let res: IResult<&str, &str> = is_a(" \t\r\n")(input);

    if res.is_err() {
        return None;
    }

    let (rest_of_string, _) = res.unwrap();
    let token = Token::Space;

    Some((token, rest_of_string))
}

fn match_comma(input: &str) -> Option<(Token, &str)>
{
    let res: IResult<&str, &str> = is_a(",")(input);

    if res.is_err() {
        return None;
    }

    let (rest_of_string, _) = res.unwrap();
    let token = Token::Comma;

    Some((token, rest_of_string))
}

fn match_equal(input: &str) -> Option<(Token, &str)>
{
    let res: IResult<&str, &str> = is_a("=")(input);

    if res.is_err() {
        return None;
    }

    let (rest_of_string, _) = res.unwrap();
    let token = Token::Equal;

    Some((token, rest_of_string))
}

fn match_parenthesis_left(input: &str) -> Option<(Token, &str)>
{
    let res: IResult<&str, &str> = is_a("(")(input);

    if res.is_err() {
        return None;
    }

    let (rest_of_string, _) = res.unwrap();
    let token = Token::ParenthesisLeft;

    Some((token, rest_of_string))
}

fn match_parenthesis_right(input: &str) -> Option<(Token, &str)>
{
    let res: IResult<&str, &str> = is_a(")")(input);

    if res.is_err() {
        return None;
    }

    let (rest_of_string, _) = res.unwrap();
    let token = Token::ParenthesisRight;

    Some((token, rest_of_string))
}

fn match_semicolon(input: &str) -> Option<(Token, &str)>
{
    let res: IResult<&str, &str> = is_a(";")(input);

    if res.is_err() {
        return None;
    }

    let (rest_of_string, _) = res.unwrap();
    let token = Token::Semicolon;

    Some((token, rest_of_string))
}

fn match_identifier(input: &str) -> Option<(Token, &str)>
{
    let res: IResult<&str, &str> = is_not(" \t\r\n=,();")(input);

    if res.is_err() {
        return None;
    }

    let (rest_of_string, value) = res.unwrap();
    let token = Token::Identifier(value.to_string());

    Some((token, rest_of_string))
}

fn match_anything(input: &str) -> Option<(Token, &str)>  {

    let mut res = match_space(&input);
    if res.is_some() { return res; }
    
    res = match_comma(&input);
    if res.is_some() { return res; }

    res = match_semicolon(&input);
    if res.is_some() { return res; }

    res = match_parenthesis_left(&input);
    if res.is_some() { return res; }

    res = match_parenthesis_right(&input);
    if res.is_some() { return res; }

    res = match_equal(&input);
    if res.is_some() { return res; }

    res = match_identifier(&input);
    if res.is_some() { return res; }

    None
}

fn lexer(input: &str) -> TokenStream {

    let mut input = input;

    let mut stream = Vec::new();
    
    loop {

        if input.is_empty() {
            break;
        }

        let res = match_anything(&input);

        if res.is_none() {
            break;
        }

        let (token, rest_of_string) = res.unwrap();

        input = rest_of_string;

        if token != Token::Space {
            stream.push(token);
        }
    }

    stream
}
*/

fn main() {
    
    // IDENTIFIER(line) IDENTIFIER(x) EQUAL IDENTIFIER(p) PARENTHESES_START LITERAL(0) ...
    let input = "line x=p(0, 10) y=(25, 50) color=black;";
    //let lexeme = lexer(input);
    //println!("{:?}", lexeme);

    

    named!(match_space<&str, Token>, map!(is_a(" \t\r\n"), |_| Token::Space));
    named!(match_comma<&str, Token>, map!(tag(","), |_| Token::Comma));
    named!(match_equal<&str, Token>, map!(tag("="), |_| Token::Equal));
    named!(match_parenthesis_left<&str, Token>, map!(tag("("), |_| Token::ParenthesisLeft));
    named!(match_parenthesis_right<&str, Token>, map!(tag(")"), |_| Token::ParenthesisRight));
    named!(match_semicolon<&str, Token>, map!(tag(";"), |_| Token::Semicolon));
    named!(match_identifier<&str, Token>, map!(is_not(" \t\r\n=,();"), |val| Token::Identifier(val.to_string())));


    use nom::branch::alt;
    use nom::combinator::all_consuming;

    let mut match_anything = alt((
        match_space,
        match_comma,
        match_equal,
        match_parenthesis_left,
        match_parenthesis_right,
        match_semicolon,
        match_identifier
    ));

    let res = match_anything(input);

    println!("{:?}", res);
}
