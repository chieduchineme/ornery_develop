use engine::PlayStyle;
use crate::patterns::types::EliteDefensiveSystemDefinition;

pub mod compact_block_control;
pub mod deep_box_protection;
pub mod high_line_sweeper;
pub mod high_press_recovery;
pub mod hybrid_man_zonal_lock;
pub mod transition_shield;

pub fn all_systems() -> Vec<EliteDefensiveSystemDefinition> {
    vec![
        high_press_recovery::system(),
        compact_block_control::system(),
        deep_box_protection::system(),
        transition_shield::system(),
        hybrid_man_zonal_lock::system(),
        high_line_sweeper::system(),
    ]
}

pub fn system_by_id(id: &str) -> Option<EliteDefensiveSystemDefinition> {
    all_systems().into_iter().find(|s| s.id == id)
}

pub fn systems_for_play_style(style: PlayStyle) -> Vec<EliteDefensiveSystemDefinition> {
    all_systems().into_iter().filter(|s| s.base_play_style == style).collect()
}
