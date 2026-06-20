use log::info;
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;

pub use crate::application::live_match::FinishLiveMatchResponse;
use crate::application::live_match::{
    apply_match_command as apply_match_command_service,
    finish_live_match as finish_live_match_service,
    get_match_snapshot as get_match_snapshot_service,
    get_spatial_frames as get_spatial_frames_service,
    start_live_match as start_live_match_service,
    step_live_match as step_live_match_service,
};
use crate::application::team_talk::apply_team_talk as apply_team_talk_service;
use ofm_core::game::Game;
use ofm_core::state::StateManager;

#[derive(Debug, Deserialize)]
pub struct PressConferenceAnswer {
    question_id: String,
    response_id: String,
    #[serde(rename = "response_tone")]
    _response_tone: String,
    response_text: String,
    #[serde(default)]
    response_text_key: String,
    #[serde(default)]
    response_text_params: HashMap<String, String>,
    question_text: String,
    #[serde(default)]
    player_id: String,
}

#[derive(Debug, Serialize)]
struct LocalizedPressQuote {
    #[serde(skip_serializing_if = "String::is_empty")]
    key: String,
    fallback: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    params: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpectatorReplayPlayer {
    id: String,
    name: String,
    shirt_number: u8,
    position: String,
    side: engine::Side,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpectatorReplayPoint {
    x: f64,
    y: f64,
    active: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpectatorReplayFrame {
    minute: u8,
    phase: engine::MatchPhase,
    possession: engine::Side,
    ball_zone: engine::Zone,
    ball_x: f64,
    ball_y: f64,
    players: HashMap<String, SpectatorReplayPoint>,
    events: Vec<engine::MatchEvent>,
    home_score: u8,
    away_score: u8,
    active_home_pattern: Option<String>,
    active_away_pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpectatorReplayMetadata {
    chunk_count: usize,
    chunk_duration_minutes: u8,
    total_minutes: u8,
    players: Vec<SpectatorReplayPlayer>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpectatorReplayChunk {
    chunk_number: usize,
    frames: Vec<SpectatorReplayFrame>,
}

// ---------------------------------------------------------------------------
// Live Match Commands
// ---------------------------------------------------------------------------

fn finish_live_match_internal(state: &StateManager) -> Result<FinishLiveMatchResponse, String> {
    finish_live_match_service(state)
}

fn apply_team_talk_internal(
    game: &mut Game,
    tone: &str,
    context: &str,
    seed: u64,
) -> Result<Vec<serde_json::Value>, String> {
    apply_team_talk_service(game, tone, context, seed)
}

/// Start a live match for a given fixture.
/// mode: "live" | "spectator" | "instant"
#[tauri::command]
pub fn start_live_match(
    state: State<'_, StateManager>,
    fixture_index: usize,
    mode: String,
    allows_extra_time: bool,
) -> Result<engine::MatchSnapshot, String> {
    start_live_match_service(&state, fixture_index, &mode, allows_extra_time)
}

/// Step the live match forward by N minutes. Returns the events from each minute.
#[tauri::command]
pub fn step_live_match(
    state: State<'_, StateManager>,
    minutes: u16,
) -> Result<Vec<engine::MinuteResult>, String> {
    step_live_match_service(&state, minutes)
}

/// Apply a match command (substitution, tactic change, set piece taker, etc.)
#[tauri::command]
pub fn apply_match_command(
    state: State<'_, StateManager>,
    command: engine::MatchCommand,
) -> Result<engine::MatchSnapshot, String> {
    apply_match_command_service(&state, command)
}

/// Get current match snapshot without advancing time.
#[tauri::command]
pub fn get_match_snapshot(state: State<'_, StateManager>) -> Result<engine::MatchSnapshot, String> {
    get_match_snapshot_service(&state)
}

/// Return replay metadata for the active live match.
#[tauri::command]
pub fn get_spectator_replay_metadata(
    state: State<'_, StateManager>,
) -> Result<SpectatorReplayMetadata, String> {
    let snapshot = get_match_snapshot_service(&state)?;
    Ok(build_spectator_replay_metadata(&snapshot))
}

/// Return one compact replay chunk derived from current match events and lineups.
#[tauri::command]
pub fn get_spectator_replay_chunk(
    state: State<'_, StateManager>,
    chunk_number: usize,
) -> Result<SpectatorReplayChunk, String> {
    let snapshot = get_match_snapshot_service(&state)?;
    let spatial_frames = get_spatial_frames_service(&state);
    build_spectator_replay_chunk(&snapshot, &spatial_frames, chunk_number)
}

/// Finish the live match: generate report, update game state, clean up.
#[tauri::command]
pub fn finish_live_match(
    state: State<'_, StateManager>,
) -> Result<FinishLiveMatchResponse, String> {
    finish_live_match_internal(&state)
}

const REPLAY_CHUNK_MINUTES: u8 = 5;

fn build_spectator_replay_metadata(snapshot: &engine::MatchSnapshot) -> SpectatorReplayMetadata {
    let total_minutes = snapshot.current_minute.max(1);
    let chunk_count = ((total_minutes as usize) / REPLAY_CHUNK_MINUTES as usize) + 1;
    SpectatorReplayMetadata {
        chunk_count,
        chunk_duration_minutes: REPLAY_CHUNK_MINUTES,
        total_minutes,
        players: collect_replay_players(snapshot),
    }
}

fn build_spectator_replay_chunk(
    snapshot: &engine::MatchSnapshot,
    spatial_frames: &[engine::spatial::SpatialFrame],
    chunk_number: usize,
) -> Result<SpectatorReplayChunk, String> {
    let start = chunk_number
        .checked_mul(REPLAY_CHUNK_MINUTES as usize)
        .ok_or_else(|| "be.error.liveMatch.invalidReplayChunk".to_string())?;
    let end = (start + REPLAY_CHUNK_MINUTES as usize).min(snapshot.current_minute as usize);
    if start > snapshot.current_minute as usize {
        return Err("be.error.liveMatch.replayChunkOutOfRange".to_string());
    }

    let frames = (start..=end)
        .map(|minute| {
            let sf = spatial_frames.iter().rev().find(|f| f.minute == minute as u8);
            build_spectator_replay_frame(snapshot, sf, minute as u8)
        })
        .collect();

    Ok(SpectatorReplayChunk {
        chunk_number,
        frames,
    })
}

fn collect_replay_players(snapshot: &engine::MatchSnapshot) -> Vec<SpectatorReplayPlayer> {
    let mut players = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for (side, squad, bench) in [
        (engine::Side::Home, &snapshot.home_team.players, &snapshot.home_bench),
        (engine::Side::Away, &snapshot.away_team.players, &snapshot.away_bench),
    ] {
        for player in squad.iter().chain(bench.iter()) {
            if !seen.insert(player.id.clone()) {
                continue;
            }
            players.push(SpectatorReplayPlayer {
                id: player.id.clone(),
                name: player.name.clone(),
                shirt_number: shirt_number(players.len(), side),
                position: format!("{:?}", player.position),
                side,
            });
        }
    }

    players
}

fn build_spectator_replay_frame(
    snapshot: &engine::MatchSnapshot,
    spatial: Option<&engine::spatial::SpatialFrame>,
    minute: u8,
) -> SpectatorReplayFrame {
    let events = snapshot
        .events
        .iter()
        .filter(|event| event.minute == minute)
        .cloned()
        .collect::<Vec<_>>();
    let (home_score, away_score) = score_at_minute(snapshot, minute);
    let possession = possession_at_minute(snapshot, minute);
    let ball_zone = zone_at_minute(snapshot, minute);
    let active_pattern = match possession {
        engine::Side::Home => snapshot.active_home_pattern.as_deref(),
        engine::Side::Away => snapshot.active_away_pattern.as_deref(),
    };

    // Use physics-derived ball position if available, else fall back to zone heuristic
    let (ball_x, ball_y) = if let Some(sf) = spatial {
        (sf.ball_x, sf.ball_y)
    } else {
        ball_coordinates(ball_zone, possession, minute, active_pattern)
    };

    let mut players = HashMap::new();
    add_side_points(
        &mut players,
        snapshot,
        engine::Side::Home,
        &snapshot.home_team.players,
        &snapshot.home_bench,
        minute,
        snapshot.active_home_pattern.as_deref(),
        spatial,
    );
    add_side_points(
        &mut players,
        snapshot,
        engine::Side::Away,
        &snapshot.away_team.players,
        &snapshot.away_bench,
        minute,
        snapshot.active_away_pattern.as_deref(),
        spatial,
    );

    SpectatorReplayFrame {
        minute,
        phase: phase_at_minute(snapshot, minute),
        possession,
        ball_zone,
        ball_x,
        ball_y,
        players,
        events,
        home_score,
        away_score,
        active_home_pattern: snapshot.active_home_pattern.clone(),
        active_away_pattern: snapshot.active_away_pattern.clone(),
    }
}

fn add_side_points(
    out: &mut HashMap<String, SpectatorReplayPoint>,
    snapshot: &engine::MatchSnapshot,
    side: engine::Side,
    squad: &[engine::PlayerData],
    bench: &[engine::PlayerData],
    minute: u8,
    pattern_id: Option<&str>,
    spatial: Option<&engine::spatial::SpatialFrame>,
) {
    let mut active_players = squad
        .iter()
        .filter(|player| is_player_active(snapshot, side, &player.id, minute))
        .collect::<Vec<_>>();
    active_players.sort_by_key(|player| position_rank(&format!("{:?}", player.position)));

    if active_players.len() < 11 {
        for player in bench {
            if active_players.len() >= 11 {
                break;
            }
            if active_players.iter().any(|active| active.id == player.id) {
                continue;
            }
            active_players.push(player);
        }
    }

    for (index, player) in active_players.iter().take(11).enumerate() {
        // Prefer physics-derived position; fall back to heuristic
        let (x, y) = if let Some(sf) = spatial {
            sf.players.get(&player.id).copied().unwrap_or_else(|| {
                player_coordinates(side, &format!("{:?}", player.position), index, minute, pattern_id)
            })
        } else {
            player_coordinates(side, &format!("{:?}", player.position), index, minute, pattern_id)
        };
        out.insert(
            player.id.clone(),
            SpectatorReplayPoint { x, y, active: true },
        );
    }
}

fn is_player_active(
    snapshot: &engine::MatchSnapshot,
    side: engine::Side,
    player_id: &str,
    minute: u8,
) -> bool {
    for substitution in &snapshot.substitutions {
        if substitution.side != side {
            continue;
        }
        if substitution.player_on_id == player_id && minute < substitution.minute {
            return false;
        }
        if substitution.player_off_id == player_id && minute >= substitution.minute {
            return false;
        }
    }
    true
}

fn score_at_minute(snapshot: &engine::MatchSnapshot, minute: u8) -> (u8, u8) {
    let mut home = 0;
    let mut away = 0;
    for event in snapshot
        .events
        .iter()
        .filter(|event| event.minute <= minute && event.is_goal())
    {
        match event.side {
            engine::Side::Home => home += 1,
            engine::Side::Away => away += 1,
        }
    }
    (home.min(snapshot.home_score), away.min(snapshot.away_score))
}

fn possession_at_minute(snapshot: &engine::MatchSnapshot, minute: u8) -> engine::Side {
    snapshot
        .events
        .iter()
        .rev()
        .find(|event| event.minute <= minute)
        .map(|event| event.side)
        .unwrap_or(snapshot.possession)
}

fn zone_at_minute(snapshot: &engine::MatchSnapshot, minute: u8) -> engine::Zone {
    snapshot
        .events
        .iter()
        .rev()
        .find(|event| event.minute <= minute)
        .map(|event| event.zone)
        .unwrap_or(snapshot.ball_zone)
}

fn phase_at_minute(snapshot: &engine::MatchSnapshot, minute: u8) -> engine::MatchPhase {
    if minute >= snapshot.current_minute {
        return snapshot.phase;
    }
    if minute == 0 {
        engine::MatchPhase::PreKickOff
    } else if minute < 45 {
        engine::MatchPhase::FirstHalf
    } else if minute == 45 {
        engine::MatchPhase::HalfTime
    } else {
        engine::MatchPhase::SecondHalf
    }
}

fn ball_coordinates(
    zone: engine::Zone,
    possession: engine::Side,
    minute: u8,
    pattern_id: Option<&str>,
) -> (f64, f64) {
    let x = match zone {
        engine::Zone::HomeBox => 12.0,
        engine::Zone::HomeDefense => 28.0,
        engine::Zone::Midfield => 50.0,
        engine::Zone::AwayDefense => 72.0,
        engine::Zone::AwayBox => 88.0,
    };
    // Pattern-aware lateral spread: wide patterns pull ball wider, central keep it tight.
    let y_scale = match pattern_id {
        Some("wing_play") | Some("overlapping_attack") | Some("underlapping_attack")
        | Some("combination_crossing") | Some("crossing_variations") | Some("overload_attack")
        | Some("switch_of_play") => 1.6,
        Some("central_attack") | Some("isolation_attack") | Some("positional_play")
        | Some("possession_based") => 0.6,
        Some("high_press_attack") | Some("fast_breaks") | Some("counter_attack") => 1.0,
        _ => 1.0,
    };
    let drift = ((((minute as u16 * 37) % 34) as f64) - 17.0) * y_scale;
    let y = if possession == engine::Side::Home {
        50.0 + drift
    } else {
        50.0 - drift
    }
    .clamp(12.0, 88.0);
    (x, y)
}

fn player_coordinates(
    side: engine::Side,
    position: &str,
    index: usize,
    minute: u8,
    pattern_id: Option<&str>,
) -> (f64, f64) {
    // Base x per position — adjust depth based on pattern
    let x_push: f64 = match pattern_id {
        Some("high_press_attack") => 6.0,   // defenders push up
        Some("counter_attack") | Some("fast_breaks") => -4.0, // hold shape, hit on break
        Some("possession_based") | Some("positional_play") => 3.0,  // compact but advanced
        _ => 0.0,
    };
    let (base_x, spread) = match (side, position) {
        (engine::Side::Home, "Goalkeeper") => (8.0, 0.0),
        (engine::Side::Home, "Defender")   => ((24.0 + x_push).clamp(14.0, 40.0), 18.0),
        (engine::Side::Home, "Midfielder") => ((46.0 + x_push * 0.5).clamp(30.0, 58.0), 16.0),
        (engine::Side::Home, "Forward")    => ((70.0 + x_push * 0.3).clamp(55.0, 82.0), 14.0),
        (engine::Side::Away, "Goalkeeper") => (92.0, 0.0),
        (engine::Side::Away, "Defender")   => ((76.0 - x_push).clamp(60.0, 86.0), 18.0),
        (engine::Side::Away, "Midfielder") => ((54.0 - x_push * 0.5).clamp(42.0, 70.0), 16.0),
        (engine::Side::Away, "Forward")    => ((30.0 - x_push * 0.3).clamp(18.0, 45.0), 14.0),
        _ => (50.0, 15.0),
    };
    // Widen spread for wing patterns, compress for central
    let spread = match pattern_id {
        Some("wing_play") | Some("overlapping_attack") | Some("overload_attack") => spread * 1.35,
        Some("central_attack") | Some("positional_play") => spread * 0.65,
        _ => spread,
    };
    let row_index = index % 5;
    let movement = ((((minute as usize + index * 7) % 11) as f64) - 5.0) * 0.65;
    let vertical_drift = ((((minute as usize * 3 + index * 5) % 9) as f64) - 4.0) * 0.55;
    let y = (if spread == 0.0 {
        50.0
    } else {
        50.0 + ((row_index as f64 - 2.0) * spread)
    } + vertical_drift)
        .clamp(8.0, 92.0);
    ((base_x + movement).clamp(5.0, 95.0), y)
}

fn position_rank(position: &str) -> u8 {
    match position {
        "Goalkeeper" => 0,
        "Defender" => 1,
        "Midfielder" => 2,
        "Forward" => 3,
        _ => 4,
    }
}

fn shirt_number(index: usize, side: engine::Side) -> u8 {
    let offset = if side == engine::Side::Home { 1 } else { 12 };
    ((index % 23) + offset).min(99) as u8
}

/// Apply a team talk and return per-player morale changes.
/// tone: "calm" | "motivational" | "assertive" | "aggressive" | "praise" | "disappointed"
/// context: "winning" | "losing" | "drawing"
#[tauri::command]
pub fn apply_team_talk(
    state: State<'_, StateManager>,
    tone: String,
    context: String,
) -> Result<Vec<serde_json::Value>, String> {
    info!("[cmd] apply_team_talk: tone={}, context={}", tone, context);
    let mut game = state
        .get_game(|g| g.clone())
        .ok_or("be.error.noActiveGameSession")?;
    let seed = rand::rng().random::<u64>();
    let results = apply_team_talk_internal(&mut game, &tone, &context, seed)?;

    state.set_game(game);
    Ok(results)
}

/// Process press conference answers: generate news article, affect squad morale.
#[tauri::command]
pub fn submit_press_conference(
    state: State<'_, StateManager>,
    answers: Vec<PressConferenceAnswer>,
    home_team: String,
    away_team: String,
    home_score: u8,
    away_score: u8,
    user_team_name: String,
    user_team_id: String,
    _prerendered_body: Option<String>,
    _prerendered_headline: Option<String>,
) -> Result<serde_json::Value, String> {
    info!(
        "[cmd] submit_press_conference: {} {} - {} {}",
        home_team, home_score, away_score, away_team
    );
    let mut game = state
        .get_game(|g| g.clone())
        .ok_or("be.error.noActiveGameSession")?;

    let today = game.clock.current_date.format("%Y-%m-%d").to_string();
    let mut rng = rand::rng();

    // Build news article from press conference answers
    let mut quotes: Vec<String> = Vec::new();
    let mut localized_quotes: Vec<LocalizedPressQuote> = Vec::new();
    let mut morale_delta: i16 = 0;
    let mut mentioned_player_ids: Vec<String> = Vec::new();

    for answer in &answers {
        let rid = answer.response_id.as_str();
        let text = answer.response_text.as_str();
        let qid = answer.question_id.as_str();

        let _ = &answer.question_text;

        if !text.is_empty() {
            quotes.push(format!("\"{}\"", text));
            localized_quotes.push(LocalizedPressQuote {
                key: answer.response_text_key.clone(),
                fallback: text.to_string(),
                params: answer.response_text_params.clone(),
            });
        }

        // Track player mentions
        if !answer.player_id.is_empty() {
            mentioned_player_ids.push(answer.player_id.clone());
        }

        // Morale effects based on stable response identifiers.
        match rid {
            "humble" | "fair" | "positive" | "focused" | "grateful" | "patience" | "appreciate"
            | "understand" => morale_delta += rng.random_range(1..=3),
            "confident" | "ambitious" | "shared" => morale_delta += rng.random_range(2..=5),
            "defiant" | "frustrated" => morale_delta += rng.random_range(-2..=2),
            "curt" | "evasive" => morale_delta += rng.random_range(-3..=0),
            "accept" | "detailed" | "apologize" => morale_delta += rng.random_range(0..=2),
            "deflect" => morale_delta += rng.random_range(-1..=1),
            "praise" => morale_delta += rng.random_range(3..=6),
            "demanding" => morale_delta += rng.random_range(-2..=3),
            _ => {}
        }

        // Player-focused question effects
        if qid == "player_focus" {
            if !answer.player_id.is_empty() {
                let player_delta: i16 = match rid {
                    "praise" => rng.random_range(4..=8),
                    "demanding" => rng.random_range(-3..=4),
                    "deflect" => rng.random_range(-2..=1),
                    _ => rng.random_range(0..=3),
                };
                if let Some(p) = game.players.iter_mut().find(|p| p.id == answer.player_id) {
                    p.morale = ((p.morale as i16) + player_delta).clamp(10, 100) as u8;
                }
            }
        }
    }

    // Apply squad-wide morale effect
    morale_delta = morale_delta.clamp(-8, 8);
    if morale_delta != 0 {
        for p in game.players.iter_mut() {
            if p.team_id.as_deref() == Some(&user_team_id) {
                p.morale = ((p.morale as i16) + morale_delta).clamp(10, 100) as u8;
            }
        }
    }

    // Generate news article
    let result_str = format!(
        "{} {} - {} {}",
        home_team, home_score, away_score, away_team
    );
    let headline_key = if quotes.is_empty() {
        ("be.news.pressConference.headlinePostMatch",)
    } else if rng.random::<bool>() {
        ("be.news.pressConference.headlineManagerQuote",)
    } else {
        ("be.news.pressConference.headlinePressConf",)
    }
    .0;

    let body_key = if quotes.len() > 1 {
        ("be.news.pressConference.bodyMultiple",)
    } else if quotes.len() == 1 {
        ("be.news.pressConference.bodySingle",)
    } else {
        ("be.news.pressConference.bodyNone",)
    }
    .0;

    let mut i18n_params = HashMap::new();
    i18n_params.insert("team".to_string(), user_team_name.clone());
    i18n_params.insert("result".to_string(), result_str.clone());
    if !localized_quotes.is_empty() {
        if let Ok(serialized_quotes) = serde_json::to_string(&localized_quotes) {
            i18n_params.insert("quotesData".to_string(), serialized_quotes);
        }
        i18n_params.insert("quote".to_string(), quotes[0].trim_matches('"').to_string());
    }

    let article_id = format!("press_conf_{}", today);
    let article = domain::news::NewsArticle::new(
        article_id,
        String::new(),
        String::new(),
        String::new(),
        today.clone(),
        domain::news::NewsCategory::MatchReport,
    )
    .with_teams(vec![user_team_id.clone()])
    .with_players(mentioned_player_ids)
    .with_i18n(headline_key, body_key, "be.source.sportsDaily", i18n_params);

    game.news.push(article);
    state.set_game(game.clone());

    Ok(serde_json::json!({
        "game": game,
        "morale_delta": morale_delta
    }))
}

#[cfg(test)]
mod tests {
    use super::{apply_team_talk_internal, finish_live_match_internal};
    use chrono::{TimeZone, Utc};
    use domain::league::{Fixture, FixtureCompetition, FixtureStatus, League, StandingEntry};
    use domain::manager::Manager;
    use domain::player::{Player, PlayerAttributes, PlayerIssue, PlayerIssueCategory, Position};
    use domain::team::Team;
    use ofm_core::clock::GameClock;
    use ofm_core::game::Game;
    use ofm_core::live_match_manager::{self, MatchMode};
    use ofm_core::state::StateManager;

    fn default_attrs(position: Position) -> PlayerAttributes {
        let is_goalkeeper = matches!(position, Position::Goalkeeper);

        PlayerAttributes {
            pace: 65,
            stamina: 65,
            strength: 65,
            agility: 65,
            passing: 65,
            shooting: if is_goalkeeper { 30 } else { 65 },
            tackling: if is_goalkeeper { 30 } else { 65 },
            dribbling: if is_goalkeeper { 30 } else { 65 },
            defending: if is_goalkeeper { 30 } else { 65 },
            positioning: 65,
            vision: 65,
            decisions: 65,
            composure: 65,
            aggression: 50,
            teamwork: 65,
            leadership: 50,
            handling: if is_goalkeeper { 75 } else { 20 },
            reflexes: if is_goalkeeper { 75 } else { 20 },
            aerial: 60,
        }
    }

    fn make_player(id: &str, name: &str, team_id: &str, position: Position) -> Player {
        let mut player = Player::new(
            id.to_string(),
            name.to_string(),
            name.to_string(),
            "1995-01-01".to_string(),
            "England".to_string(),
            position.clone(),
            default_attrs(position),
        );
        player.team_id = Some(team_id.to_string());
        player.condition = 100;
        player.morale = 70;
        player
    }

    fn make_team(id: &str, name: &str) -> Team {
        Team::new(
            id.to_string(),
            name.to_string(),
            name[..3].to_string(),
            "England".to_string(),
            "London".to_string(),
            "Stadium".to_string(),
            40_000,
        )
    }

    fn make_squad(team_id: &str, prefix: &str) -> Vec<Player> {
        let mut players = Vec::new();
        players.push(make_player(
            &format!("{}_gk", prefix),
            &format!("{} GK", prefix),
            team_id,
            Position::Goalkeeper,
        ));
        for index in 0..4 {
            players.push(make_player(
                &format!("{}_def{}", prefix, index),
                &format!("{} Def{}", prefix, index),
                team_id,
                Position::Defender,
            ));
        }
        for index in 0..4 {
            players.push(make_player(
                &format!("{}_mid{}", prefix, index),
                &format!("{} Mid{}", prefix, index),
                team_id,
                Position::Midfielder,
            ));
        }
        for index in 0..2 {
            players.push(make_player(
                &format!("{}_fwd{}", prefix, index),
                &format!("{} Fwd{}", prefix, index),
                team_id,
                Position::Forward,
            ));
        }
        players
    }

    fn make_game_with_round() -> Game {
        let clock = GameClock::new(Utc.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap());
        let mut manager = Manager::new(
            "mgr1".to_string(),
            "Test".to_string(),
            "Manager".to_string(),
            "1980-01-01".to_string(),
            "England".to_string(),
        );
        manager.hire("team1".to_string());

        let teams = vec![
            make_team("team1", "Home FC"),
            make_team("team2", "Away FC"),
            make_team("team3", "Third FC"),
            make_team("team4", "Fourth FC"),
        ];
        let mut players = make_squad("team1", "t1");
        players.extend(make_squad("team2", "t2"));
        players.extend(make_squad("team3", "t3"));
        players.extend(make_squad("team4", "t4"));

        let league = League {
            id: "league1".to_string(),
            name: "Test League".to_string(),
            season: 1,
            fixtures: vec![
                Fixture {
                    id: "fix1".to_string(),
                    matchday: 1,
                    date: "2025-06-15".to_string(),
                    home_team_id: "team1".to_string(),
                    away_team_id: "team2".to_string(),
                    competition: FixtureCompetition::League,
                    status: FixtureStatus::Scheduled,
                    result: None,
                },
                Fixture {
                    id: "fix2".to_string(),
                    matchday: 1,
                    date: "2025-06-15".to_string(),
                    home_team_id: "team3".to_string(),
                    away_team_id: "team4".to_string(),
                    competition: FixtureCompetition::League,
                    status: FixtureStatus::Scheduled,
                    result: None,
                },
            ],
            standings: vec![
                StandingEntry::new("team1".to_string()),
                StandingEntry::new("team2".to_string()),
                StandingEntry::new("team3".to_string()),
                StandingEntry::new("team4".to_string()),
            ],
            transfer_log: vec![],
            transfer_rumours: vec![],
        };

        let mut game = Game::new(clock, manager, teams, players, vec![], vec![]);
        game.league = Some(league);
        game
    }

    fn delta_for(results: &[serde_json::Value], player_id: &str) -> i64 {
        results
            .iter()
            .find(|result| result["player_id"] == player_id)
            .and_then(|result| result["delta"].as_i64())
            .unwrap()
    }

    #[test]
    fn finish_live_match_returns_completed_round_summary_response() {
        let state = StateManager::new();
        let mut game = make_game_with_round();
        let today = game.clock.current_date.format("%Y-%m-%d").to_string();
        ofm_core::turn::simulate_other_matches(&mut game, &today, Some(0));

        let mut session =
            live_match_manager::create_live_match(&game, 0, MatchMode::Instant, false).unwrap();
        session.user_side = None;
        session.run_to_completion();

        state.set_game(game);
        state.set_live_match(session);

        let response = finish_live_match_internal(&state).expect("finish live match response");

        let round_summary = response.round_summary.expect("round summary response");
        assert!(round_summary.is_complete);
        assert_eq!(round_summary.pending_fixture_count, 0);
        assert_eq!(round_summary.completed_results.len(), 2);
        assert_eq!(
            response
                .game
                .clock
                .current_date
                .format("%Y-%m-%d")
                .to_string(),
            "2025-06-16"
        );
    }

    #[test]
    fn team_talk_reactions_vary_by_player_context() {
        let mut game = make_game_with_round();
        let composed = game
            .players
            .iter_mut()
            .find(|player| player.id == "t1_mid0")
            .unwrap();
        composed.attributes.composure = 90;
        composed.attributes.leadership = 90;
        composed.attributes.aggression = 20;
        composed.morale_core.manager_trust = 80;

        let volatile = game
            .players
            .iter_mut()
            .find(|player| player.id == "t1_fwd0")
            .unwrap();
        volatile.attributes.composure = 20;
        volatile.attributes.leadership = 20;
        volatile.attributes.aggression = 90;
        volatile.morale_core.manager_trust = 25;
        volatile.morale_core.unresolved_issue = Some(PlayerIssue {
            category: PlayerIssueCategory::Morale,
            severity: 70,
        });

        let results = apply_team_talk_internal(&mut game, "aggressive", "winning", 7).unwrap();

        assert!(delta_for(&results, "t1_mid0") > delta_for(&results, "t1_fwd0"));
    }

    #[test]
    fn repeating_same_team_talk_loses_effectiveness() {
        let mut game = make_game_with_round();
        let player = game
            .players
            .iter_mut()
            .find(|player| player.id == "t1_mid0")
            .unwrap();
        player.morale = 50;
        player.morale_core.manager_trust = 70;

        let first = apply_team_talk_internal(&mut game, "motivational", "losing", 13).unwrap();
        let second = apply_team_talk_internal(&mut game, "motivational", "losing", 13).unwrap();

        assert!(delta_for(&second, "t1_mid0") <= delta_for(&first, "t1_mid0"));
    }
}
