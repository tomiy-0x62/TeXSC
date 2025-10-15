use std::collections::BTreeMap;
use std::fmt;

pub struct Factorized {
    num: u64,
    facters: BTreeMap<u64, u64>,
}

impl fmt::Display for Factorized {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = ", self.num)?;
        for (base, exp, is_last) in self
            .facters
            .iter()
            .enumerate()
            .map(|(i, (b, e))| (b, e, i == self.facters.len() - 1))
        {
            if *exp == 1 {
                if is_last {
                    write!(f, "{base}")?;
                } else {
                    write!(f, "{base} * ")?;
                }
            } else if is_last {
                write!(f, "{base}^{{{exp}}}")?;
            } else {
                write!(f, "{base}^{{{exp}}} * ")?;
            }
        }
        write!(f, "")
    }
}

pub fn factorize(num: u64) -> Factorized {
    let mut facters = BTreeMap::new();
    let mut n = num;
    if num == 1 {
        facters.insert(1, 1);
    }
    for i in 2..num {
        if i * i > num {
            break;
        } else if n.is_multiple_of(i) {
            let mut exp = 0;
            while n.is_multiple_of(i) {
                n /= i;
                exp += 1;
            }
            facters.insert(i, exp);
        }
    }
    if n != 1 {
        facters.insert(n, 1);
    }
    Factorized { num, facters }
}
