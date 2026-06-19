use engine::PlayStyle;
use crate::patterns::types::{EliteAttackingSystemDefinition, SystemPhase};

pub fn system() -> EliteAttackingSystemDefinition {
    EliteAttackingSystemDefinition {
        id: "box_control_crossing",
        name: "Box Control Crossing System",
        source_md: "Box_Control_Crossing_System.md",
        structural_idea: "Treat open play like structured set pieces — position, pull defenders, then deliver into a pre-loaded box",
        real_world_archetypes: &["Manchester City (wide build)", "Chelsea (Tuchel)", "West Ham (Moyes)"],
        base_play_style: PlayStyle::Attacking,
        phases: vec![
            SystemPhase {
                name: "Positional Box Occupation",
                attacking_form_id: "positional_play",
                tactical_purpose: "Five-lane structure maintained; strict zone spacing ensures attackers ready for delivery at all times",
                entry_condition: "Team in possession in final third or approaching it; formation shape set",
                handoff: "Wide combination triggered once box occupation confirmed and defenders identified",
                weight: 0.30,
            },
            SystemPhase {
                name: "Wide Combination Pre-Cross",
                attacking_form_id: "combination_crossing",
                tactical_purpose: "Multiple passes before cross shift defenders laterally; crossing lane opened by combination movement",
                entry_condition: "Box occupied — at least 3 runners in penalty area ready; wide player in possession",
                handoff: "Cross delivery once combination creates clearest crossing angle and maximum defensive disorganization",
                weight: 0.40,
            },
            SystemPhase {
                name: "Cross Execution — Variation Selection",
                attacking_form_id: "crossing_variations",
                tactical_purpose: "Choose cross type based on defensive positioning — early, inswinger, driven low, or cutback",
                entry_condition: "Crosser in position with ball after combination; box loaded with ST, CM, and far-post runner",
                handoff: "Structured finish from controlled box: ST near post, CM penalty spot, winger far post",
                weight: 0.30,
            },
        ],
        final_output: "Structured finish from controlled box chaos; set-piece level organization in open play",
    }
}
