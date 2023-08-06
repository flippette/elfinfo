use crate::{
    types::{Address, Flags, Offset},
    util, ParseInput, ParseResult,
};
use derive_try_from_primitive::TryFromPrimitive;
use nom::{
    bytes::streaming::{tag, take},
    error::context,
    number::{
        streaming::{u16, u8},
        Endianness,
    },
};
use std::mem;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Header {
    pub ident: Identifier,
    pub ty: Type,
    pub machine: Machine,
    pub version: u16,
    pub entry: Address,
    pub pht_offset: Offset,
    pub sht_offset: Offset,
    pub flags: Flags,
    pub header_size: u16,
    pub pht_entry_size: u16,
    pub pht_entry_num: u16,
    pub sht_entry_size: u16,
    pub sht_entry_num: u16,
    pub sht_section_name_index: u16,
}

impl Header {
    pub fn parser() -> impl Fn(ParseInput) -> ParseResult<Self> {
        move |i| {
            let (i, ident) = Identifier::parser()(i)?;
            let (i, ty) = Type::parser(ident.encoding)(i)?;
            let (i, machine) = Machine::parser(ident.encoding)(i)?;
            let (i, version) = context(
                "<version>",
                util::encoding_to_endianness(u16, ident.encoding),
            )(i)?;
            let (i, entry) = Address::parser(ident.class, ident.encoding)(i)?;
            let (i, pht_offset) = Offset::parser(ident.class, ident.encoding)(i)?;
            let (i, sht_offset) = Offset::parser(ident.class, ident.encoding)(i)?;
            let (i, flags) = Flags::parser(ident.encoding)(i)?;
            let (i, header_size) = context(
                "<header_size>",
                util::encoding_to_endianness(u16, ident.encoding),
            )(i)?;
            let (i, pht_entry_size) = context(
                "<pht_entry_size>",
                util::encoding_to_endianness(u16, ident.encoding),
            )(i)?;
            let (i, pht_entry_num) = context(
                "<pht_entry_num>",
                util::encoding_to_endianness(u16, ident.encoding),
            )(i)?;
            let (i, sht_entry_size) = context(
                "<sht_entry_size>",
                util::encoding_to_endianness(u16, ident.encoding),
            )(i)?;
            let (i, sht_entry_num) = context(
                "<sht_entry_num>",
                util::encoding_to_endianness(u16, ident.encoding),
            )(i)?;
            let (i, sht_section_name_index) = context(
                "<sht_entry_num>",
                util::encoding_to_endianness(u16, ident.encoding),
            )(i)?;

            Ok((
                i,
                Self {
                    ident,
                    ty,
                    machine,
                    version,
                    entry,
                    pht_offset,
                    sht_offset,
                    flags,
                    header_size,
                    pht_entry_size,
                    pht_entry_num,
                    sht_entry_size,
                    sht_entry_num,
                    sht_section_name_index,
                },
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    // magic: [u8; 4],
    pub class: Class,
    pub encoding: Encoding,
    pub version: u8,
    pub abi: Abi,
    pub abi_version: u8,
    // padding: [u8; 16 - <size of the other fields>],
}

impl Identifier {
    pub fn parser() -> impl Fn(ParseInput) -> ParseResult<Self> {
        move |i| {
            let (i, _magic) = context("<magic>", tag(b"\x7fELF"))(i)?;
            let (i, class) = Class::parser()(i)?;
            let (i, encoding) = Encoding::parser()(i)?;
            let (i, version) = u8(i)?;
            let (i, abi) = Abi::parser()(i)?;
            let (i, abi_version) = context("<abi_version>", u8)(i)?;
            let (i, _padding) = context(
                "<padding>",
                take(
                    16 // EI_NIDENT
                    - _magic.len()
                    - mem::size_of::<Class>()
                    - mem::size_of::<Encoding>()
                    - mem::size_of::<u8>()
                    - mem::size_of::<Abi>()
                    - mem::size_of::<u8>(),
                ),
            )(i)?;
            Ok((
                i,
                Self {
                    class,
                    encoding,
                    version,
                    abi,
                    abi_version,
                },
            ))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
#[repr(u8)]
pub enum Class {
    None = 0,
    Elf32 = 1,
    Elf64 = 2,
    Num = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
#[repr(u8)]
pub enum Encoding {
    None = 0,
    Lsb = 1,
    Msb = 2,
    Num = 3,
}

#[derive(Debug, Error)]
pub enum EncodingError {
    #[error("unsupported encoding!")]
    NomUnsupported,
}

impl TryFrom<Encoding> for Endianness {
    type Error = EncodingError;

    fn try_from(enc: Encoding) -> Result<Self, Self::Error> {
        match enc {
            Encoding::None | Encoding::Num => Err(EncodingError::NomUnsupported),
            Encoding::Lsb => Ok(Endianness::Little),
            Encoding::Msb => Ok(Endianness::Big),
        }
    }
}

#[allow(non_camel_case_types)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
#[repr(u8)]
pub enum Abi {
    SysV = 0,
    HPUX = 1,
    NetBSD = 2,
    GNU = 3,
    Solaris = 6,
    AIX = 7,
    Irix = 8,
    FreeBSD = 9,
    TRU64 = 10,
    Modesto = 11,
    OpenBSD = 12,
    ARM_EABI = 64,
    ARM = 97,
    Standalone = 255,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
#[repr(u16)]
pub enum Type {
    None = 0,
    Rel = 1,
    Exec = 2,
    Dyn = 3,
    Core = 4,
    Num = 5,
    LoOS = 0xfe00,
    HiOS = 0xfeff,
    LoProc = 0xff00,
    HiProc = 0xffff,
}

#[allow(non_camel_case_types)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
#[repr(u16)]
pub enum Machine {
    None = 0,
    M32 = 1,
    SPARC = 2,
    i386 = 3,
    m68k = 4,
    m88k = 5,
    IAMCU = 6,
    i860 = 7,
    MIPS = 8,
    S370 = 9,
    MIPS_RS3_LE = 10,
    PARISC = 15,
    VPP500 = 17,
    SPARC32plus = 18,
    i960 = 19,
    PowerPC = 20,
    PowerPC64 = 21,
    S390 = 22,
    SPU = 23,
    V800 = 36,
    FR20 = 37,
    RH32 = 38,
    RCE = 39,
    ARM = 40,
    FAKE_ALPHA = 41,
    SH = 42,
    SPARCv9 = 43,
    Tricore = 44,
    ARC = 45,
    H8_300 = 46,
    H8_300H = 47,
    H8S = 48,
    H8_500 = 49,
    IA64 = 50,
    MIPSX = 51,
    Coldfire = 52,
    M68HC12 = 53,
    MMA = 54,
    PCP = 55,
    nCPU = 56,
    NDF1 = 57,
    StartCore = 58,
    ME16 = 59,
    ST100 = 60,
    Tinyj = 61,
    x86_64 = 62,
    DSP = 63,
    PDP10 = 64,
    PDP11 = 65,
    FX66 = 66,
    ST9Plus = 67,
    ST7 = 68,
    M68HC16 = 69,
    M68HC11 = 70,
    M68HC08 = 71,
    M68HC05 = 72,
    SVx = 73,
    ST19 = 74,
    VAX = 75,
    Cris = 76,
    Javelin = 77,
    Firepath = 78,
    ZSP = 79,
    MMIX = 80,
    HUAny = 81,
    Prism = 82,
    AVR = 83,
    FR30 = 84,
    D10V = 85,
    D30V = 86,
    v850 = 87,
    M32R = 88,
    MN10300 = 89,
    MN10200 = 90,
    picoJava = 91,
    OpenRISC = 92,
    ARCompact = 93,
    Xtensa = 94,
    VideoCode = 95,
    TMMGPP = 96,
    NS32K = 97,
    TPC = 98,
    SNP1K = 99,
    ST200 = 100,
    IP2K = 101,
    MAX = 102,
    CompactRISC = 103,
    F2MC16 = 104,
    msp430 = 105,
    Blackfin = 106,
    S1C33 = 107,
    SEP = 108,
    ArcaRISC = 109,
    Unicore = 110,
    eXcess = 111,
    DXP = 112,
    AlteraNiosII = 113,
    CRX = 114,
    XGATE = 115,
    C166 = 116,
    M16C = 117,
    dsPIC30F = 118,
    CE = 119,
    M32C = 120,
    TSK3000 = 131,
    RS08 = 132,
    SHARC = 133,
    eCOG2 = 134,
    Score7 = 135,
    DSP24 = 136,
    VideoCoreIII = 137,
    LatticeMICO32 = 138,
    C17 = 139,
    TMS320C6000 = 140,
    TMS320C2000 = 141,
    TMS320C55x = 142,
    TI_ARP32 = 143,
    TI_PRU = 144,
    MMDSPPlus = 160,
    CypressM8C = 161,
    R32C = 162,
    TriMedia = 163,
    QDSP6 = 164,
    i8051 = 165,
    STxP7x = 166,
    NDS32 = 167,
    eCOG1X = 168,
    MAXQ30 = 169,
    XIMO16 = 170,
    M2000 = 171,
    CrayNV2 = 172,
    RX = 173,
    METAg = 174,
    MCST_Elbrus = 175,
    eCOG16 = 176,
    CR16 = 177,
    ETPU = 178,
    SLE9X = 179,
    L10M = 180,
    K10M = 181,
    AArch64 = 183,
    AVR32 = 185,
    STM8 = 186,
    TILE64 = 187,
    TILEPro = 188,
    MicroBlaze = 189,
    CUDA = 190,
    TILEGx = 191,
    CloudShield = 192,
    COREA_1st = 193,
    COREA_2nd = 194,
    ARCv2 = 195,
    Open8 = 196,
    RL78 = 197,
    VideoCoreV = 198,
    R78KOR = 199,
    F56800EX = 200,
    BA1 = 201,
    BA2 = 202,
    xCORE = 203,
    MchpPIC = 204,
    iGT = 205,
    KM32 = 210,
    KMX32 = 211,
    KMX16 = 212,
    KMX8 = 213,
    KVARC = 214,
    CDP = 215,
    COGE = 216,
    CoolEngine = 217,
    NORC = 218,
    CSR_Kalimba = 219,
    Z80 = 220,
    VISIUMcore = 221,
    FT32 = 222,
    Moxie = 223,
    AMDGPU = 224,
    RISCV = 243,
    BPF = 247,
    CSKY = 252,
    LoongArch = 258,
    Num = 259,
    Alpha = 0x9026,
}

macro_rules! impl_enum_parser {
    ($name:ident, u8) => {
        impl $name {
            pub fn parser() -> impl Fn(crate::ParseInput) -> crate::ParseResult<Self> {
                move |i| {
                    let (i, val) = nom::error::context(
                        stringify!($name),
                        nom::combinator::map_res(nom::number::streaming::u8, $name::try_from),
                    )(i)?;
                    Ok((i, val))
                }
            }
        }
    };
    ($name:ident, $nom_parser:ident) => {
        impl $name {
            pub fn parser(
                encoding: crate::elf::Encoding,
            ) -> impl Fn(crate::ParseInput) -> crate::ParseResult<Self> {
                move |i| {
                    let (i, val) = nom::error::context(
                        stringify!($name),
                        nom::combinator::map_res(
                            nom::number::streaming::$nom_parser(encoding.try_into().map_err(
                                |_| {
                                    nom::Err::Error(nom::error::make_error(
                                        i,
                                        nom::error::ErrorKind::Tag,
                                    ))
                                },
                            )?),
                            $name::try_from,
                        ),
                    )(i)?;
                    Ok((i, val))
                }
            }
        }
    };
}

impl_enum_parser!(Class, u8);
impl_enum_parser!(Encoding, u8);
impl_enum_parser!(Abi, u8);
impl_enum_parser!(Type, u16);
impl_enum_parser!(Machine, u16);
