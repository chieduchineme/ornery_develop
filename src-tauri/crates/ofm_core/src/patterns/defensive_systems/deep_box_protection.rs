use engine::PlayStyle;
use crate::patterns::types::{EliteDefensiveSystemDefinition, DefensiveSystemPhase};

pub fn system() -> EliteDefensiveSystemDefinition {
    EliteDefensiveSystemDefinition {
        id: "deep_box_protection",
        name: "Deep Box Protection System",
        source_md: "Deep_Box_Protection_System.md",
        structural_idea: "Accept territory, protect the penalty area, and win the decisive aerial and second-ball moments.",
        real_world_archetypes: &["Chelsea (Tuchel)", "Atletico Madrid (final-third)", "Boca Juniors"],
        base_play_style: PlayStyle::Defensive,
        phases: vec![
            DefensiveSystemPhase {
                name: "Deep Block",
                defensive_form_id: "low_block_defense",
                tactical_purpose: "Deny space in final third",
                entry_condition: "Opponent enters final third",
                handoff: "Ball moved wide",
                weight: 0.40,
            },
            DefensiveSystemPhase {
                name: "Width Protection",
                defensive_form_id: "back_five_defending",
                tactical_purpose: "Protect box width against crosses",
                entry_condition: "Cross preparation from wide",
                handoff: "Cross delivered",
                weight: 0.35,
            },
            DefensiveSystemPhase {
                name: "Penalty Area Control",
                defensive_form_id: "box_defending",
                tactical_purpose: "Win aerial and second-ball battles",
                entry_condition: "Ball arrives in box",
                handoff: "Clearance or restart",
                weight: 0.25,
            },
        ],
        final_output: "Blocked shots, cleared crosses, controlled second balls, and lower-quality opponent chances",
    }
}
