use engine::PlayStyle;
use crate::patterns::types::{EliteDefensiveSystemDefinition, DefensiveSystemPhase};

pub fn system() -> EliteDefensiveSystemDefinition {
    EliteDefensiveSystemDefinition {
        id: "high_line_sweeper",
        name: "High Line Sweeper System",
        source_md: "High_Line_Sweeper_System.md",
        structural_idea: "Compress the pitch with a high line, then protect the space behind it with timing and goalkeeper coverage.",
        real_world_archetypes: &["Liverpool (Klopp high line)", "Manchester City (Guardiola)", "Ajax (Ten Hag)"],
        base_play_style: PlayStyle::HighPress,
        phases: vec![
            DefensiveSystemPhase {
                name: "Territorial Pressure",
                defensive_form_id: "high_press_defense",
                tactical_purpose: "Force rushed forward passes",
                entry_condition: "Opponent builds from back",
                handoff: "Ball played long or wide",
                weight: 0.35,
            },
            DefensiveSystemPhase {
                name: "Line Timing",
                defensive_form_id: "offside_trap",
                tactical_purpose: "Catch runners beyond last defender",
                entry_condition: "Opponent tries ball in behind",
                handoff: "Offside called or GK needed",
                weight: 0.35,
            },
            DefensiveSystemPhase {
                name: "Depth Cover",
                defensive_form_id: "goalkeeper_sweeper_defense",
                tactical_purpose: "GK sweeps space behind high line",
                entry_condition: "Through ball played in behind",
                handoff: "GK clears, back to press",
                weight: 0.30,
            },
        ],
        final_output: "Opponent has no easy space in front of the block or behind the back line",
    }
}
