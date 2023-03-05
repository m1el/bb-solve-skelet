use crate::{TDescr, TRule, State, MEMORY_MAX, UNDEFINED, SAME_DIRECTION, TM_STATES, EvaluationResult};
use crate::loop_t::LoopInfo;
use crate::af::Formula;

const V_STR: [u8; 2] = [b'-', b'+'];
fn succ_m_step(loop_info: &mut LoopInfo) -> bool {
    let cd = &mut loop_info.current_desc;
    let repeat = |s: &str, rep: usize| -> String {
        std::iter::repeat(s).take(rep).collect()
    };
    let test = |_l: &mut String, _r: &mut String| true;
    let mut ii = 0;
    for rule in loop_info.app_shift_rules.iter_mut() {
        if !(cd.move_dir == rule.move_dir
            && (loop_info.rule_min == 0 || loop_info.rule_min % rule.c0.len() == 0)
            && rule.allowed) {
                continue;
            }
        let succ = if cd.position > 0 {
            test(&mut cd.left, &mut cd.right)
        } else {
            test(&mut cd.right, &mut cd.left)
        };
        if !succ {
            return true;
        }
    }
    false
}
fn succ_b_step(_cd: &mut TDescr) -> bool {
    todo!()
}
fn unpack_first(s: &mut String) {
    let b = s.as_bytes();
    if b[0] != b'(' { return; }
    let mut splits = s[1..].splitn(2, '|');
    let pattern = splits.next().unwrap();
    let mut splits = splits.next().unwrap().splitn(2, ')');
    let repeat = splits.next().unwrap();
    let rest = splits.next().unwrap();
    if repeat == "1" {
        *s = format!("{pattern}{rest}");
    } else {
        let mut repeat = repeat.parse::<Formula>().unwrap();
        repeat.dec(1);
        *s = format!("{pattern}({pattern}|{repeat}){rest}");
    }
}

fn make_d_step(cd: &mut TDescr, s1: u8, v1: u8, d1: u8, macro_: u8) {
    let mov_v = |l: &mut String, r: &mut String| -> usize {
        if v1 == 1 || r != "." {
            let prefix = V_STR[v1 as usize] as char;
            *r = format!("{prefix}{r}");
        }
        if l != "." { l.drain(..1); }
        r.len()
    };
    let set_v = |l: &mut String, r: &mut String| {
        if l != "." {
            let prefix = V_STR[v1 as usize] as char;
            *l = format!("{}{}", prefix, &l[1..]);
            if l == "-." {
               *l = ".".to_string();
            }
        } else if v1 == 1 {
            *l = "+.".to_string();
        }
        //if macro_ == 1 && ms_cnt > 3 {
        //  PackPatt()
        //}
    };
    if cd.move_dir == 0 {
        unpack_first(&mut cd.left);
    } else {
        unpack_first(&mut cd.right);
    }
    if cd.is_af && macro_ > 0 {
        cd.delta_c = "0".to_string();
        cd.delta_p = if d1 == 0 { "-1" } else { "1" }
            .to_string();
    } else {
        cd.counter += 1;
        cd.position += (d1 + d1 - 1) as isize;
    }
    if !cd.is_af && macro_ == 0
        && cd.position > -(MEMORY_MAX as isize)
        && cd.position < MEMORY_MAX as isize
    {
        // inc pos stat[p][md,s]
    }
    cd.state += s1;
    cd.rule_number = !0;
    if d1 == cd.move_dir {
        let len = if cd.move_dir == 0 {
            mov_v(&mut cd.left, &mut cd.right)
        } else {
            mov_v(&mut cd.right, &mut cd.left)
        };
        cd.smax = cd.smax.max(len);
    } else {
        if cd.move_dir == 0 {
            set_v(&mut cd.left, &mut cd.right);
        } else {
            set_v(&mut cd.right, &mut cd.left);
        }
    }
}

fn ch_state() {
    todo!()
}

pub fn machine_step(
    state: &mut State,
    cd: &mut TDescr,
    linfo: &mut LoopInfo,
    macro_: u8
) {
    let mut pr_fail = false;
    let sstep = match macro_ {
        0 => true,
        1 => !succ_m_step(&mut linfo.current_desc, linfo),
        2 => !succ_b_step(&mut linfo.current_desc),
        3 => !succ_m_step(&mut linfo.current_desc, linfo),
        _ => panic!("invalid machine step macro: {}", macro_),
    };
    if !sstep {
        return;
    }
    if cd.move_dir == 0 {
        unpack_first(&mut cd.left);
    } else {
        unpack_first(&mut cd.right);
    }
    let v0 = cd.get_v();
    cd.rule_name = [b'A' + cd.state - 1, V_STR[v0 as usize], 0, 0];
    let st = cd.state as usize;
    let (k, ko) = if v0 == 0 {
        (st, st + TM_STATES)
    } else {
        (st + TM_STATES, st)
    };
    let machine = &mut state.machine;
    let mut v1 = machine.c[k];
    if v1 == UNDEFINED {
        pr_fail = true;
    } else if machine.c[k] == 0 {
        v1 = 0;
    } else {
        machine.c[k] = 0;
        state.stack.push(machine.clone());
        machine.c[k] = 1;
        v1 = 1;
        state.new_def = true;
    }
    let s1 = machine.a[k];
    if s1 == UNDEFINED {
        if cd.is_af {
            pr_fail = true;
        } else {
            ch_state();
            state.new_def = true;
        }
    }
    let mut d1 = machine.b[k];
    if d1 == UNDEFINED {
        if cd.is_af {
            pr_fail = true;
        } else if machine.a[k] == 0 || linfo.state as u8 > 0 {
            d1 = 0;
        } else {
            machine.b[k] = 0;
            if SAME_DIRECTION { machine.b[ko] = 0; }
            state.stack.push(machine.clone());
            machine.b[k] = 1;
            if SAME_DIRECTION { machine.b[ko] = 1; }
            d1 = 1;
            state.new_def = true;
        }
    }
    if !pr_fail {
        make_d_step(cd, s1, v1, d1, macro_)
    }
    if !cd.is_af {
        if cd.position < linfo.p_left { linfo.p_left = cd.position; }
        if cd.position > linfo.p_right { linfo.p_right = cd.position; }
    }
    if s1 == 0 {
        if cd.is_af {
            pr_fail = true;
        } else {
            linfo.state = EvaluationResult::NormalFinish;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_unpack() {
        let pairs = [
            ("(+-|4).", "+-(+-|3)."),
            ("(+-|1).", "+-."),
            ("(+++-|a)+.", "+++-(+++-|a-1)+."),
            ("(+++-|a+1)+.", "+++-(+++-|a)+."),
            ("(+++-|a+2)+.", "+++-(+++-|a+1)+."),
        ];
        for (l, r) in pairs {
            let mut l = l.to_string();
            unpack_first(&mut l);
            assert_eq!(l, r);
        }
    }
}
