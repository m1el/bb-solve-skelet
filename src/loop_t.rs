use crate::{EvaluationResult, TDescr, TRule, State, DM_2_QUB};
pub struct LoopInfo {
    pub rmin0: usize,
    pub rulem: usize,
    pub rule_min: usize,
    pub simple_history: Vec<TDescr>,
    // SRules Approved shift rules
    pub app_shift_rules: Vec<TRule>,
    // SSRules Special approved shift rules
    pub spec_shift_rules: Vec<TRule>,
    // ASRules All shift rules
    all_shift_rules: Vec<TRule>,
    // BSRules Basic shift rules
    pub basic_shift_rules: Vec<TRule>,
    pub current_desc: TDescr,
    pub state: EvaluationResult,
    pub p_right: isize,
    pub p_left: isize,
}

impl LoopInfo {
    fn clear0() -> Self {
        Self {
            rmin0: 0,
            rulem: 0,
            rule_min: 0,
            p_left: 0,
            p_right: 0,
            simple_history: Vec::new(),
            app_shift_rules: Vec::new(),
            all_shift_rules: Vec::new(),
            basic_shift_rules: Vec::new(),
            spec_shift_rules: Vec::new(),
            current_desc: TDescr::default(),
            state: EvaluationResult::Initial,
        }
    }
}

fn pass0() {
    let mut info = LoopInfo::clear0();
    while info.state as u8 == 0 || info.simple_history.len() <= DM_2_QUB {
        info.simple_history.push(info.current_desc.clone());
    }
}
/*
 procedure Pass0;
  begin
   clear0; 
{
  writeln(ttm,'M_c2=',m,' e_state=',e_state,' rmin0=',rmin0); flush(ttm);
}    
   repeat
    SH[SH_cnt]:=CD; inc(SH_cnt); 
    MakeMStep0(CD,0);
    if (e_state=0) then ShiftTest;
    if new_def and (e_state=0) then begin
     InvSTest;
     if (e_state=0) and lDum then back_track;
     new_def:=false;
    end;
    if e_state=0 then begin
     if ((SH_cnt>10) and (SH_cnt mod 200=70) and (SH_cnt<800)) 
        or (SH_cnt=1500) 
     then begin 
      SortRules;

      if e_state in [0,mem_ov,too_lng] then Pass2(SH_cnt); 
      if (e_state in [0,mem_ov,too_lng]) and (SH_cnt=1500) then begin
       { Pass2(SH_cnt); }
       TryBL_Loop(SH_cnt);
      end; 
      
      if e_state in [0,mem_ov,too_lng] then Pass1(true,SH_cnt); 

      if ((SH_cnt=470) or (SH_cnt=1500)) and
         (e_state in [0,mem_ov,too_lng]) 
       then ExonesSpecFast(SH_cnt); 

      if (e_state in [0,mem_ov,too_lng]) and (SH_cnt>200) then TryPFin_2; 
      if (e_state in [0,mem_ov,too_lng]) and (SH_cnt=470) then TryPFin_2_all(2); 
     end;   
     if e_state in [mem_ov,too_lng] then e_state:=0;  
    end else if e_state=nm_end then Pass2(cmax - 1 {SH_cnt+1});    
   until (e_state>0) or (SH_cnt>dm2qub);
   SortRules;
  end;
*/

pub fn loop_t(state: &mut State) {
    pass0();
}
/*
  Pass0;
  if e_state in [0,mem_ov,too_lng] then begin

   if not ShRecTest then begin
    if e_state in [0,mem_ov,too_lng] then Pass100K(cmax div 5);
    if e_state in [0,mem_ov,too_lng] then TryPFin_2_all(dm);
    if e_state in [0,mem_ov,too_lng] then Pass1(true,cmax);
    max_rid:=sr_lmax*2;
    while (e_state in [0,mem_ov,too_lng]) and (AvRule>0)
     do Pass1(false,cmax);
    if e_state in [0,mem_ov,too_lng] then Pass2(cmax); 
    if e_state in [0,mem_ov,too_lng] then TryBL_Loop(SH_cnt);
    SpecStr:=StrRCnt(spec_max);
    if (e_state in [0,mem_ov,too_lng]) and (not slow_scan)
     then ExonesSpecFast(cmax);
    if e_state in [0,mem_ov,too_lng] then Pass100K(cmax);
    if e_state in [0,mem_ov,too_lng] then TryPFin_2_all(dm_2rR);
    if (e_state in [0,mem_ov,too_lng]) and (slow_scan)
     then ExonesSpecFast(cmax);
   end;

   if e_state in [0,mem_ov,too_lng] then begin {Pass2(cmax); }
    clear(true);
    rule_min:=best_rlen; rep_min:=1;
    MacroLoop(CM,MH,MH_cnt,1,cmax);
   end;
  end;
  
  info1;

  if e_state in [0,mem_ov,too_lng] then begin 
   if ShRecTest then begin
    writeln(tsrec,'M# = ',m-1); 
    wrf_m(tsrec,true,false,'ShRec');
    wr_rules(tsrec,'all shift',ASRules,ASR_cnt,true);
    wr_history(tsrec);
    flush(tsrec); 
    nrtyp:='SRec';
   end else nrtyp:='----';
   SpecStr:=StrRCnt(spec_max);
   SpecAdd(SpecStr);
   wrf_m(tnr1,e_cnt[mem_ov]+e_cnt[too_lng]<=1,false,nrtyp+SpecStr);
   flush(tnr1);
  end; 
  
 end; { loop_t }
 */