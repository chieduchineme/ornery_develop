use engine::PlayStyle;
use crate::patterns::types::{EliteAttackingSystemDefinition, SystemPhase};

pub fn system() -> EliteAttackingSystemDefinition {
    EliteAttackingSystemDefinition {
        id: "explosive_transition",
        name: "Explosive Transition System",
        source_md: "Explosive_Transition_System.md",
        structural_idea: "Win ball → attack immediately at maximum tempo → isolate best attacker for 1v1 finish",
        real_world_archetypes: &["Liverpool (transition moments)", "Atletico Madrid", "RB Leipzig"],
        base_play_style: PlayStyle::Counter,
        phases: vec![
            SystemPhase {
                name: "Ball Win — Instant Vertical",
                attacking_form_id: "fast_breaks",
                tactical_purpose: "Maximum tempo vertical attack the instant possession is won; no buildup, bypass midfield",
                entry_condition: "Ball recovered anywhere on pitch with at least 2 attackers ahead of ball",
                handoff: "First pass is always forward; reach final third in 2-3 passes before defense recovers",
                weight: 0.35,
            },
            SystemPhase {
                name: "Direct Penetration",
                attacking_form_id: "direct_attack",
                tactical_purpose: "First pass forward always; bypass opponent's midfield block; unsettle defense with pace",
                entry_condition: "Ball in transition — opponent mid-attack or recovering shape",
                handoff: "Ball in final third; identify isolable defender and best 1v1 attacker",
                weight: 0.30,
            },
            SystemPhase {
                name: "Isolation Creation and Finish",
                attacking_form_id: "isolation_attack",
                tactical_purpose: "Engineer clean 1v1 for highest-quality attacker; remove support players to create space",
                entry_condition: "Ball reaches final third within 3 passes; one attacker has isolated opponent defender",
                handoff: "Dribble → shot OR cutback to arriving CM OR 1v1 finish",
                weight: 0.35,
            },
        ],
        final_output: "Dribble penetration and shot OR cutback to arriving CM OR 1v1 finish vs exposed GK",
    }
}
