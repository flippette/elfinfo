use crate::{elf::Encoding, ParseInput, ParseResult};
use nom::{
    error::{make_error, ErrorKind},
    number::Endianness,
    Err,
};

pub fn encoding_to_endianness<'a, T, F: Fn(ParseInput<'a>) -> ParseResult<'a, T>>(
    parser: impl Fn(Endianness) -> F,
    encoding: Encoding,
) -> impl Fn(ParseInput<'a>) -> ParseResult<'a, T> {
    move |i| match Endianness::try_from(encoding) {
        Ok(endianness) => {
            let (i, val) = parser(endianness)(i)?;
            Ok((i, val))
        }
        _ => Err(Err::Error(make_error(i, ErrorKind::Tag))),
    }
}
