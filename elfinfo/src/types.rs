macro_rules! elfn_type {
    ($name:ident, $inner_debug_fmt:literal) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(pub u64);

        impl $name {
            pub fn parser(
                class: crate::elf::Class,
                encoding: crate::elf::Encoding,
            ) -> impl Fn(crate::ParseInput) -> crate::ParseResult<Self> {
                move |i| match nom::number::Endianness::try_from(encoding) {
                    Ok(endianness) => match class {
                        crate::elf::Class::None | crate::elf::Class::Num => Err(nom::Err::Error(
                            nom::error::make_error(i, nom::error::ErrorKind::Tag),
                        )),
                        crate::elf::Class::Elf32 => {
                            let (i, val) = nom::error::context(
                                stringify!($name),
                                nom::number::streaming::u32(endianness),
                            )(i)?;
                            Ok((i, $name(val.into())))
                        }
                        crate::elf::Class::Elf64 => {
                            let (i, val) = nom::error::context(
                                stringify!($name),
                                nom::number::streaming::u64(endianness),
                            )(i)?;
                            Ok((i, $name(val)))
                        }
                    },
                    Err(_) => Err(nom::Err::Error(nom::error::make_error(
                        i,
                        nom::error::ErrorKind::Tag,
                    ))),
                }
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $inner_debug_fmt, self.0)
            }
        }
    };
}

elfn_type!(Address, "{:#x}");
elfn_type!(Offset, "{:#x}");
elfn_type!(Size, "{}");

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Flags(pub u32);

impl Flags {
    pub fn parser(
        encoding: crate::elf::Encoding,
    ) -> impl Fn(crate::ParseInput) -> crate::ParseResult<Self> {
        move |i| match nom::number::Endianness::try_from(encoding) {
            Ok(endianness) => {
                let (i, val) =
                    nom::error::context("Flags", nom::number::streaming::u32(endianness))(i)?;
                Ok((i, Flags(val)))
            }
            Err(_) => Err(nom::Err::Error(nom::error::make_error(
                i,
                nom::error::ErrorKind::Tag,
            ))),
        }
    }
}

impl std::fmt::Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#b}", self.0)
    }
}
