use std::env;

use std::process;
use texide::tex::driver;

use std::rc;
use texide::tex::primitive::library::conditional;
use texide::tex::state;
use texide::tex::state::TexState;
use texide::tex::token::catcode;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => {
            println!("Pass the tex file as an argument");
            process::exit(1);
        }
        Some(file_name) => {
            let r = run(file_name);
            if let Some(err) = r.err() {
                println!("Failed: {}", err);
                process::exit(1);
            }
        }
    };
}

macro_rules! set_e {
    ($state: expr, $name: expr, $p: expr) => {
        &mut $state.set_expansion_primitive($name.to_string(), rc::Rc::new($p))
    };
}

pub fn run(file_name: &str) -> Result<(), anyhow::Error> {
    let mut s = state::SimpleState::new();
    set_e![s, "if", conditional::get_if()];
    set_e![s, "else", conditional::get_else()];
    let input_module = &mut s.base_mut().input_module;
    catcode::set_tex_defaults(&mut input_module.cat_code_map);
    input_module.open_file(file_name)?;
    driver::run(s)?;
    Ok(())
}
