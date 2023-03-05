// --------- AF primitives ------------
// AF = int | var | var+int | var-int  
// var = small_leter ('u' - for error )

#[derive(Debug, Clone)]
pub enum Formula {
    Invalid,
    Int(i32),
    Add(u8, i32),
    Mod(i32),
}
const DIGITS: std::ops::RangeInclusive<u8> = b'0'..=b'9';
const LETTERS: std::ops::RangeInclusive<u8> = b'a'..=b'z';
use std::str::FromStr;
impl FromStr for Formula {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        let head = b[0];
        let rv = if DIGITS.contains(&head) {
            let val = s.parse::<i32>().unwrap();
            Formula::Int(val)
        } else if LETTERS.contains(&head) {
            let val =
                if s.len() == 1 { 0 }
                else { s[1..].parse::<i32>().unwrap() };
            if head == b'x' {
                Formula::Mod(val)
            } else {
                Formula::Add(head, val)
            }
        } else {
            return Err("bad formula str");
        };
        Ok(rv)
    }
}
use std::fmt;

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Formula::*;
        match self {
            Invalid => write!(f, "u"),
            &Int(val) => write!(f, "{}", val),
            &Add(var, add) => {
                if add == 0 {
                    write!(f, "{}", var as char)
                } else {
                    let plus = if add > 0 { "+" } else { "" };
                    write!(f, "{}{}{}", var as char, plus, add)
                }
            }
            &Mod(modulo) => write!(f, "x+{}", modulo),
        }
    }
}

impl Formula {
    pub fn dec(&mut self, sub: i32) {
        use Formula::*;
        *self = match self {
            Invalid =>Invalid,
            &mut Int(prev) => if prev < sub { Invalid } else { Int(prev - sub) },
            &mut Add(var, prev) => Self::Add(var, prev - sub),
            &mut Mod(prev) => Mod(prev - sub),
        }
    }
    pub fn inc(&mut self, inc: i32) {
        self.dec(-inc);
    }
    pub fn add(&self, other: &Self) -> Self {
        use Formula::*;
        match (self, other) {
            (Invalid, _) | (_, Invalid) => Invalid,
            (&Int(a), &Int(b)) => Int(a + b),
            (&Add(var, a), &Int(b)) | (&Int(a), &Add(var, b)) => Add(var, a + b),
            (&Mod(a), &Mod(b)) => Mod(a + b),
            _ => Invalid,
        }
    }
    pub fn make_mod(&self, modulo: i32) -> Self {
        use Formula::*;
        match self {
            Invalid | Add(_, _) | Mod(_) => Invalid,
            &Int(a) => {
                let mut new_mod = a % modulo;
                if new_mod == 0 { new_mod = modulo; }
                Mod(new_mod)
            }
        }
    }
    pub fn adjust_mod(&mut self, modulo: i32) -> bool {
        use Formula::*;
        let mut rv = false;
        *self = match self {
            Invalid => Invalid,
            Int(_) => panic!("invalid adjust_mod call"),
            Mod(_) => Mod(modulo),
            &mut Add(_, a) => {
                rv = true;
                let mut new_mod = a % modulo;
                if new_mod == 0 { new_mod = modulo; }
                Mod(new_mod)
            }
        };
        rv
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_roundtrip() {
        let formulas = ["3", "a", "x+3", "a-3", "a+3"];
        for formula in formulas {
            let parsed = formula.parse::<Formula>().unwrap();
            let formatted = format!("{}", parsed);
            assert_eq!(formula, formatted);
        }
    }
}
