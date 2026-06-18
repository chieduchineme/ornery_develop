/**
 * Sub-minute match simulation - breaks down minute-level steps into seconds
 * Allows smooth animation and detailed pattern observation
 */

import { MatchSnapshot, SpectatorReplayFrame } from "../components/match/types";

export interface SubMinuteFrame {
  second: number; // 0-59
  minute: number;
  timestamp: number; // total seconds into match
  frame: SpectatorReplayFrame;
}

export interface MatchStateCache {
  minute: number;
  snapshot: MatchSnapshot;
  interpolatedFrames: Map<number, SpectatorReplayFrame>;
}

/**
 * Generate interpolated sub-minute frames between two match minutes
 * This creates smooth movement and allows pattern observation
 */
export function generateSubMinuteFrames(
  startFrame: SpectatorReplayFrame,
  endFrame: SpectatorReplayFrame,
  stepCount: number = 10 // subdivisions within a minute
): SubMinuteFrame[] {
  const frames: SubMinuteFrame[] = [];
  const secondsPerStep = 60 / stepCount;

  for (let i = 0; i <= stepCount; i++) {
    const progress = i / stepCount;
    const interpolated = interpolateFrame(startFrame, endFrame, progress);

    frames.push({
      second: Math.round((i * secondsPerStep) % 60),
      minute: endFrame.minute,
      timestamp: endFrame.minute * 60 + Math.round(i * secondsPerStep),
      frame: interpolated,
    });
  }

  return frames;
}

/**
 * Interpolate between two frame states with easing
 */
function interpolateFrame(
  startFrame: SpectatorReplayFrame,
  endFrame: SpectatorReplayFrame,
  progress: number
): SpectatorReplayFrame {
  const eased = easeInOutCubic(progress);

  const players: Record<string, { x: number; y: number; active: boolean }> = {};
  const playerIds = new Set([
    ...Object.keys(startFrame.players),
    ...Object.keys(endFrame.players),
  ]);

  for (const playerId of playerIds) {
    const start = startFrame.players[playerId];
    const end = endFrame.players[playerId];

    if (!start || !end) {
      players[playerId] = end || start;
      continue;
    }

    players[playerId] = {
      x: lerp(start.x, end.x, eased),
      y: lerp(start.y, end.y, eased),
      active: end.active,
    };
  }

  return {
    ...endFrame,
    ball_x: lerp(startFrame.ball_x, endFrame.ball_x, eased),
    ball_y: lerp(startFrame.ball_y, endFrame.ball_y, eased),
    players,
  };
}

/**
 * Easing function for smooth animation
 */
function easeInOutCubic(t: number): number {
  return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;
}

/**
 * Linear interpolation
 */
function lerp(start: number, end: number, progress: number): number {
  return start + (end - start) * progress;
}

/**
 * Generate movement vector for a player based on pattern
 */
export function calculateMovementVector(
  playerId: string,
  position: string,
  pattern: any,
  hasBall: boolean,
  currentX: number,
  currentY: number
): { dx: number; dy: number; magnitude: number } {
  // Base movement intensity on pattern parameters
  const baseSpeed = pattern.aggressiveness / 100 + 0.3;
  const width = pattern.width / 100;
  const depth = pattern.depth / 100;

  let dx = 0;
  let dy = 0;

  // Determine general movement direction based on position and pattern
  if (hasBall) {
    // Move toward goal
    const targetX = position.includes("F") ? 85 : position.includes("M") ? 60 : 40;
    dx = (targetX - currentX) * baseSpeed * 0.1;
  } else {
    // Off-ball movement
    if (pattern.offTheOallAggression > 60) {
      // Aggressive forward runs
      const targetX = position.includes("F") ? 80 : 65;
      dx = (targetX - currentX) * baseSpeed * 0.08;
    } else {
      // Positional coverage
      dx = (Math.random() - 0.5) * 0.05;
    }
  }

  // Width-based lateral movement
  const lateralBias = Math.sin(Date.now() / 1000) * width * 0.1;
  dy = lateralBias * baseSpeed;

  const magnitude = Math.sqrt(dx * dx + dy * dy);

  return { dx, dy, magnitude };
}
