use engine::PlayStyle;
use crate::patterns::types::{EliteDefensiveSystemDefinition, DefensiveSystemPhase};

pub fn system() -> EliteDefensiveSystemDefinition {
    EliteDefensiveSystemDefinition {
        id: "high_press_recovery",
        name: "High Press Recovery System",
        source_md: "High_Press_Recovery_System.md",
        structural_idea: "Force the opponent to build where you want, then recover the ball near goal.",
        real_world_archetypes: &["Liverpool (Klopp)", "Napoli (Spalletti)", "RB Leipzig (Nagelsmann)"],
        base_play_style: PlayStyle::HighPress,
        phases: vec![
            DefensiveSystemPhase {
                name: "First Line Press",
                defensive_form_id: "high_press_defense",
                tactical_purpose: "Force errors near opponent box",
                entry_condition: "Ball at opponent CB or FB in build-up",
                handoff: "Ball goes wide or pressure beaten",
                weight: 0.35,
            },
            DefensiveSystemPhase {
                name: "Lane Control",
                defensive_form_id: "cover_shadow_defending",
                tactical_purpose: "Hide pivot and force wide",
                entry_condition: "First press lands, receiver isolated",
                handoff: "Ball enters wide trap zone",
                weight: 0.35,
            },
            DefensiveSystemPhase {
                name: "Trap Execution",
                defensive_form_id: "pressing_traps",
                tactical_purpose: "Collapse on wide receiver and win ball",
                entry_condition: "Ball played to touchline",
                handoff: "Turnover or press reset",
                weight: 0.30,
            },
        ],
        final_output: "Turnover near the box, rushed clearance, or immediate attacking transition",
    }
}
