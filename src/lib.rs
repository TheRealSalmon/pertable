//! Periodic table in Rust
//!
//! Provides an Element enum which has a few utility functions that are useful
//! for general cheminformatics programming. Elements can be created either from
//! the atomic number or the atomic symbol using the TryFrom<u8> and FromStr
//! respectively.
//!
//! This library provides a few utility functions useful for cheminformatics:
//! - `atomic_number`
//! - `atomic_symbol`
//! - `atomic_weight` (WIP!)
//! - `n_valence_electrons` (for SMILES parsing/perception, not for general use)
//! - `valence` (for SMILES parsing/perception, not for general use)
//!
//! This library has its own `Error` enum with the following variants:
//! - `InvalidAtomicNumber`
//! - `InvalidAtomicSymbol`
//! - `InvalidIsotope`
//! - `InvalidFormalCharge`
//!
//! Here's some example code:
//! ```rust
//! use pertable::Element;
//!
//! let element = Element::C;
//! assert_eq!(element.atomic_number(), 6);
//! assert_eq!(element.atomic_symbol(), "C".to_owned());
//! assert_eq!(element.atomic_weight(None).unwrap(), 12.0106);
//! assert_eq!(element.n_valence_electrons(0).unwrap(), 4);
//! assert_eq!(element.valence(0).unwrap(), 4);
//! ```

use std::fmt::Display;
use std::str::FromStr;

/// Error enum for pertable.
///
/// Possible variants are:
/// - `InvalidAtomicNumber`
///     - Invoked when creating an element with atomic_number > 118
/// - `InvalidAtomicSymbol`
///     - Invoked when creating an element with an invalid atomic_symbol
/// - `InvalidIsotope`
///     - Invoked when querying atomic_weight of an unknown isotope
/// - `InvalidFormalCharge`
///     - Invoked when querying n_valence_electrons but n_valence_electrons < 0 or > 8
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidAtomicNumber(u8),
    InvalidAtomicSymbol(String),
    InvalidIsotope(String, u16),
    InvalidFormalCharge(String, i8),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidAtomicNumber(atomic_number) => {
                write!(f, "invalid atomic number {atomic_number}")
            }
            Error::InvalidAtomicSymbol(atomic_symbol) => {
                write!(f, "invalid atomic symbol {atomic_symbol}")
            }
            Error::InvalidIsotope(atomic_symbol, isotope) => {
                write!(f, "invalid isotope {isotope} for {atomic_symbol}")
            }
            Error::InvalidFormalCharge(atomic_symbol, formal_charge) => {
                write!(
                    f,
                    "invalid formal charge {formal_charge} for {atomic_symbol}"
                )
            }
        }
    }
}

impl std::error::Error for Error {}

/// Element enum for the periodic table of elements.
/// 
/// "Class methods" include:
/// - `atomic_number`
/// - `atomic_symbol`
/// - `atomic_weight`
/// - `n_valence_electrons`
/// - `valence`
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Element {
    #[default]
    Any,
    H ,                                                                 He,
    Li, Be,                                         B , C , N , O , F , Ne,
    Na, Mg,                                         Al, Si, P , S , Cl, Ar,
    K , Ca, Sc, Ti, V , Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr,
    Rb, Sr, Y , Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, I , Xe,
    Cs, Ba,     Hf, Ta, W , Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn,
    Fr, Ra,     Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,

            La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu,
            Ac, Th, Pa, U , Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr,
}

impl TryFrom<u8> for Element {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Element::Any),
            1 => Ok(Element::H),
            2 => Ok(Element::He),
            3 => Ok(Element::Li),
            4 => Ok(Element::Be),
            5 => Ok(Element::B),
            6 => Ok(Element::C),
            7 => Ok(Element::N),
            8 => Ok(Element::O),
            9 => Ok(Element::F),
            10 => Ok(Element::Ne),
            11 => Ok(Element::Na),
            12 => Ok(Element::Mg),
            13 => Ok(Element::Al),
            14 => Ok(Element::Si),
            15 => Ok(Element::P),
            16 => Ok(Element::S),
            17 => Ok(Element::Cl),
            18 => Ok(Element::Ar),
            19 => Ok(Element::K),
            20 => Ok(Element::Ca),
            21 => Ok(Element::Sc),
            22 => Ok(Element::Ti),
            23 => Ok(Element::V),
            24 => Ok(Element::Cr),
            25 => Ok(Element::Mn),
            26 => Ok(Element::Fe),
            27 => Ok(Element::Co),
            28 => Ok(Element::Ni),
            29 => Ok(Element::Cu),
            30 => Ok(Element::Zn),
            31 => Ok(Element::Ga),
            32 => Ok(Element::Ge),
            33 => Ok(Element::As),
            34 => Ok(Element::Se),
            35 => Ok(Element::Br),
            36 => Ok(Element::Kr),
            37 => Ok(Element::Rb),
            38 => Ok(Element::Sr),
            39 => Ok(Element::Y),
            40 => Ok(Element::Zr),
            41 => Ok(Element::Nb),
            42 => Ok(Element::Mo),
            43 => Ok(Element::Tc),
            44 => Ok(Element::Ru),
            45 => Ok(Element::Rh),
            46 => Ok(Element::Pd),
            47 => Ok(Element::Ag),
            48 => Ok(Element::Cd),
            49 => Ok(Element::In),
            50 => Ok(Element::Sn),
            51 => Ok(Element::Sb),
            52 => Ok(Element::Te),
            53 => Ok(Element::I),
            54 => Ok(Element::Xe),
            55 => Ok(Element::Cs),
            56 => Ok(Element::Ba),
            57 => Ok(Element::La),
            58 => Ok(Element::Ce),
            59 => Ok(Element::Pr),
            60 => Ok(Element::Nd),
            61 => Ok(Element::Pm),
            62 => Ok(Element::Sm),
            63 => Ok(Element::Eu),
            64 => Ok(Element::Gd),
            65 => Ok(Element::Tb),
            66 => Ok(Element::Dy),
            67 => Ok(Element::Ho),
            68 => Ok(Element::Er),
            69 => Ok(Element::Tm),
            70 => Ok(Element::Yb),
            71 => Ok(Element::Lu),
            72 => Ok(Element::Hf),
            73 => Ok(Element::Ta),
            74 => Ok(Element::W),
            75 => Ok(Element::Re),
            76 => Ok(Element::Os),
            77 => Ok(Element::Ir),
            78 => Ok(Element::Pt),
            79 => Ok(Element::Au),
            80 => Ok(Element::Hg),
            81 => Ok(Element::Tl),
            82 => Ok(Element::Pb),
            83 => Ok(Element::Bi),
            84 => Ok(Element::Po),
            85 => Ok(Element::At),
            86 => Ok(Element::Rn),
            87 => Ok(Element::Fr),
            88 => Ok(Element::Ra),
            89 => Ok(Element::Ac),
            90 => Ok(Element::Th),
            91 => Ok(Element::Pa),
            92 => Ok(Element::U),
            93 => Ok(Element::Np),
            94 => Ok(Element::Pu),
            95 => Ok(Element::Am),
            96 => Ok(Element::Cm),
            97 => Ok(Element::Bk),
            98 => Ok(Element::Cf),
            99 => Ok(Element::Es),
            100 => Ok(Element::Fm),
            101 => Ok(Element::Md),
            102 => Ok(Element::No),
            103 => Ok(Element::Lr),
            104 => Ok(Element::Rf),
            105 => Ok(Element::Db),
            106 => Ok(Element::Sg),
            107 => Ok(Element::Bh),
            108 => Ok(Element::Hs),
            109 => Ok(Element::Mt),
            110 => Ok(Element::Ds),
            111 => Ok(Element::Rg),
            112 => Ok(Element::Cn),
            113 => Ok(Element::Nh),
            114 => Ok(Element::Fl),
            115 => Ok(Element::Mc),
            116 => Ok(Element::Lv),
            117 => Ok(Element::Ts),
            118 => Ok(Element::Og),
            _ => Err(Error::InvalidAtomicNumber(value)),
        }
    }
}

impl From<Element> for u8 {
    fn from(value: Element) -> Self {
        match value {
            Element::Any => 0,
            Element::H => 1,
            Element::He => 2,
            Element::Li => 3,
            Element::Be => 4,
            Element::B => 5,
            Element::C => 6,
            Element::N => 7,
            Element::O => 8,
            Element::F => 9,
            Element::Ne => 10,
            Element::Na => 11,
            Element::Mg => 12,
            Element::Al => 13,
            Element::Si => 14,
            Element::P => 15,
            Element::S => 16,
            Element::Cl => 17,
            Element::Ar => 18,
            Element::K => 19,
            Element::Ca => 20,
            Element::Sc => 21,
            Element::Ti => 22,
            Element::V => 23,
            Element::Cr => 24,
            Element::Mn => 25,
            Element::Fe => 26,
            Element::Co => 27,
            Element::Ni => 28,
            Element::Cu => 29,
            Element::Zn => 30,
            Element::Ga => 31,
            Element::Ge => 32,
            Element::As => 33,
            Element::Se => 34,
            Element::Br => 35,
            Element::Kr => 36,
            Element::Rb => 37,
            Element::Sr => 38,
            Element::Y => 39,
            Element::Zr => 40,
            Element::Nb => 41,
            Element::Mo => 42,
            Element::Tc => 43,
            Element::Ru => 44,
            Element::Rh => 45,
            Element::Pd => 46,
            Element::Ag => 47,
            Element::Cd => 48,
            Element::In => 49,
            Element::Sn => 50,
            Element::Sb => 51,
            Element::Te => 52,
            Element::I => 53,
            Element::Xe => 54,
            Element::Cs => 55,
            Element::Ba => 56,
            Element::La => 57,
            Element::Ce => 58,
            Element::Pr => 59,
            Element::Nd => 60,
            Element::Pm => 61,
            Element::Sm => 62,
            Element::Eu => 63,
            Element::Gd => 64,
            Element::Tb => 65,
            Element::Dy => 66,
            Element::Ho => 67,
            Element::Er => 68,
            Element::Tm => 69,
            Element::Yb => 70,
            Element::Lu => 71,
            Element::Hf => 72,
            Element::Ta => 73,
            Element::W => 74,
            Element::Re => 75,
            Element::Os => 76,
            Element::Ir => 77,
            Element::Pt => 78,
            Element::Au => 79,
            Element::Hg => 80,
            Element::Tl => 81,
            Element::Pb => 82,
            Element::Bi => 83,
            Element::Po => 84,
            Element::At => 85,
            Element::Rn => 86,
            Element::Fr => 87,
            Element::Ra => 88,
            Element::Ac => 89,
            Element::Th => 90,
            Element::Pa => 91,
            Element::U => 92,
            Element::Np => 93,
            Element::Pu => 94,
            Element::Am => 95,
            Element::Cm => 96,
            Element::Bk => 97,
            Element::Cf => 98,
            Element::Es => 99,
            Element::Fm => 100,
            Element::Md => 101,
            Element::No => 102,
            Element::Lr => 103,
            Element::Rf => 104,
            Element::Db => 105,
            Element::Sg => 106,
            Element::Bh => 107,
            Element::Hs => 108,
            Element::Mt => 109,
            Element::Ds => 110,
            Element::Rg => 111,
            Element::Cn => 112,
            Element::Nh => 113,
            Element::Fl => 114,
            Element::Mc => 115,
            Element::Lv => 116,
            Element::Ts => 117,
            Element::Og => 118,
        }
    }
}

impl From<&Element> for u8 {
    fn from(value: &Element) -> Self {
        match value {
            Element::Any => 0,
            Element::H => 1,
            Element::He => 2,
            Element::Li => 3,
            Element::Be => 4,
            Element::B => 5,
            Element::C => 6,
            Element::N => 7,
            Element::O => 8,
            Element::F => 9,
            Element::Ne => 10,
            Element::Na => 11,
            Element::Mg => 12,
            Element::Al => 13,
            Element::Si => 14,
            Element::P => 15,
            Element::S => 16,
            Element::Cl => 17,
            Element::Ar => 18,
            Element::K => 19,
            Element::Ca => 20,
            Element::Sc => 21,
            Element::Ti => 22,
            Element::V => 23,
            Element::Cr => 24,
            Element::Mn => 25,
            Element::Fe => 26,
            Element::Co => 27,
            Element::Ni => 28,
            Element::Cu => 29,
            Element::Zn => 30,
            Element::Ga => 31,
            Element::Ge => 32,
            Element::As => 33,
            Element::Se => 34,
            Element::Br => 35,
            Element::Kr => 36,
            Element::Rb => 37,
            Element::Sr => 38,
            Element::Y => 39,
            Element::Zr => 40,
            Element::Nb => 41,
            Element::Mo => 42,
            Element::Tc => 43,
            Element::Ru => 44,
            Element::Rh => 45,
            Element::Pd => 46,
            Element::Ag => 47,
            Element::Cd => 48,
            Element::In => 49,
            Element::Sn => 50,
            Element::Sb => 51,
            Element::Te => 52,
            Element::I => 53,
            Element::Xe => 54,
            Element::Cs => 55,
            Element::Ba => 56,
            Element::La => 57,
            Element::Ce => 58,
            Element::Pr => 59,
            Element::Nd => 60,
            Element::Pm => 61,
            Element::Sm => 62,
            Element::Eu => 63,
            Element::Gd => 64,
            Element::Tb => 65,
            Element::Dy => 66,
            Element::Ho => 67,
            Element::Er => 68,
            Element::Tm => 69,
            Element::Yb => 70,
            Element::Lu => 71,
            Element::Hf => 72,
            Element::Ta => 73,
            Element::W => 74,
            Element::Re => 75,
            Element::Os => 76,
            Element::Ir => 77,
            Element::Pt => 78,
            Element::Au => 79,
            Element::Hg => 80,
            Element::Tl => 81,
            Element::Pb => 82,
            Element::Bi => 83,
            Element::Po => 84,
            Element::At => 85,
            Element::Rn => 86,
            Element::Fr => 87,
            Element::Ra => 88,
            Element::Ac => 89,
            Element::Th => 90,
            Element::Pa => 91,
            Element::U => 92,
            Element::Np => 93,
            Element::Pu => 94,
            Element::Am => 95,
            Element::Cm => 96,
            Element::Bk => 97,
            Element::Cf => 98,
            Element::Es => 99,
            Element::Fm => 100,
            Element::Md => 101,
            Element::No => 102,
            Element::Lr => 103,
            Element::Rf => 104,
            Element::Db => 105,
            Element::Sg => 106,
            Element::Bh => 107,
            Element::Hs => 108,
            Element::Mt => 109,
            Element::Ds => 110,
            Element::Rg => 111,
            Element::Cn => 112,
            Element::Nh => 113,
            Element::Fl => 114,
            Element::Mc => 115,
            Element::Lv => 116,
            Element::Ts => 117,
            Element::Og => 118,
        }
    }
}

impl FromStr for Element {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "*" => Ok(Element::Any),
            "h" => Ok(Element::H),
            "he" => Ok(Element::He),
            "li" => Ok(Element::Li),
            "be" => Ok(Element::Be),
            "b" => Ok(Element::B),
            "c" => Ok(Element::C),
            "n" => Ok(Element::N),
            "o" => Ok(Element::O),
            "f" => Ok(Element::F),
            "ne" => Ok(Element::Ne),
            "na" => Ok(Element::Na),
            "mg" => Ok(Element::Mg),
            "al" => Ok(Element::Al),
            "si" => Ok(Element::Si),
            "p" => Ok(Element::P),
            "s" => Ok(Element::S),
            "cl" => Ok(Element::Cl),
            "ar" => Ok(Element::Ar),
            "k" => Ok(Element::K),
            "ca" => Ok(Element::Ca),
            "sc" => Ok(Element::Sc),
            "ti" => Ok(Element::Ti),
            "v" => Ok(Element::V),
            "cr" => Ok(Element::Cr),
            "mn" => Ok(Element::Mn),
            "fe" => Ok(Element::Fe),
            "co" => Ok(Element::Co),
            "ni" => Ok(Element::Ni),
            "cu" => Ok(Element::Cu),
            "zn" => Ok(Element::Zn),
            "ga" => Ok(Element::Ga),
            "ge" => Ok(Element::Ge),
            "as" => Ok(Element::As),
            "se" => Ok(Element::Se),
            "br" => Ok(Element::Br),
            "kr" => Ok(Element::Kr),
            "rb" => Ok(Element::Rb),
            "sr" => Ok(Element::Sr),
            "y" => Ok(Element::Y),
            "zr" => Ok(Element::Zr),
            "nb" => Ok(Element::Nb),
            "mo" => Ok(Element::Mo),
            "tc" => Ok(Element::Tc),
            "ru" => Ok(Element::Ru),
            "rh" => Ok(Element::Rh),
            "pd" => Ok(Element::Pd),
            "ag" => Ok(Element::Ag),
            "cd" => Ok(Element::Cd),
            "in" => Ok(Element::In),
            "sn" => Ok(Element::Sn),
            "sb" => Ok(Element::Sb),
            "te" => Ok(Element::Te),
            "i" => Ok(Element::I),
            "xe" => Ok(Element::Xe),
            "cs" => Ok(Element::Cs),
            "ba" => Ok(Element::Ba),
            "la" => Ok(Element::La),
            "ce" => Ok(Element::Ce),
            "pr" => Ok(Element::Pr),
            "nd" => Ok(Element::Nd),
            "pm" => Ok(Element::Pm),
            "sm" => Ok(Element::Sm),
            "eu" => Ok(Element::Eu),
            "gd" => Ok(Element::Gd),
            "tb" => Ok(Element::Tb),
            "dy" => Ok(Element::Dy),
            "ho" => Ok(Element::Ho),
            "er" => Ok(Element::Er),
            "tm" => Ok(Element::Tm),
            "yb" => Ok(Element::Yb),
            "lu" => Ok(Element::Lu),
            "hf" => Ok(Element::Hf),
            "ta" => Ok(Element::Ta),
            "w" => Ok(Element::W),
            "re" => Ok(Element::Re),
            "os" => Ok(Element::Os),
            "ir" => Ok(Element::Ir),
            "pt" => Ok(Element::Pt),
            "au" => Ok(Element::Au),
            "hg" => Ok(Element::Hg),
            "tl" => Ok(Element::Tl),
            "pb" => Ok(Element::Pb),
            "bi" => Ok(Element::Bi),
            "po" => Ok(Element::Po),
            "at" => Ok(Element::At),
            "rn" => Ok(Element::Rn),
            "fr" => Ok(Element::Fr),
            "ra" => Ok(Element::Ra),
            "ac" => Ok(Element::Ac),
            "th" => Ok(Element::Th),
            "pa" => Ok(Element::Pa),
            "u" => Ok(Element::U),
            "np" => Ok(Element::Np),
            "pu" => Ok(Element::Pu),
            "am" => Ok(Element::Am),
            "cm" => Ok(Element::Cm),
            "bk" => Ok(Element::Bk),
            "cf" => Ok(Element::Cf),
            "es" => Ok(Element::Es),
            "fm" => Ok(Element::Fm),
            "md" => Ok(Element::Md),
            "no" => Ok(Element::No),
            "lr" => Ok(Element::Lr),
            "rf" => Ok(Element::Rf),
            "db" => Ok(Element::Db),
            "sg" => Ok(Element::Sg),
            "bh" => Ok(Element::Bh),
            "hs" => Ok(Element::Hs),
            "mt" => Ok(Element::Mt),
            "ds" => Ok(Element::Ds),
            "rg" => Ok(Element::Rg),
            "cn" => Ok(Element::Cn),
            "nh" => Ok(Element::Nh),
            "fl" => Ok(Element::Fl),
            "mc" => Ok(Element::Mc),
            "lv" => Ok(Element::Lv),
            "ts" => Ok(Element::Ts),
            "og" => Ok(Element::Og),
            _ => Err(Error::InvalidAtomicSymbol(s.to_owned())),
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Any => write!(f, "*"),
            Element::H => write!(f, "H"),
            Element::He => write!(f, "He"),
            Element::Li => write!(f, "Li"),
            Element::Be => write!(f, "Be"),
            Element::B => write!(f, "B"),
            Element::C => write!(f, "C"),
            Element::N => write!(f, "N"),
            Element::O => write!(f, "O"),
            Element::F => write!(f, "F"),
            Element::Ne => write!(f, "Ne"),
            Element::Na => write!(f, "Na"),
            Element::Mg => write!(f, "Mg"),
            Element::Al => write!(f, "Al"),
            Element::Si => write!(f, "Si"),
            Element::P => write!(f, "P"),
            Element::S => write!(f, "S"),
            Element::Cl => write!(f, "Cl"),
            Element::Ar => write!(f, "Ar"),
            Element::K => write!(f, "K"),
            Element::Ca => write!(f, "Ca"),
            Element::Sc => write!(f, "Sc"),
            Element::Ti => write!(f, "Ti"),
            Element::V => write!(f, "V"),
            Element::Cr => write!(f, "Cr"),
            Element::Mn => write!(f, "Mn"),
            Element::Fe => write!(f, "Fe"),
            Element::Co => write!(f, "Co"),
            Element::Ni => write!(f, "Ni"),
            Element::Cu => write!(f, "Cu"),
            Element::Zn => write!(f, "Zn"),
            Element::Ga => write!(f, "Ga"),
            Element::Ge => write!(f, "Ge"),
            Element::As => write!(f, "As"),
            Element::Se => write!(f, "Se"),
            Element::Br => write!(f, "Br"),
            Element::Kr => write!(f, "Kr"),
            Element::Rb => write!(f, "Rb"),
            Element::Sr => write!(f, "Sr"),
            Element::Y => write!(f, "Y"),
            Element::Zr => write!(f, "Zr"),
            Element::Nb => write!(f, "Nb"),
            Element::Mo => write!(f, "Mo"),
            Element::Tc => write!(f, "Tc"),
            Element::Ru => write!(f, "Ru"),
            Element::Rh => write!(f, "Rh"),
            Element::Pd => write!(f, "Pd"),
            Element::Ag => write!(f, "Ag"),
            Element::Cd => write!(f, "Cd"),
            Element::In => write!(f, "In"),
            Element::Sn => write!(f, "Sn"),
            Element::Sb => write!(f, "Sb"),
            Element::Te => write!(f, "Te"),
            Element::I => write!(f, "I"),
            Element::Xe => write!(f, "Xe"),
            Element::Cs => write!(f, "Cs"),
            Element::Ba => write!(f, "Ba"),
            Element::La => write!(f, "La"),
            Element::Ce => write!(f, "Ce"),
            Element::Pr => write!(f, "Pr"),
            Element::Nd => write!(f, "Nd"),
            Element::Pm => write!(f, "Pm"),
            Element::Sm => write!(f, "Sm"),
            Element::Eu => write!(f, "Eu"),
            Element::Gd => write!(f, "Gd"),
            Element::Tb => write!(f, "Tb"),
            Element::Dy => write!(f, "Dy"),
            Element::Ho => write!(f, "Ho"),
            Element::Er => write!(f, "Er"),
            Element::Tm => write!(f, "Tm"),
            Element::Yb => write!(f, "Yb"),
            Element::Lu => write!(f, "Lu"),
            Element::Hf => write!(f, "Hf"),
            Element::Ta => write!(f, "Ta"),
            Element::W => write!(f, "W"),
            Element::Re => write!(f, "Re"),
            Element::Os => write!(f, "Os"),
            Element::Ir => write!(f, "Ir"),
            Element::Pt => write!(f, "Pt"),
            Element::Au => write!(f, "Au"),
            Element::Hg => write!(f, "Hg"),
            Element::Tl => write!(f, "Tl"),
            Element::Pb => write!(f, "Pb"),
            Element::Bi => write!(f, "Bi"),
            Element::Po => write!(f, "Po"),
            Element::At => write!(f, "At"),
            Element::Rn => write!(f, "Rn"),
            Element::Fr => write!(f, "Fr"),
            Element::Ra => write!(f, "Ra"),
            Element::Ac => write!(f, "Ac"),
            Element::Th => write!(f, "Th"),
            Element::Pa => write!(f, "Pa"),
            Element::U => write!(f, "U"),
            Element::Np => write!(f, "Np"),
            Element::Pu => write!(f, "Pu"),
            Element::Am => write!(f, "Am"),
            Element::Cm => write!(f, "Cm"),
            Element::Bk => write!(f, "Bk"),
            Element::Cf => write!(f, "Cf"),
            Element::Es => write!(f, "Es"),
            Element::Fm => write!(f, "Fm"),
            Element::Md => write!(f, "Md"),
            Element::No => write!(f, "No"),
            Element::Lr => write!(f, "Lr"),
            Element::Rf => write!(f, "Rf"),
            Element::Db => write!(f, "Db"),
            Element::Sg => write!(f, "Sg"),
            Element::Bh => write!(f, "Bh"),
            Element::Hs => write!(f, "Hs"),
            Element::Mt => write!(f, "Mt"),
            Element::Ds => write!(f, "Ds"),
            Element::Rg => write!(f, "Rg"),
            Element::Cn => write!(f, "Cn"),
            Element::Nh => write!(f, "Nh"),
            Element::Fl => write!(f, "Fl"),
            Element::Mc => write!(f, "Mc"),
            Element::Lv => write!(f, "Lv"),
            Element::Ts => write!(f, "Ts"),
            Element::Og => write!(f, "Og"),
        }
    }
}

impl Element {
    /// Returns the atomic number of the Element.
    pub fn atomic_number(&self) -> u8 {
        u8::from(self)
    }

    /// Returns the atomic symbol of the Element.
    pub fn atomic_symbol(&self) -> String {
        self.to_string()
    }

    /// Returns the atomic weight of the Element. If isotope is None, the
    /// standard atomic weight is given.
    ///
    /// Weights are sourced from NIST.
    pub fn atomic_weight(&self, isotope: Option<u16>) -> Result<f64, Error> {
        match self {
            Element::Any => Ok(0.0),
            Element::H => match isotope {
                None => Ok(1.007_975),
                Some(isotope) => match isotope {
                    1 => Ok(1.007_825),
                    2 => Ok(2.014_102),
                    3 => Ok(3.016_049),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::He => match isotope {
                None => Ok(4.002_602),
                Some(isotope) => match isotope {
                    3 => Ok(3.016_029),
                    4 => Ok(4.002_603),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Li => match isotope {
                None => Ok(6.967_5),
                Some(isotope) => match isotope {
                    6 => Ok(6.015_123),
                    7 => Ok(7.016_003),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Be => match isotope {
                None => Ok(9.012_183),
                Some(isotope) => match isotope {
                    9 => Ok(9.012_183),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::B => match isotope {
                None => Ok(10.813_5),
                Some(isotope) => match isotope {
                    10 => Ok(10.012_937),
                    11 => Ok(11.009_305),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::C => match isotope {
                None => Ok(12.010_6),
                Some(isotope) => match isotope {
                    12 => Ok(12.000_000),
                    13 => Ok(13.003_355),
                    14 => Ok(14.003_242),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::N => match isotope {
                None => Ok(14.006_855),
                Some(isotope) => match isotope {
                    14 => Ok(14.003_074),
                    15 => Ok(15.000_109),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::O => match isotope {
                None => Ok(15.9994),
                Some(isotope) => match isotope {
                    16 => Ok(15.994_915),
                    17 => Ok(16.999_132),
                    18 => Ok(17.999_160),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::F => match isotope {
                None => Ok(18.998_403),
                Some(isotope) => match isotope {
                    19 => Ok(18.998_403),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ne => match isotope {
                None => Ok(20.179_7),
                Some(isotope) => match isotope {
                    20 => Ok(19.997_440),
                    21 => Ok(20.993_847),
                    22 => Ok(21.991_385),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Na => match isotope {
                None => Ok(22.989_769),
                Some(isotope) => match isotope {
                    23 => Ok(22.989_769),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Mg => match isotope {
                None => Ok(24.305_5),
                Some(isotope) => match isotope {
                    24 => Ok(23.985_042),
                    25 => Ok(24.985_837),
                    26 => Ok(25.982_593),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Al => match isotope {
                None => Ok(26.981_539),
                Some(isotope) => match isotope {
                    27 => Ok(26.981_538),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Si => match isotope {
                None => Ok(28.085),
                Some(isotope) => match isotope {
                    28 => Ok(27.976_927),
                    29 => Ok(28.976_495),
                    30 => Ok(29.973_770),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::P => match isotope {
                None => Ok(30.973_762),
                Some(isotope) => match isotope {
                    31 => Ok(30.973_762),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::S => match isotope {
                None => Ok(32.0675),
                Some(isotope) => match isotope {
                    32 => Ok(31.972_071),
                    33 => Ok(32.971_459),
                    34 => Ok(33.967_867),
                    36 => Ok(35.967_081),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Cl => match isotope {
                None => Ok(35.4515),
                Some(isotope) => match isotope {
                    35 => Ok(34.968_853),
                    37 => Ok(36.965_093),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ar => match isotope {
                None => Ok(39.948_1),
                Some(isotope) => match isotope {
                    36 => Ok(35.967_545),
                    38 => Ok(37.962_732),
                    40 => Ok(39.962_383),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Br => match isotope {
                None => Ok(79.904),
                Some(isotope) => match isotope {
                    79 => Ok(78.918_338),
                    81 => Ok(80.916_290),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::I => match isotope {
                None => Ok(126.904_473),
                Some(isotope) => match isotope {
                    127 => Ok(126.904_472),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            _ => unimplemented!(),
        }
    }

    /// Returns the number of valence electrons only for aliphatic/aromatic
    /// elements under the OpenSMILES specification.
    pub fn n_valence_electrons(&self, formal_charge: i8) -> Result<u8, Error> {
        let mut n_valence_electrons = match self {
            Element::H => 1,
            Element::B => 3,
            Element::C => 4,
            Element::N => 5,
            Element::O => 6,
            Element::F => 7,
            Element::P => 5,
            Element::S => 6,
            Element::Cl => 7,
            Element::Br => 7,
            Element::I => 7,
            _ => unimplemented!(),
        };

        n_valence_electrons -= formal_charge;
        if !(0..=8).contains(&n_valence_electrons) {
            return Err(Error::InvalidFormalCharge(
                self.atomic_symbol(),
                formal_charge,
            ));
        }

        Ok(n_valence_electrons as u8)
    }

    /// The valence only for aliphatic/aromatic elements under the OpenSMILES
    /// specification.
    pub fn valence(&self, formal_charge: i8) -> Result<u8, Error> {
        let n_valence_electrons = self.n_valence_electrons(formal_charge)?;

        match n_valence_electrons {
            0 => Ok(0),
            1 => Ok(1),
            2 => Ok(2),
            3 => Ok(3),
            4 => Ok(4),
            5 => Ok(3),
            6 => Ok(2),
            7 => Ok(1),
            8 => Ok(0),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let error = Error::InvalidAtomicNumber(200);
        assert_eq!(format!("{error}"), "invalid atomic number 200".to_owned());

        let error = Error::InvalidAtomicSymbol("A".to_owned());
        assert_eq!(format!("{error}"), "invalid atomic symbol A");

        let error = Error::InvalidIsotope("C".to_owned(), 10);
        assert_eq!(format!("{error}"), "invalid isotope 10 for C");

        let error = Error::InvalidFormalCharge("O".to_owned(), -3);
        assert_eq!(format!("{error}"), "invalid formal charge -3 for O");
    }

    #[test]
    fn test_try_from_atomic_number() {
        assert_eq!(Element::H, Element::try_from(1).unwrap());
        assert_eq!(Element::C, Element::try_from(6).unwrap());
        assert_eq!(Err(Error::InvalidAtomicNumber(200)), Element::try_from(200));
    }

    #[test]
    fn test_atomic_number() {
        assert_eq!(Element::H.atomic_number(), 1);
        assert_eq!(Element::C.atomic_number(), 6);
    }

    #[test]
    fn test_from_atomic_symbol() {
        assert_eq!(Element::H, "H".parse().unwrap());
        assert_eq!(Element::C, "c".parse().unwrap());
        assert_eq!(
            Err(Error::InvalidAtomicSymbol("A".to_owned())),
            "A".parse::<Element>()
        );
    }

    #[test]
    fn test_atomic_symbol() {
        assert_eq!(Element::H.atomic_symbol(), "H".to_owned());
        assert_eq!(Element::C.atomic_symbol(), "C".to_owned());
    }

    #[test]
    fn test_to_str() {
        assert_eq!(format!("{}", Element::H), "H");
        assert_eq!(format!("{}", Element::C), "C");
    }

    #[test]
    fn test_atomic_weight() {
        assert_eq!(Element::H.atomic_weight(None).unwrap(), 1.007975);
        assert_eq!(Element::C.atomic_weight(Some(13)).unwrap(), 13.003355);
    }

    #[test]
    fn test_valence_electrons() {
        assert_eq!(Element::C.n_valence_electrons(0).unwrap(), 4);
        assert_eq!(Element::C.n_valence_electrons(-1).unwrap(), 5);
        assert_eq!(Element::O.n_valence_electrons(-2).unwrap(), 8);
        assert_eq!(
            Element::F.n_valence_electrons(-2),
            Err(Error::InvalidFormalCharge("F".to_owned(), -2))
        )
    }

    #[test]
    fn test_valence() {
        assert_eq!(Element::C.valence(0).unwrap(), 4);
        assert_eq!(Element::S.valence(0).unwrap(), 2);
    }
}
