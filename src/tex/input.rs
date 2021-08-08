use crate::datastructures::scopedmap::ScopedMap;
use crate::tex::token::catcode::RawCatCode;
use crate::tex::token::lexer;
use crate::tex::token::stream;
use crate::tex::token::token;
use std::fs;
use std::io;

// TODO: this implementation seems....completely wrong?
// Like, why does the inital input file get special treatment, whereas
//   subsequent files imported via \input are handled differently?
// Also how do we handle MULTIPLE sequential input files i.e., plain.tex
pub struct InputModule {
    pub cat_code_map: ScopedMap<char, RawCatCode>,
    lexer: Option<lexer::Lexer<io::BufReader<fs::File>>>,
    next_token: Option<token::Token>,
}

impl InputModule {
    pub fn new(cat_code_map: ScopedMap<char, RawCatCode>) -> InputModule {
        InputModule {
            cat_code_map,
            lexer: None,
            next_token: None,
        }
    }

    pub fn open_file(&mut self, file_name: &str) -> anyhow::Result<()> {
        let f = io::BufReader::new(fs::File::open(file_name)?);
        self.lexer = Some(lexer::Lexer::new(f));
        Ok(())
    }
}

impl stream::Stream for InputModule {
    fn next(&mut self) -> anyhow::Result<Option<token::Token>> {
        self.prepare_imut_peek()?;
        Ok(self.next_token.take())
    }

    fn prepare_imut_peek(&mut self) -> anyhow::Result<()> {
        if self.next_token == None {
            if let Some(lexer) = self.lexer.as_mut() {
                self.next_token = lexer.next(&self.cat_code_map)?;
            }
        }
        Ok(())
    }

    fn imut_peek(&self) -> anyhow::Result<Option<&token::Token>> {
        Ok(self.next_token.as_ref())
    }
}
