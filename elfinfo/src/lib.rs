pub mod elf;
pub mod types;
mod util;

pub(crate) type ParseInput<'a> = &'a [u8];
pub(crate) type ParseResult<'a, O> =
    nom::IResult<ParseInput<'a>, O, nom::error::VerboseError<ParseInput<'a>>>;
