use engine::PlayStyle;
use crate::patterns::types::{EliteDefensiveSystemDefinition, DefensiveSystemPhase};

pub fn system() -> EliteDefensiveSystemDefinition {
    EliteDefensiveSystemDefinition {
        id: "compact_block_control",
        name: "Compact Block Control System",
        source_md: "Compact_Block_Control_System.md",
        structural_idea: "Control the opponent without chasing them, then force play into predictable wide zones.",
        real_world_archetypes: &["Atletico Madrid (Simeone)", "Inter Milan (Conte)", "Paris Saint-Germain (deep block)"],
        base_play_style: PlayStyle::Defensive,
        phases: vec![
            DefensiveSystemPhase {
                name: "Midfield Screen",
                defensive_form_id: "mid_block_defense",
                tactical_purpose: "Screen pivot and deny central progression",
                entry_condition: "Opponent builds from back",
                handoff: "Ball played wide or into block",
                weight: 0.40,
            },
            DefensiveSystemPhase {
                name: "Team Compactness",
                defensive_form_id: "compactness_defense",
                tactical_purpose: "Shrink pitch, force opponent around block",
                entry_condition: "Block set, opponent circulates",
                handoff: "Opponent enters wide zone",
                weight: 0.35,
            },
            DefensiveSystemPhase {
                name: "Forced Direction",
                defensive_form_id: "funnel_defending",
                tactical_purpose: "Guide ball to touchline for trap",
                entry_condition: "Ball approaches half-space",
                handoff: "Ball played to touchline for counter-press",
                weight: 0.25,
            },
        ],
        final_output: "Opponent circulates wide, crosses from poor zones, or plays backward",
    }
}
