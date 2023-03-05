use crate::BASIC_RULE_MAX;

const HUFF_SIZE: usize = 9;
const ESET_MAX: usize = 1000; // {3000} {5000} {10000};
const T_INTRON_STR_LEN: usize = 60;

// TIntroneStrLen must be enough for TryBL_Proof intrones !!!
type TIntronArr = [String; BASIC_RULE_MAX];
#[derive(Default, Clone, PartialEq, Eq)]
pub struct TExoneSet {
    pub EA: Vec<String>,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum HuffMode { Huffman, SemiHuffman }

fn double_exones(exones: &mut TExoneSet) {
    let count = exones.EA.len();
    if count > 32 {
        return
    }
    for ii in 0..count {
        let mut last = exones.EA[ii].clone();
        last.insert(0, '-');
        exones.EA.push(last);
        exones.EA[ii].insert(0, '+');
    }
}

fn make_new_set(
    codes: &mut Vec<TExoneSet>,
    mut c: TExoneSet,
    ii: usize, s_mode: usize,
) {
    let last;
    match s_mode {
        0 => {
            last = "-".to_string() + &c.EA[ii];
            c.EA[ii].insert(0, '+');
        }
        1 => last = "+".to_string() + &c.EA[ii],
        2 => last = "-".to_string() + &c.EA[ii],
        _ => unreachable!(),
    }
    c.EA.push(last);
    // SortHSet
    c.EA.sort_by(|a, b| {
        a.len().cmp(&b.len()).reverse().then_with(||{
            a.chars().rev().cmp(b.chars().rev())
        })
    });
    let exists = codes.iter().any(|old| old == &c);
    if !exists && codes.len() < ESET_MAX {
        codes.push(c);
    }
}

fn test_acc(codes: &TExoneSet, ii: usize) -> bool {
    let mut rv = false;
    let mut s0 = codes.EA[ii].clone().into_bytes();
    s0[0] = if s0[0] == b'-' { b'+' } else { b'-' };
    for s1 in &codes.EA {
        let mut s1 = s1.as_bytes();
        if s1.len() > s0.len() {
            s1 = &s1[s1.len() - s0.len()..];
        }
        let s0 = std::str::from_utf8(&s0).unwrap();
        let s1 = std::str::from_utf8(s1).unwrap();
        if s1 == s0 {
            rv = true;
        }
    }
    rv
}

pub fn print_codes(fd: &mut std::fs::File, codes: &[TExoneSet], mode: HuffMode) ->std::io::Result<()> {
    use std::io::Write;
    match mode {
        HuffMode::Huffman =>     writeln!(fd, "------------- Huffman codes: --------------")?,
        HuffMode::SemiHuffman => writeln!(fd, "----------- Semi_Huffman codes: -----------")?,
    }
    for (ii, code) in codes.iter().enumerate() {
        write!(fd, "HuffSet[{}] cnt={}   ", ii, code.EA.len())?;
        for (jj, s) in code.EA.iter().enumerate() {
            fd.write_all(&[b'a'+jj as u8])?;
            write!(fd, "={} ", s)?;
        }
        writeln!(fd)?;
    }
    writeln!(fd, "-------------------------------------------")?; 
    fd.flush()?;
    Ok(())
}

pub fn create_huff_codes(mode: HuffMode) -> Vec<TExoneSet> {
    let mut huff_codes = vec![TExoneSet::default(); 5];
    // TODO: InitHStat()
    let first = &mut huff_codes[0];
    first.EA.push("+".to_string());
    first.EA.push("-".to_string());
    huff_codes[4] = huff_codes[0].clone(); // 1-bit set
    double_exones(&mut huff_codes[0]); // 2-bit set
    double_exones(&mut huff_codes[0]); // 3-bit set
    huff_codes[1] = huff_codes[0].clone();
    double_exones(&mut huff_codes[1]); // 4-bit set
    huff_codes[2] = huff_codes[1].clone();
    double_exones(&mut huff_codes[2]); // 5-bit set
    huff_codes[3] = huff_codes[2].clone();
    double_exones(&mut huff_codes[3]); // 6-bit set
    let mut gen_cnt = 4;
    while huff_codes.len() > gen_cnt && huff_codes.len() < ESET_MAX {
        let current = huff_codes[gen_cnt].clone();
        let n = current.EA.len();
        if n - 1 <= HUFF_SIZE {
            for ii in 0..n {
                let mut allowed = true;
                for jj in 0..n {
                    if &current.EA[jj][1..] == &current.EA[ii] {
                        allowed = false;
                    }
                }
                if allowed {
                    match mode {
                        HuffMode::Huffman => {
                            make_new_set(&mut huff_codes, current.clone(), ii, 0);
                        }
                        HuffMode::SemiHuffman => {
                            if test_acc(&current, ii) {
                               make_new_set(&mut huff_codes, current.clone(), ii, 0);
                            }
                            make_new_set(&mut huff_codes, current.clone(), ii, 1);
                            make_new_set(&mut huff_codes, current.clone(), ii, 2);
                        }
                    }
                }
            }
        }
        gen_cnt += 1;
    }
    let mid_v = match mode {
        HuffMode::Huffman => 25,
        HuffMode::SemiHuffman => 39,
    };
    huff_codes[..mid_v + 1].rotate_left(4);
    huff_codes
}