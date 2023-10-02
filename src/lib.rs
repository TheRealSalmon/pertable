use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidAtomicNumber(usize),
    InvalidAtomicSymbol(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidAtomicNumber(atomic_number) => {
                write!(f, "invalid atomic number {atomic_number}")
            }
            Error::InvalidAtomicSymbol(atomic_symbol) => {
                write!(f, "invalid atomic symbol {atomic_symbol}")
            }
        }
    }
}

impl std::error::Error for Error {}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq)]
pub enum Element {
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

impl TryFrom<usize> for Element {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let error = Error::InvalidAtomicNumber(999);
        assert_eq!(format!("{error}"), "invalid atomic number 999".to_owned());

        let error = Error::InvalidAtomicSymbol("A".to_owned());
        assert_eq!(format!("{error}"), "invalid atomic symbol A");
    }

    #[test]
    fn test_try_from_usize() {
        assert_eq!(Element::H, Element::try_from(1).unwrap());
        assert_eq!(Element::C, Element::try_from(6).unwrap());
        assert_eq!(Err(Error::InvalidAtomicNumber(999)), Element::try_from(999));
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Element::H, "H".parse().unwrap());
        assert_eq!(Element::C, "c".parse().unwrap());
        assert_eq!(
            Err(Error::InvalidAtomicSymbol("A".to_owned())),
            "A".parse::<Element>()
        );
    }
}
