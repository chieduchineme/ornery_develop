use engine::PlayStyle;
use crate::patterns::types::{EliteAttackingSystemDefinition, SystemPhase};

pub fn system() -> EliteAttackingSystemDefinition {
    EliteAttackingSystemDefinition {
        id: "flank_dismantling",
        name: "Flank Dismantling System",
        source_md: "Flank_Dismantling_System.md",
        structural_idea: "Destroy one side with overload, break internally with underlap, then cross into pre-built box",
        real_world_archetypes: &["Liverpool (Klopp)", "Real Madrid (Ancelotti wingers)", "Bayern Munich"],
        base_play_style: PlayStyle::Attacking,
        phases: vec![
            SystemPhase {
                name: "Wide Overload Creation",
                attacking_form_id: "overload_attack",
                tactical_purpose: "LW + LB + LCM create 3v2 on left flank; force defensive collapse toward wing",
                entry_condition: "Ball enters wide zone with LB already advanced; at least two supporting runners available",
                handoff: "Once overload commits the FB and covering CM, trigger internal underlap break",
                weight: 0.35,
            },
            SystemPhase {
                name: "Internal Underlap Break",
                attacking_form_id: "underlapping_attack",
                tactical_purpose: "LB makes inside-channel run while LW holds width; exploits gap created by overload confusion",
                entry_condition: "Opposing FB has stepped up to engage LW; inside channel behind FB is exposed",
                handoff: "LB in half-space triggers combination crossing build-up before delivery",
                weight: 0.30,
            },
            SystemPhase {
                name: "Combination Crossing Delivery",
                attacking_form_id: "combination_crossing",
                tactical_purpose: "2-3 pass build-up before cross; defenders have shifted and box is pre-loaded",
                entry_condition: "LB in half-space or LW has beaten FB; box loading runners in motion",
                handoff: "Cutback or driven cross into penalty spot; ST near post, CM penalty spot",
                weight: 0.35,
            },
        ],
        final_output: "Cutback from byline OR driven cross into penalty spot; box pre-loaded with 3-4 runners",
    }
}
