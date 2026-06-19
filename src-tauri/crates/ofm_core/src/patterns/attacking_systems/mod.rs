pub mod box_control_crossing;
pub mod explosive_transition;
pub mod flank_dismantling;
pub mod hybrid_press_to_attack;
pub mod possession_dominance;
pub mod rotational_chaos;

use crate::patterns::types::EliteAttackingSystemDefinition;

pub fn all_systems() -> Vec<EliteAttackingSystemDefinition> {
    vec![
        possession_dominance::system(),
        flank_dismantling::system(),
        explosive_transition::system(),
        rotational_chaos::system(),
        box_control_crossing::system(),
        hybrid_press_to_attack::system(),
    ]
}

pub fn system_by_id(id: &str) -> Option<EliteAttackingSystemDefinition> {
    all_systems().into_iter().find(|s| s.id == id)
}

/// Resolve which attacking systems naturally suit a given play style.
pub fn systems_for_play_style(style: engine::PlayStyle) -> Vec<EliteAttackingSystemDefinition> {
    all_systems()
        .into_iter()
        .filter(|s| s.base_play_style == style)
        .collect()
}
