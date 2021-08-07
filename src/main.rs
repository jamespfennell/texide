use std::fs::File;
use std::io::BufReader;
use texide::tex::primitives;
use texide::tex::token::catcode;
use texide::tex::token::lexer;

pub fn run() -> Result<(), lexer::LexerError> {
    let f = BufReader::new(File::open("foo.tex")?);
    // let mut reader = CharReader::new(f);
    let mut lexer = lexer::Lexer::new(f);
    let map = catcode::tex_defaults();

    while let Some(t) = lexer.next(&map)? {
        println!("Token: {:?}", t)
    }
    Ok(())
}

fn main() {
    primitives::expand();
    run();

    /*
    let err = lexer::TokenError{
        token: token::Token::Character(),
        message: "".to_string(),
        noted: vec![]
    }
     */
    //let r = run();
    //if let Err(s) = r {
    //    println!("{:?}", s);
    //}
    /*
    let mut map = ScopedMap::new();
    map.insert(1, 3);
    println!("{:?}", map.get(&1));
    println!("{:?}", map.get(&2));
    map.begin_scope();
    map.insert(1, 2);
    map.insert(2, 5);
    println!("{:?}", map.get(&1));
    println!("{:?}", map.get(&2));
    assert!(map.end_scope());
    println!("{:?}", map.get(&1));
    println!("{:?}", map.get(&2));
    println!("Hello, world!");

     */
}
