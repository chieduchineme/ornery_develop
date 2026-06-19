/**
 * Pattern Analyzer - Maps football patterns from pattern files to tactical parameters
 * Generates human-readable descriptions of match tactics
 */

export interface PatternParameters {
  name: string;
  description: string;
  pressureIntensity: number; // 0-100
  compactness: number; // 0-100 (defensive spacing)
  aggressiveness: number; // 0-100
  directness: number; // 0-100 (direct vs possession play)
  width: number; // 0-100 (wide vs central)
  depth: number; // 0-100 (how spread out vertically)
  offTheOallAggression: number; // 0-100
}

// Pattern library - maps pattern names to behavioral parameters
const ATTACKING_PATTERNS: Record<string, PatternParameters> = {
  "Counter-Attack": {
    name: "Counter-Attack",
    description: "Quick transitions from defense to attack",
    pressureIntensity: 20,
    compactness: 40,
    aggressiveness: 85,
    directness: 90,
    width: 50,
    depth: 60,
    offTheOallAggression: 30,
  },
  "Fast Break": {
    name: "Fast Break",
    description: "Immediate attack after turnover with few touches",
    pressureIntensity: 15,
    compactness: 35,
    aggressiveness: 90,
    directness: 95,
    width: 55,
    depth: 75,
    offTheOallAggression: 25,
  },
  "Possession-Based": {
    name: "Possession-Based",
    description: "Patient build-up with many passes, controlling space",
    pressureIntensity: 10,
    compactness: 55,
    aggressiveness: 30,
    directness: 15,
    width: 70,
    depth: 50,
    offTheOallAggression: 40,
  },
  "Direct Attack": {
    name: "Direct Attack",
    description: "Long passes forward to reach attackers quickly",
    pressureIntensity: 20,
    compactness: 45,
    aggressiveness: 70,
    directness: 85,
    width: 60,
    depth: 65,
    offTheOallAggression: 35,
  },
  "Wing Play": {
    name: "Wing Play",
    description: "Attacking through wide areas with crosses",
    pressureIntensity: 25,
    compactness: 50,
    aggressiveness: 60,
    directness: 65,
    width: 90,
    depth: 55,
    offTheOallAggression: 45,
  },
  "Central Attack": {
    name: "Central Attack",
    description: "Direct penetration through the middle",
    pressureIntensity: 30,
    compactness: 40,
    aggressiveness: 75,
    directness: 75,
    width: 30,
    depth: 60,
    offTheOallAggression: 55,
  },
  "Overload Attack": {
    name: "Overload Attack",
    description: "Creating numerical superiority in one area",
    pressureIntensity: 35,
    compactness: 60,
    aggressiveness: 65,
    directness: 55,
    width: 50,
    depth: 55,
    offTheOallAggression: 60,
  },
  "Combination Play": {
    name: "Combination Play",
    description: "Quick one-twos and triangles to break lines",
    pressureIntensity: 20,
    compactness: 50,
    aggressiveness: 50,
    directness: 40,
    width: 60,
    depth: 50,
    offTheOallAggression: 50,
  },
  "High Press Attack": {
    name: "High Press Attack",
    description: "Winning possession high and attacking immediately",
    pressureIntensity: 90,
    compactness: 50,
    aggressiveness: 85,
    directness: 80,
    width: 65,
    depth: 70,
    offTheOallAggression: 85,
  },
  "Positional Play": {
    name: "Positional Play",
    description: "Structured attacking with players in specific spaces",
    pressureIntensity: 15,
    compactness: 65,
    aggressiveness: 45,
    directness: 25,
    width: 75,
    depth: 55,
    offTheOallAggression: 50,
  },
  "Rotational Attack": {
    name: "Rotational Attack",
    description: "Frequent position swaps to confuse defenders",
    pressureIntensity: 25,
    compactness: 55,
    aggressiveness: 55,
    directness: 40,
    width: 65,
    depth: 60,
    offTheOallAggression: 55,
  },
  "Total Football": {
    name: "Total Football",
    description: "Fluid movement with constant position interchanges",
    pressureIntensity: 30,
    compactness: 60,
    aggressiveness: 60,
    directness: 40,
    width: 70,
    depth: 70,
    offTheOallAggression: 65,
  },
};

const DEFENSIVE_PATTERNS: Record<string, PatternParameters> = {
  "High Press": {
    name: "High Press",
    description: "Press opponents near their own goal",
    pressureIntensity: 95,
    compactness: 55,
    aggressiveness: 90,
    directness: 30,
    width: 65,
    depth: 65,
    offTheOallAggression: 90,
  },
  "Mid-Block": {
    name: "Mid-Block",
    description: "Defend from middle third with compact spacing",
    pressureIntensity: 60,
    compactness: 75,
    aggressiveness: 55,
    directness: 25,
    width: 65,
    depth: 50,
    offTheOallAggression: 60,
  },
  "Low Block": {
    name: "Low Block",
    description: "Deep defense near penalty area",
    pressureIntensity: 40,
    compactness: 85,
    aggressiveness: 30,
    directness: 20,
    width: 70,
    depth: 40,
    offTheOallAggression: 35,
  },
  "Counter-Press": {
    name: "Counter-Press",
    description: "Immediate pressure after losing the ball",
    pressureIntensity: 85,
    compactness: 60,
    aggressiveness: 85,
    directness: 35,
    width: 60,
    depth: 60,
    offTheOallAggression: 80,
  },
  "Man-Oriented Pressing": {
    name: "Man-Oriented Pressing",
    description: "Defenders follow direct opponents tightly",
    pressureIntensity: 75,
    compactness: 40,
    aggressiveness: 70,
    directness: 30,
    width: 60,
    depth: 65,
    offTheOallAggression: 70,
  },
  "Zonal Defending": {
    name: "Zonal Defending",
    description: "Players protect zones rather than opponents",
    pressureIntensity: 65,
    compactness: 70,
    aggressiveness: 50,
    directness: 25,
    width: 65,
    depth: 55,
    offTheOallAggression: 55,
  },
  "Hybrid Defending": {
    name: "Hybrid Defending",
    description: "Combines zonal structure with man-oriented pressure",
    pressureIntensity: 70,
    compactness: 70,
    aggressiveness: 60,
    directness: 25,
    width: 65,
    depth: 60,
    offTheOallAggression: 65,
  },
  "Compactness Defense": {
    name: "Compactness Defense",
    description: "Team spacing shrinks to prevent central progression",
    pressureIntensity: 50,
    compactness: 90,
    aggressiveness: 45,
    directness: 20,
    width: 50,
    depth: 45,
    offTheOallAggression: 45,
  },
  "Pressing Traps": {
    name: "Pressing Traps",
    description: "Leave passing option open, then collapse",
    pressureIntensity: 80,
    compactness: 65,
    aggressiveness: 75,
    directness: 35,
    width: 60,
    depth: 60,
    offTheOallAggression: 75,
  },
  "Touchline Press": {
    name: "Touchline Press",
    description: "Use sideline as extra defender",
    pressureIntensity: 85,
    compactness: 70,
    aggressiveness: 80,
    directness: 35,
    width: 40,
    depth: 65,
    offTheOallAggression: 80,
  },
  "Back Five Defending": {
    name: "Back Five Defending",
    description: "Five-player defensive line for width protection",
    pressureIntensity: 55,
    compactness: 80,
    aggressiveness: 40,
    directness: 25,
    width: 85,
    depth: 45,
    offTheOallAggression: 40,
  },
  "Box Defending": {
    name: "Box Defending",
    description: "Prioritize defending the penalty area",
    pressureIntensity: 50,
    compactness: 85,
    aggressiveness: 35,
    directness: 20,
    width: 75,
    depth: 35,
    offTheOallAggression: 35,
  },
  "Offside Trap": {
    name: "Offside Trap",
    description: "Defensive line steps up for offside catches",
    pressureIntensity: 70,
    compactness: 65,
    aggressiveness: 65,
    directness: 30,
    width: 65,
    depth: 75,
    offTheOallAggression: 60,
  },
  "Funnel Defending": {
    name: "Funnel Defending",
    description: "Guide opponent to less dangerous zones",
    pressureIntensity: 55,
    compactness: 75,
    aggressiveness: 45,
    directness: 25,
    width: 70,
    depth: 50,
    offTheOallAggression: 50,
  },
};

/**
 * Get attack pattern parameters based on team state
 */
export function getAttackingPattern(
  playStyle: string,
  possession: boolean,
  ballZone: string,
  possession_pct: number
): PatternParameters {
  // Determine pattern based on play style and ball position
  const styleMap: Record<string, string> = {
    Attacking: "High Press Attack",
    Counter: "Counter-Attack",
    Balanced: "Combination Play",
    Possession: "Possession-Based",
    HighPress: "High Press Attack",
    Defensive: "Direct Attack", // Even in defense, if attacking switch to direct
  };

  const basePattern = styleMap[playStyle] || "Possession-Based";
  let pattern = ATTACKING_PATTERNS[basePattern] || ATTACKING_PATTERNS["Possession-Based"];

  // Modify based on ball position
  if (ballZone.includes("attacking")) {
    // Increase aggressiveness in attacking third
    return {
      ...pattern,
      aggressiveness: Math.min(100, pattern.aggressiveness + 15),
      directness: Math.min(100, pattern.directness + 10),
    };
  }

  if (ballZone.includes("defensive")) {
    // More likely to play direct or counter when near own goal
    return {
      ...pattern,
      aggressiveness: Math.max(0, pattern.aggressiveness - 20),
      directness: Math.min(100, pattern.directness + 20),
    };
  }

  return pattern;
}

/**
 * Get defense pattern parameters based on team state
 */
export function getDefensivePattern(
  playStyle: string,
  possession: boolean,
  ballZone: string,
  possession_pct: number
): PatternParameters {
  const styleMap: Record<string, string> = {
    Attacking: "Counter-Press",
    Counter: "Counter-Press",
    Balanced: "Hybrid Defending",
    Possession: "Mid-Block",
    HighPress: "High Press",
    Defensive: "Low Block",
  };

  const basePattern = styleMap[playStyle] || "Mid-Block";
  let pattern = DEFENSIVE_PATTERNS[basePattern] || DEFENSIVE_PATTERNS["Mid-Block"];

  // Adjust based on possession
  if (possession_pct < 35) {
    // Defending with majority of play in own half
    return {
      ...pattern,
      compactness: Math.min(100, pattern.compactness + 10),
      pressureIntensity: Math.max(0, pattern.pressureIntensity - 15),
    };
  }

  if (possession_pct > 65) {
    // Team has more freedom to press
    return {
      ...pattern,
      pressureIntensity: Math.min(100, pattern.pressureIntensity + 15),
    };
  }

  return pattern;
}

/**
 * Generate human-friendly tactical description for a team
 */
export function generateTacticalDescription(
  teamName: string,
  isAttacking: boolean,
  playStyle: string,
  formation: string,
  pattern: PatternParameters,
  ballZone: string
): string {
  const getIntensityLabel = (intensity: number): string => {
    if (intensity > 70) return "aggressive";
    if (intensity > 40) return "measured";
    return "deep";
  };

  const getPlayStyle = (directness: number): string => {
    if (directness > 70) return "direct";
    if (directness > 40) return "balanced";
    return "possession";
  };

  const getWidthStyle = (width: number): string => {
    if (width > 70) return "wide";
    if (width > 40) return "central";
    return "narrow";
  };

  const intensity = getIntensityLabel(pattern.pressureIntensity);
  const play = getPlayStyle(pattern.directness);
  const width = getWidthStyle(pattern.width);

  if (isAttacking) {
    const descriptions = [
      `${teamName} playing ${play} ${pattern.name}`,
      `${teamName} (${formation}) building with ${pattern.name}`,
      `${teamName} attacking in the ${ballZone}`,
      `${teamName} pushing forward with ${play} passes`,
      `${teamName} exploiting the ${width} areas`,
    ];

    return descriptions[Math.floor(Math.random() * descriptions.length)];
  } else {
    const descriptions = [
      `${teamName} defending with a ${intensity} ${pattern.name}`,
      `${teamName} (${formation}) maintaining ${pattern.name} shape`,
      `${teamName} defending in the ${ballZone}`,
      `${teamName} pressing ${intensity}ly`,
      `${teamName} holding a ${intensity} block`,
    ];

    return descriptions[Math.floor(Math.random() * descriptions.length)];
  }
}

/**
 * Get a specific movement instruction for a player based on pattern and position
 */
export function getMovementInstruction(
  pattern: PatternParameters,
  position: string,
  hasBall: boolean,
  timeInPossession: number
): string {
  if (hasBall) {
    if (pattern.directness > 75) return "Pass forward quickly";
    if (pattern.directness > 50) return "Look for forward option";
    return "Circulate possession";
  }

  // Off-ball movement based on pattern
  if (pattern.offTheOallAggression > 70) {
    return "Make aggressive run forward";
  }

  if (pattern.offTheOallAggression > 40) {
    return "Move to create passing angle";
  }

  return "Hold position";
}

/**
 * Analyze match state and return primary pattern description
 */
// Map from Rust pattern IDs → ATTACKING_PATTERNS keys
const PATTERN_ID_TO_KEY: Record<string, string> = {
  counter_attack: "Counter-Attack",
  fast_breaks: "Fast Break",
  possession_based: "Possession-Based",
  direct_attack: "Direct Attack",
  wing_play: "Wing Play",
  central_attack: "Central Attack",
  combination_crossing: "Crossing Attack",
  combination_play: "Combination Play",
  overload_attack: "Overload Attack",
  switch_of_play: "Switch of Play",
  high_press_attack: "High Press",
  positional_play: "Positional Play",
  rotational_attack: "Rotational Attack",
  total_football: "Total Football",
  overlapping_attack: "Wing Play",
  underlapping_attack: "Wing Play",
  isolation_attack: "Isolation Attack",
  set_piece_attack: "Set Piece",
  third_man_attack: "Third Man Run",
  crossing_variations: "Crossing Attack",
};

export function analyzePrimaryPattern(params: {
  homePlayStyle: string;
  awayPlayStyle: string;
  possession: "Home" | "Away";
  ballZone: string;
  homePossession_pct: number;
  awayPossession_pct: number;
  formation_home: string;
  formation_away: string;
  activeHomePattern?: string;
  activeAwayPattern?: string;
}): {
  home: string;
  away: string;
  homePattern: PatternParameters;
  awayPattern: PatternParameters;
} {
  const isHomeAttacking = params.possession === "Home";
  const isAwayAttacking = params.possession === "Away";

  // Prefer the live engine pattern id over inferring from play style
  const homeLookupKey = params.activeHomePattern
    ? PATTERN_ID_TO_KEY[params.activeHomePattern]
    : undefined;
  const awayLookupKey = params.activeAwayPattern
    ? PATTERN_ID_TO_KEY[params.activeAwayPattern]
    : undefined;

  const homePattern = (homeLookupKey && ATTACKING_PATTERNS[homeLookupKey])
    ? ATTACKING_PATTERNS[homeLookupKey]
    : isHomeAttacking
      ? getAttackingPattern(params.homePlayStyle, true, params.ballZone, params.homePossession_pct)
      : getDefensivePattern(params.homePlayStyle, false, params.ballZone, params.homePossession_pct);

  const awayPattern = (awayLookupKey && ATTACKING_PATTERNS[awayLookupKey])
    ? ATTACKING_PATTERNS[awayLookupKey]
    : isAwayAttacking
      ? getAttackingPattern(params.awayPlayStyle, true, params.ballZone, params.awayPossession_pct)
      : getDefensivePattern(params.awayPlayStyle, false, params.ballZone, params.awayPossession_pct);

  const homeDesc = generateTacticalDescription(
    "Home",
    isHomeAttacking,
    params.activeHomePattern
      ? (homeLookupKey ?? params.homePlayStyle)
      : params.homePlayStyle,
    params.formation_home,
    homePattern,
    params.ballZone
  );

  const awayDesc = generateTacticalDescription(
    "Away",
    isAwayAttacking,
    params.activeAwayPattern
      ? (awayLookupKey ?? params.awayPlayStyle)
      : params.awayPlayStyle,
    params.formation_away,
    awayPattern,
    params.ballZone
  );

  return {
    home: homeDesc,
    away: awayDesc,
    homePattern,
    awayPattern,
  };
}
