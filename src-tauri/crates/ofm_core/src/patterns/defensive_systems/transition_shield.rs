use engine::PlayStyle;
use crate::patterns::types::{EliteDefensiveSystemDefinition, DefensiveSystemPhase};

pub fn system() -> EliteDefensiveSystemDefinition {
    EliteDefensiveSystemDefinition {
        id: "transition_shield",
        name: "Transition Shield System",
        source_md: "Transition_Shield_System.md",
        structural_idea: "Attack with numbers while keeping enough defensive structure to kill counter-attacks immediately.",
        real_world_archetypes: &["Manchester City (Guardiola)", "Arsenal (Arteta)", "Bayern Munich (Tuchel)"],
        base_play_style: PlayStyle::Counter,
        phases: vec![
            DefensiveSystemPhase {
                name: "Behind-Ball Security",
                defensive_form_id: "rest_defense",
                tactical_purpose: "Protect against counters during attack",
                entry_condition: "Team in possession final third",
                handoff: "Ball lost",
                weight: 0.35,
            },
            DefensiveSystemPhase {
                name: "Immediate Recovery",
                defensive_form_id: "counter_press_defense",
                tactical_purpose: "Kill counter within seconds of loss",
                entry_condition: "Possession lost",
                handoff: "Counter-press succeeds or fails",
                weight: 0.40,
            },
            DefensiveSystemPhase {
                name: "Fallback Decision",
                defensive_form_id: "transition_defense",
                tactical_purpose: "Drop and delay if counter-press fails",
                entry_condition: "Ball carrier has time on ball",
                handoff: "Block reset",
                weight: 0.25,
            },
        ],
        final_output: "Counter stopped early or opponent slowed long enough for block to reset",
    }
}
