#[derive(Debug)]
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
#[derive(Debug)]
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

// impl TryFrom<usize> for Element {

// }

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
}
