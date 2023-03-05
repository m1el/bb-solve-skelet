#![allow(dead_code, non_snake_case)]

mod huff;
mod machine_step;
mod af;
mod loop_t;
use huff::{HuffMode, create_huff_codes};

/// TM dimension. Must be 2,3,4,5 or 6
const TM_STATES: usize = 5;
/// Reversal / Normal TM
/// Only consider Turing Machines that reverse the
/// value at current location on every step.
/// https://skelet.ludost.net/bb/RTM.htm
const REVERSAL: bool = false;
/// Dir(0) = Dir(1) limitation
/// Only consider Turing machiens which have the following property:
/// For every TM state, the direction of movement is the same
/// for all values of current tape cell
const SAME_DIRECTION: bool = false;
/// Destructive TM: value_1=1 limitation
const DESTRUCTIVE: bool = false;
/// slow/fast scan
const SLOW_SCAN: bool = true; 
const SLOW_SC0: bool = true;
const SLOW_SC2: bool = true;
const SLOW_0: bool = SLOW_SCAN && SLOW_SC0;
const SLOW_2: bool = SLOW_SCAN && SLOW_SC2;
/// scan only machines, writing 1 at first step
/// Note: This trivially reduces the number of machines,
/// since a TM that writes zero and switches to state X
/// is equivalent to TM that starts with X.
const IGNORE_FIRST_ZERO: bool = true;

/// NOTE: these are intermediate computation steps
/// for the heuristics of max steps for rules
const DM_2: usize = TM_STATES + TM_STATES;
const DM_2R: usize = DM_2
    - 2 * ((REVERSAL as usize) + (SAME_DIRECTION as usize)) * (1 - (DESTRUCTIVE as usize))
    - 4 * (DESTRUCTIVE as usize);
const DM_2RR: usize = DM_2R - 2 * (REVERSAL as usize);
/// limit size for rule finder
const DM_2_SQR: usize = DM_2R * DM_2R;
const DM_2_QUB: usize = DM_2_SQR * DM_2R * 2 + 1900;
// limit step for rule finder and size of used arrays
// 4 --> 2024, 5 --> 3000, 6 --> 4440
// t_low=0; t_mid=40000; t_hig=80000;
// start, midle and end tape position

/// Upper limit for steps, cmax
const CYCLES_MAX: usize = 100 + DM_2_QUB;
/// Upper limit for used tape, mmax
const MEMORY_MAX: usize = 250;
/// Stack size s_sz
const STACK_SIZE: usize =100;
/// undefined state dum
/// TODO: replace this?
const UNDEFINED: u8 = 99;
/// how many dump steps
const DUMP_MAX: usize = 400;
/// Upper limit for rules
/// 2000 is small for some machines !!!
const RULES_MAX: usize = 3000;
/// max len for approved shift rules
const SHIFT_RULES_LEN_MAX: usize = 31;
/// How many proved machines to show from any group
const GROUP_SHOW_MAX: usize = 20;
/// Limit for basic rulei and exones arrays
const BASIC_RULE_MAX: usize = 64;
// Test results:
const EVALUATION_OUTCOMES: usize = 21;
#[derive(Debug, Clone, Copy)]
pub enum EvaluationResult {
    /// 
    Initial = 0,
    /// normal finish
    NormalFinish = 1,
    /// tape overflow
    MemoryOverflow = 2, 
    /// time overflow
    TimeOverflow = 3, 
    // Hard tests for infinite loops:
    /// back-track test, halt state unrechable
    BacktrackUnreachable = 4,
    /// invariant states
    InvariantStates = 5,
    /// in place circle
    InPlaceCircle = 6,
    /// closed sub-graph in TM graph, halt unrechable 
    SubGraphUnreachable = 7,
    /// moving left or right infinite
    MovingInfinite = 8,
    /// simple linear counter with 1 repeating word
    LinearCounter = 9,
    /// finite transition graph (modulo version)
    FiniteTransition = 10,
    /// finite position formula - binary or similar counters
    FinitePositionCounter = 11,
    /// swiming position test   - binary or similar counters
    SwimmingPositionCounter = 12,
    /// Exones chaotick loop 0
    ExonesChaoticLoop0 = 13,
    /// Exones chaotick loop 1
    ExonesChaoticLoop1 = 14,
    /// finite exones graph loop
    ExonesFiniteGraph=15,
    /// finite exones + intrones graph loop
    Exones = 16,
    /// Exones special loop 0
    ExonesSpecialLoop0 = 17,
    /// Exones special loop 1
    ExonesSpecialLoop1 = 18,
    /// Exones special loop 2
    ExonesSpecialLoop2 = 19,
    /// Finite transition graph
    BLfinl = 20,
    /// Collatz like function
    CollatzLike = 21,
}
const SPECTRUM_MAX: usize = 10;

type TArray = [u8; DM_2];
#[derive(Debug, Clone)]
pub struct Machine {
    /// next state,
    a: TArray,
    /// direction
    b: TArray,
    /// write
    c: TArray,
}
impl Machine {
    fn init(halt_pos: usize) -> Self {
        let mut a = TArray::default();
        let mut b = TArray::default();
        let mut c = TArray::default();
        for ll in 0..DM_2 {
            a[ll] = UNDEFINED;
            b[ll] = UNDEFINED;
            c[ll] = if REVERSAL {
                (ll < TM_STATES) as u8
            } else {
                UNDEFINED
            };
        }
        b[0] = 0;
        if SAME_DIRECTION { b[TM_STATES] = 0; }
        if IGNORE_FIRST_ZERO { b[0] = 1; }
        if halt_pos == TM_STATES {
            a[2] = 0;
            if !SAME_DIRECTION { b[2] = 0; }
            c[2] = 1;
        } else {
            a[halt_pos] = 0;
            if !SAME_DIRECTION { b[halt_pos] = 0; }
            if !REVERSAL { c[halt_pos] = 1; }
        }
        if DESTRUCTIVE {
            if REVERSAL || SAME_DIRECTION {
                println!();
                println!("Reversal and SameDir must be false !!!");
                panic!();
            }
            for ll in 1..TM_STATES {
                c[TM_STATES + ll] = 1
            }
        }
        Self { a, b, c }
    }
}
/*
struct StateChange {
    next: u8,
    write: u8,
}
struct Machine {
    [[StateChange; 2]; TM_STATES],
}
impl FromStr for Machine {
    type 
    fn from_str(s: &str) -> Result<Self> {
        let s = s.as_bytes();

    }
}
*/
/// full state of the machine
#[derive(Clone)]
pub struct TDescr {
    // /// value under the head
    // value: u8,
    /// move direction
    /// only 1 of v/md must be used !
    move_dir: u8,
    /// current state
    state: u8,
    /// left and right parts of the tape
    /// TODO: translate to VecDeq
    left: String,
    right: String,
    /// step counter
    counter: u64,
    /// tape position
    position: isize,
    /// false/true - full_state/formula
    is_af: bool,
    // delta c/p for formulas
    delta_c: String,
    delta_p: String,
    // /// mark strings for exone proofs
    // l_mark,r_mark:string[40];
    /// rule number
    rule_number: usize,
    // rule names
    rule_name: [u8; 4],
    rule_name0: [u8; 4],
    rule_name1: [u8; 4],
    smax: usize,
}
impl Default for TDescr {
    fn default() -> Self {
        Self {
            move_dir: 0,
            state: 1,
            left: ".".to_string(),
            right: ".".to_string(),
            counter: 0,
            position: 0,
            is_af: false,
            delta_c: String::new(),
            delta_p: String::new(),
            rule_number: !0,
            rule_name: *b"--  ",
            rule_name0: [0; 4],
            rule_name1: [0; 4],
            smax: 0,
        }
    }
}
impl TDescr {
    fn get_v(&self) -> u8 {
        fn v(l: &[u8]) -> u8 {
            if l == b"." { 0 }
            else if l[1] == b'-' { 0 }
            else { 0 }
        }

        if self.move_dir == 0 { v(self.left.as_bytes()) }
        else { v(self.right.as_bytes()) }
    }
}

type THistory = [TDescr; CYCLES_MAX];
/// description for a shift rule
pub struct TRule {
    // syntax: *aS>b(c0)* --> *(c1)aS>b*
    //         *(c0)b<Sa* --> *b<Sa(c1)*
    id: u32,
    /// move direction
    move_dir: u8,
    state: u8,
    // rule patterns
    a: Vec<u8>,
    // b: Vec<u8>,
    c0: Vec<u8>,
    c1: Vec<u8>,
    /// steps needed
    cycles: usize,
    /// position moves
    position_moves: isize,
    used: bool,
    allowed: bool,
    spectrum: [usize; SPECTRUM_MAX],
}
type TRuleArr = [TRule; RULES_MAX];
struct TBRuleEl {
    // patterns
    c0: Vec<u8>,
    c1: Vec<u8>,
    /// steps needed
    cycles: usize,
    /// usage counter
    usage_cnt: usize,
}
struct TBRule {
    id: u32,
    /// move direction
    move_dir: u8,
    /// state
    state: u8,
    /// lead rule pattern
    a: Vec<u8>,
    patterns: [TBRuleEl; BASIC_RULE_MAX],
    pattern_count: usize,
    usage_cnt: usize,
    nice: usize,
    main_index: usize,
}
type TBRuleArr = [TBRule; BASIC_RULE_MAX];

use std::fs::{File, OpenOptions};
struct Outputs {
    tnr: File,
    tnr1: File,
    trc: File,
    tproof: File,
    tsrec: File,
    ttm: File,
}

fn open_output(fname: &str) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(fname)
        .expect("could not open output file")
}
const OUT_DIR: &str = "./out";

fn open_outputs(subset: &str) -> Outputs {
    Outputs {
        tnr: open_output(&format!("{OUT_DIR}/noreg_{subset}.txt")),
        tnr1: open_output(&format!("{OUT_DIR}/nreg_{subset}.txt")),
        trc: open_output(&format!("{OUT_DIR}/recrd_{subset}.txt")),
        tproof: open_output(&format!("{OUT_DIR}/proved_{subset}.txt")),
        tsrec: open_output(&format!("{OUT_DIR}/ShRec_{subset}.txt")),
        ttm: open_output(&format!("{OUT_DIR}/others.txt")),
    }
}

pub struct State {
    outputs: Outputs,
    CTG_cnt: usize,
    eval_count: [usize; EVALUATION_OUTCOMES],
    machine_count: usize,
    max_cycles: usize,
    max_sigma: usize,
    last_time: std::time::Instant,
    rec_min: usize,
    //pr_fail: usize,
    new_def: bool,

    stack: Vec<Machine>,
    machine: Machine,
}

impl State {
    fn new(outputs: Outputs) -> Self {
        Self {
            outputs,
            CTG_cnt: 0,
            eval_count: [0; EVALUATION_OUTCOMES],
            machine_count: 0,
            max_cycles: 0,
            max_sigma: 0,
            last_time: std::time::Instant::now(),
            rec_min: Self::get_rec_min(),
            //pr_fail: false,
            new_def: false,

            stack: Vec::new(),
            machine: Machine::init(TM_STATES),
        }
    }
    fn get_rec_min() -> usize {
        let mut rec_min = CYCLES_MAX;
        let dm0 = TM_STATES - (SAME_DIRECTION as usize);
        rec_min = match (dm0, REVERSAL) {
            (2, true) => 1, (2, false) => 5,
            (3, true) => 10, (3, false) => 15,
            (4, true) => 40, (4, false) => 80,
            (5, true) => 800, (5, false) => 10000000,
            (6, true) => 10000000, (6, false) => 50000000,
            _ => panic!("unsupported TM size: {}", TM_STATES),
        };
        if DESTRUCTIVE {
            rec_min = match TM_STATES {
                3 | 4 => 20,
                5 => 50,
                6 => 200,
                _ => panic!("unsupported TM size: {}", TM_STATES),
            };
        };
        rec_min
    }
}

fn main() {
    let subset_prefix = match (DESTRUCTIVE, REVERSAL, SAME_DIRECTION) {
        (true, _, _) => "DTM",
        (false, true, true) => "STM",
        (false, true, false) => "RTM",
        (false, false, true) => "UTM",
        (false, false, false) => "TM",
    };
    let subset_name = format!("{subset_prefix}{TM_STATES}");
    std::fs::create_dir_all(OUT_DIR)
        .expect("could not create output dir");
    let outputs = open_outputs(&subset_name);
    let mut state = State::new(outputs);
    let huff_codes = create_huff_codes(HuffMode::Huffman);
    huff::print_codes(&mut state.outputs.ttm, &huff_codes, HuffMode::Huffman).unwrap();
    let semi_codes = create_huff_codes(HuffMode::SemiHuffman);
    huff::print_codes(&mut state.outputs.ttm, &semi_codes, HuffMode::SemiHuffman).unwrap();
    // let a1 = dm + 0; // 0,1,2 partial scan
    for halt_pos in TM_STATES..=TM_STATES+2 { // full scan
        state.stack.push(Machine::init(halt_pos));
        while let Some(machine) = state.stack.pop() {
            state.machine = machine;
        }
    }
}
