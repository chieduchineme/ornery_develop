use crate::types::Side;
use super::vec2::Vec2;

/// Parse "4-3-3" into [4, 3, 3].
fn parse_counts(formation: &str) -> Vec<usize> {
    formation
        .split('-')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect()
}

/// Evenly spread N players along the Y axis (pitch width = 100 units).
fn spread_y(count: usize) -> Vec<f64> {
    match count {
        0 => vec![],
        1 => vec![50.0],
        2 => vec![28.0, 72.0],
        3 => vec![18.0, 50.0, 82.0],
        4 => vec![14.0, 36.0, 64.0, 86.0],
        5 => vec![10.0, 28.0, 50.0, 72.0, 90.0],
        n => (0..n)
            .map(|i| 10.0 + 80.0 * i as f64 / (n - 1).max(1) as f64)
            .collect(),
    }
}

/// Generate 11 formation base positions (pitch = 100×100 units).
/// Home attacks toward x = 100; Away attacks toward x = 0.
/// Order: GK first, then lines back → front.
pub fn base_positions(formation: &str, side: Side) -> Vec<Vec2> {
    let counts = parse_counts(formation);
    let mut positions = Vec::with_capacity(11);

    // GK
    let gk_x: f64 = if side == Side::Home { 7.0 } else { 93.0 };
    positions.push(Vec2::new(gk_x, 50.0));

    let total_lines = counts.len() as f64;
    for (i, &count) in counts.iter().enumerate() {
        // x fraction from own goal toward midfield (lines 0..N go 1/(N+1) .. N/(N+1))
        let frac = (i as f64 + 1.0) / (total_lines + 1.0);
        let x = if side == Side::Home {
            10.0 + 60.0 * frac   // home: from ~18 up to ~58
        } else {
            90.0 - 60.0 * frac   // away: from ~82 down to ~42
        };
        for y in spread_y(count) {
            positions.push(Vec2::new(x, y));
        }
    }

    // Pad to 11 with midfield fallbacks
    while positions.len() < 11 {
        let x = if side == Side::Home { 50.0 } else { 50.0 };
        positions.push(Vec2::new(x, 50.0));
    }
    positions.truncate(11);
    positions
}

/// Shift an x position toward the opponent's goal by `amount` units.
pub fn push_forward(x: f64, side: Side, amount: f64) -> f64 {
    if side == Side::Home {
        (x + amount).clamp(5.0, 93.0)
    } else {
        (x - amount).clamp(7.0, 95.0)
    }
}
