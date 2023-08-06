use elfinfo::elf;
use eyre::Result;
use nom::{error::VerboseErrorKind, Err};
use owo_colors::OwoColorize;
use std::{env, fs, sync::OnceLock};

static DATA: OnceLock<Vec<u8>> = OnceLock::new();

fn main() -> Result<()> {
    color_eyre::install()?;

    let _ = DATA.set(
        fs::read(env::args().nth(1).expect("please provide a path!"))
            .expect("failed to read file at provided path!"),
    );

    let header = match elf::Header::parser()(DATA.get().unwrap()) {
        Ok((_, header)) => header,
        Err(Err::Error(err)) | Err(Err::Failure(err)) => {
            eprint!("{}", "error: ".red());

            for (_, err) in err.errors.iter().rev() {
                match err {
                    VerboseErrorKind::Context(cx) => eprint!("{} -> ", cx.yellow()),
                    VerboseErrorKind::Char(c) => eprint!("{} '{}'", "expected".yellow(), c.green()),
                    VerboseErrorKind::Nom(ek) => {
                        eprint!("{}{:?}", "nom error: ".red(), ek.red())
                    }
                }
            }

            eprintln!();
            panic!("parsing failed!");
        }
        Err(_) => panic!("parsing failed: unknown error!"),
    };

    println!("{header:#?}");

    Ok(())
}
