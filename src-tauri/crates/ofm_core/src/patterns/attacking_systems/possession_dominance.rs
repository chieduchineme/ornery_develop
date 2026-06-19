use engine::PlayStyle;
use crate::patterns::types::{EliteAttackingSystemDefinition, SystemPhase};

pub fn system() -> EliteAttackingSystemDefinition {
    EliteAttackingSystemDefinition {
        id: "possession_dominance",
        name: "Possession Dominance Engine",
        source_md: "Possession_Dominance_System.md",
        structural_idea: "Control space first → manipulate pressure → break lines with third-man runs",
        real_world_archetypes: &["Manchester City (Guardiola)", "Barcelona (Xavi era)", "Spain national team"],
        base_play_style: PlayStyle::Possession,
        phases: vec![
            SystemPhase {
                name: "Build-Up Positional Control",
                attacking_form_id: "positional_play",
                tactical_purpose: "Create structured superiority — CBs wide, DM drops, FBs invert into midfield lanes",
                entry_condition: "Team wins possession in own half or resets from GK",
                handoff: "Once opponent compacts and waits, trigger circulation trap",
                weight: 0.35,
            },
            SystemPhase {
                name: "Circulation Trap",
                attacking_form_id: "combination_play",
                tactical_purpose: "Lure opponent press onto one side with rapid one-touch combinations; create predictable overload to exploit",
                entry_condition: "Structured superiority established — opponent still passive or pressing lightly",
                handoff: "When opponent commits press, trigger third-man vertical break",
                weight: 0.35,
            },
            SystemPhase {
                name: "Vertical Break via Third Man",
                attacking_form_id: "third_man_attack",
                tactical_purpose: "Third-man run behind defense while opponent is mid-press — RW or ST exploits vacated channel",
                entry_condition: "Opponent overcommits to press after circulation trap; gap identified in defensive line",
                handoff: "Final output: cutback, through ball, or penalty-spot finish",
                weight: 0.30,
            },
        ],
        final_output: "Cutback from third-man arrival OR through ball to ST OR penalty-spot finish from CM late run",
    }
}
