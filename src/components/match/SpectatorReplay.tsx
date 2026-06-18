import { useEffect, useMemo, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useTranslation } from "react-i18next";
import { Circle, Loader2, Zap, Shield } from "lucide-react";
import {
  MatchSnapshot,
  SpectatorReplayChunk,
  SpectatorReplayFrame,
  SpectatorReplayMetadata,
  SpectatorReplayPlayer,
} from "./types";
import { getEventTypeLabel, getPlayerName, phaseLabel } from "./helpers";
import { analyzePrimaryPattern } from "../../utils/patternAnalyzer";

interface SpectatorReplayProps {
  snapshot: MatchSnapshot;
  homeTeamColor: string;
  awayTeamColor: string;
}

const PITCH_WIDTH = 100;
const PITCH_HEIGHT = 100;
const HOME_REPLAY_COLOR = "#2563eb";
const AWAY_REPLAY_COLOR = "#dc2626";
const ANIMATION_MS = 700;

export default function SpectatorReplay({
  snapshot,
  homeTeamColor: _homeTeamColor,
  awayTeamColor: _awayTeamColor,
}: SpectatorReplayProps) {
  const { t } = useTranslation();
  const [metadata, setMetadata] = useState<SpectatorReplayMetadata | null>(null);
  const [chunks, setChunks] = useState<Record<number, SpectatorReplayChunk>>({});
  const [isLoading, setIsLoading] = useState(true);
  const [selectedMinute, setSelectedMinute] = useState(snapshot.current_minute);
  const [followLive, setFollowLive] = useState(true);
  const [visualFrame, setVisualFrame] = useState<SpectatorReplayFrame>(() => buildFallbackFrame(snapshot));
  const visualFrameRef = useRef(visualFrame);
  const animationRef = useRef<number | null>(null);
  const chunkDuration = metadata?.chunk_duration_minutes || 5;
  const maxMinute = Math.max(snapshot.current_minute, 1);
  const chunkNumber = Math.floor(selectedMinute / chunkDuration);

  useEffect(() => {
    let cancelled = false;
    setIsLoading(true);

    invoke<SpectatorReplayMetadata>("get_spectator_replay_metadata")
      .then((result) => {
        if (!cancelled) setMetadata(result);
      })
      .catch((err) => console.error("Failed to load spectator replay metadata:", err))
      .finally(() => {
        if (!cancelled) setIsLoading(false);
      });

    return () => {
      cancelled = true;
    };
  }, [snapshot.home_team.id, snapshot.away_team.id]);

  useEffect(() => {
    if (followLive) {
      setSelectedMinute(snapshot.current_minute);
    }
  }, [followLive, snapshot.current_minute]);

  useEffect(() => {
    const cachedFrame = chunks[chunkNumber]?.frames.some((frame) => frame.minute === selectedMinute);
    const nextChunkNumber = chunkNumber + 1;
    const shouldLoadNext = nextChunkNumber <= Math.floor(maxMinute / chunkDuration);

    if (cachedFrame && (!shouldLoadNext || chunks[nextChunkNumber])) return;

    let cancelled = false;
    const chunkRequests = [chunkNumber];
    if (shouldLoadNext) chunkRequests.push(nextChunkNumber);

    Promise.all(
      chunkRequests
        .filter((value, index, values) => values.indexOf(value) === index)
        .filter((value) => value >= 0)
        .map((value) => invoke<SpectatorReplayChunk>("get_spectator_replay_chunk", { chunkNumber: value })),
    )
      .then((loadedChunks) => {
        if (cancelled) return;
        setChunks((current) => {
          const next = { ...current };
          for (const chunk of loadedChunks) next[chunk.chunk_number] = chunk;
          return next;
        });
      })
      .catch((err) => console.error("Failed to load spectator replay chunk:", err));

    return () => {
      cancelled = true;
    };
  }, [chunkDuration, chunkNumber, chunks, maxMinute, selectedMinute]);

  const targetFrame = useMemo(() => {
    const chunkFrames = chunks[chunkNumber]?.frames || [];
    return (
      chunkFrames.find((frame) => frame.minute === selectedMinute) ||
      chunkFrames[chunkFrames.length - 1] ||
      buildFallbackFrame(snapshot)
    );
  }, [chunkNumber, chunks, selectedMinute, snapshot]);

  useEffect(() => {
    visualFrameRef.current = visualFrame;
  }, [visualFrame]);

  useEffect(() => {
    if (animationRef.current !== null) cancelAnimationFrame(animationRef.current);

    const startFrame = visualFrameRef.current;
    const startTime = performance.now();

    const animate = (now: number) => {
      const progress = Math.min(1, (now - startTime) / ANIMATION_MS);
      const eased = easeInOut(progress);
      const nextFrame = interpolateFrame(startFrame, targetFrame, eased);
      visualFrameRef.current = nextFrame;
      setVisualFrame(nextFrame);

      if (progress < 1) {
        animationRef.current = requestAnimationFrame(animate);
      }
    };

    animationRef.current = requestAnimationFrame(animate);

    return () => {
      if (animationRef.current !== null) cancelAnimationFrame(animationRef.current);
    };
  }, [targetFrame]);

  const playersById = useMemo(() => {
    const result: Record<string, SpectatorReplayPlayer> = {};
    for (const player of metadata?.players || []) result[player.id] = player;
    return result;
  }, [metadata]);

  const frameEvents = targetFrame.events.slice(-4);
  const goalEvents = snapshot.events.filter(
    (event) => event.event_type === "Goal" || event.event_type === "PenaltyGoal",
  );

  const handleScrub = (clientX: number, rect: DOMRect) => {
    const pct = Math.max(0, Math.min(1, (clientX - rect.left) / rect.width));
    setSelectedMinute(Math.round(pct * maxMinute));
    setFollowLive(false);
  };

  return (
    <div className="flex-1 grid grid-cols-1 xl:grid-cols-[minmax(0,1fr)_18rem] overflow-hidden">
      <div className="min-h-0 p-4 flex flex-col gap-3 bg-gray-100 dark:bg-navy-900">
        <div className="flex items-center justify-between gap-3">
          <div className="min-w-0">
            <p className="text-[10px] font-heading uppercase tracking-widest text-gray-500 dark:text-gray-400">
              {t("match.spectatorMode")}
            </p>
            <p className="text-sm font-heading font-bold text-gray-800 dark:text-gray-200 truncate">
              {phaseLabel(targetFrame.phase, t)} - {selectedMinute}'
            </p>
          </div>
          <button
            type="button"
            onClick={() => {
              setFollowLive(true);
              setSelectedMinute(snapshot.current_minute);
            }}
            className={`px-3 py-1.5 rounded text-xs font-heading uppercase tracking-wider transition-colors ${
              followLive
                ? "bg-primary-500/20 text-primary-700 dark:text-primary-300"
                : "bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-navy-700 dark:text-gray-300 dark:hover:bg-navy-600"
            }`}
          >
            {t("match.live")}
          </button>
        </div>

        <div className="relative px-1 pt-2 pb-6">
          <div
            className="relative h-3 cursor-pointer rounded-full bg-gray-300 dark:bg-navy-700"
            onMouseDown={(event) => handleScrub(event.clientX, event.currentTarget.getBoundingClientRect())}
          >
            <div
              className="absolute left-0 top-0 h-full rounded-full bg-primary-500"
              style={{ width: `${(selectedMinute / maxMinute) * 100}%` }}
            />
            {goalEvents.map((event, index) => (
              <div
                key={`${event.minute}-${event.player_id}-${index}`}
                className={`absolute top-1/2 h-4 w-4 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-white ${
                  event.side === "Home" ? "bg-blue-600" : "bg-red-600"
                }`}
                style={{ left: `${(event.minute / maxMinute) * 100}%` }}
                title={`${event.minute}'`}
              />
            ))}
            <div
              className="absolute top-1/2 h-5 w-5 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-white bg-gray-900 shadow"
              style={{ left: `${(selectedMinute / maxMinute) * 100}%` }}
            />
          </div>
          <div className="absolute bottom-0 left-1 text-[10px] font-heading uppercase tracking-widest text-gray-500">0'</div>
          <div className="absolute bottom-0 right-1 text-[10px] font-heading uppercase tracking-widest text-gray-500">{maxMinute}'</div>
        </div>

        <div className="relative flex-1 min-h-[24rem] rounded bg-emerald-800 shadow-inner overflow-hidden">
          {isLoading && (
            <div className="absolute inset-0 z-20 grid place-items-center bg-navy-950/60 text-white">
              <Loader2 className="w-7 h-7 animate-spin" />
            </div>
          )}
          <svg
            viewBox={`0 0 ${PITCH_WIDTH} ${PITCH_HEIGHT}`}
            className="absolute inset-0 h-full w-full"
            role="img"
            aria-label={t("match.spectatorMode")}
          >
            <PitchMarkings />
            <ReplayBallTrail frame={visualFrame} />
            <ReplayPlayers frame={visualFrame} playersById={playersById} />
            <circle cx={visualFrame.ball_x} cy={visualFrame.ball_y} r="1.6" fill="#ffffff" stroke="#111827" strokeWidth="0.4" />
            <circle cx={visualFrame.ball_x} cy={visualFrame.ball_y} r="3.4" fill="none" stroke="#ffffff" strokeOpacity="0.28" strokeWidth="0.7" />
          </svg>
          <div className="absolute bottom-3 left-3 flex flex-col gap-2 rounded bg-black/40 px-3 py-2 text-xs font-heading text-white">
            <div className="flex items-center gap-4">
              <span>{targetFrame.home_score} - {targetFrame.away_score}</span>
              <span className="flex items-center gap-1">
                <Circle className="w-3 h-3 fill-current" />
                {targetFrame.ball_zone}
              </span>
            </div>
            {/* Quick tactical indicator */}
            <TacticalIndicator
              possession={targetFrame.possession}
              ballZone={targetFrame.ball_zone}
            />
          </div>
        </div>
      </div>

      <aside className="min-h-0 overflow-auto border-l border-gray-200 bg-white p-4 dark:border-navy-700 dark:bg-navy-800 flex flex-col gap-4">
        {/* Tactical Analysis Panel */}
        <TacticalAnalysisPanel
          snapshot={snapshot}
          isVisible={true}
        />

        {/* Events Feed */}
        <div>
          <h3 className="text-xs font-heading font-bold uppercase tracking-widest text-gray-500 dark:text-gray-400 mb-3">
            {t("match.events")}
          </h3>
          <div className="space-y-2">
            {frameEvents.length > 0 ? frameEvents.map((event, index) => (
              <div key={`${event.minute}-${event.event_type}-${index}`} className="rounded border border-gray-200 p-2 dark:border-navy-700">
                <div className="flex items-center justify-between gap-2 text-xs">
                  <span className="font-heading font-bold text-gray-800 dark:text-gray-200">
                    {getEventTypeLabel(event.event_type, t)}
                  </span>
                <span className="tabular-nums text-gray-500">{event.minute}'</span>
              </div>
              <p className="mt-1 truncate text-xs text-gray-600 dark:text-gray-400">
                {getPlayerName(snapshot, event.player_id) || event.side}
              </p>
            </div>
          )) : (
            <p className="text-xs text-gray-600 dark:text-gray-500">{t("match.noEventsYet")}</p>
          )}
          </div>
        </div>
      </aside>
    </div>
  )
}

function ReplayPlayers({
  frame,
  playersById,
}: {
  frame: SpectatorReplayFrame;
  playersById: Record<string, SpectatorReplayPlayer>;
}) {
  const visiblePlayers = Object.entries(frame.players)
    .filter(([playerId, point]) => point.active && playersById[playerId])
    .sort(([leftId], [rightId]) => {
      const left = playersById[leftId];
      const right = playersById[rightId];
      if (left.side !== right.side) return left.side === "Home" ? -1 : 1;
      return left.shirt_number - right.shirt_number;
    });
  const homePlayers = visiblePlayers.filter(([id]) => playersById[id].side === "Home").slice(0, 11);
  const awayPlayers = visiblePlayers.filter(([id]) => playersById[id].side === "Away").slice(0, 11);

  return (
    <>
      {[...homePlayers, ...awayPlayers].map(([playerId, point]) => {
        const player = playersById[playerId];
        const isHome = player.side === "Home";
        const fill = isHome ? HOME_REPLAY_COLOR : AWAY_REPLAY_COLOR;
        const number = player.shirt_number || 0;
        const label = player.name?.split(" ").slice(-1)[0] || "";

        return (
          <g key={playerId} transform={`translate(${point.x} ${point.y})`}>
            <circle r="3.1" fill="#ffffff" opacity="0.95" />
            <circle r="2.5" fill={fill} stroke="#111827" strokeWidth="0.35" />
            <text y="0.8" textAnchor="middle" fontSize="2.4" fontWeight="700" fill="#ffffff">
              {number}
            </text>
            <text
              y="6"
              textAnchor="middle"
              fontSize="2.4"
              fontWeight="700"
              fill="#ffffff"
              paintOrder="stroke"
              stroke="#111827"
              strokeWidth="0.8"
            >
              {label}
            </text>
          </g>
        );
      })}
    </>
  );
}

function ReplayBallTrail({ frame }: { frame: SpectatorReplayFrame }) {
  return (
    <g opacity="0.28">
      <circle cx={frame.ball_x - 2.4} cy={frame.ball_y + 1.2} r="1.2" fill="#ffffff" />
      <circle cx={frame.ball_x - 4.4} cy={frame.ball_y + 2.1} r="0.8" fill="#ffffff" />
    </g>
  );
}

function PitchMarkings() {
  return (
    <g>
      <rect x="0" y="0" width="100" height="100" fill="#137047" />
      {Array.from({ length: 10 }, (_, index) => (
        <rect
          key={index}
          x={index * 10}
          y="0"
          width="10"
          height="100"
          fill={index % 2 === 0 ? "#166f49" : "#1a7a50"}
        />
      ))}
      <rect x="3" y="5" width="94" height="90" fill="none" stroke="#d9f99d" strokeWidth="0.7" />
      <line x1="50" y1="5" x2="50" y2="95" stroke="#d9f99d" strokeWidth="0.7" />
      <circle cx="50" cy="50" r="9" fill="none" stroke="#d9f99d" strokeWidth="0.7" />
      <rect x="3" y="31" width="14" height="38" fill="none" stroke="#d9f99d" strokeWidth="0.7" />
      <rect x="83" y="31" width="14" height="38" fill="none" stroke="#d9f99d" strokeWidth="0.7" />
      <rect x="3" y="41" width="5" height="18" fill="none" stroke="#d9f99d" strokeWidth="0.7" />
      <rect x="92" y="41" width="5" height="18" fill="none" stroke="#d9f99d" strokeWidth="0.7" />
    </g>
  );
}

function buildFallbackFrame(snapshot: MatchSnapshot): SpectatorReplayFrame {
  return {
    minute: snapshot.current_minute,
    phase: snapshot.phase,
    possession: snapshot.possession,
    ball_zone: snapshot.ball_zone,
    ball_x: 50,
    ball_y: 50,
    players: {},
    events: snapshot.events.filter((event) => event.minute === snapshot.current_minute),
    home_score: snapshot.home_score,
    away_score: snapshot.away_score,
  };
}

function interpolateFrame(
  startFrame: SpectatorReplayFrame,
  endFrame: SpectatorReplayFrame,
  progress: number,
): SpectatorReplayFrame {
  const players: SpectatorReplayFrame["players"] = {};
  const playerIds = new Set([
    ...Object.keys(startFrame.players),
    ...Object.keys(endFrame.players),
  ]);

  for (const playerId of playerIds) {
    const start = startFrame.players[playerId] || endFrame.players[playerId];
    const end = endFrame.players[playerId] || startFrame.players[playerId];
    if (!start || !end) continue;

    players[playerId] = {
      x: lerp(start.x, end.x, progress),
      y: lerp(start.y, end.y, progress),
      active: end.active,
    };
  }

  return {
    ...endFrame,
    ball_x: lerp(startFrame.ball_x, endFrame.ball_x, progress),
    ball_y: lerp(startFrame.ball_y, endFrame.ball_y, progress),
    players,
  };
}

function lerp(start: number, end: number, progress: number): number {
  return start + (end - start) * progress;
}

function easeInOut(progress: number): number {
  return progress < 0.5
    ? 2 * progress * progress
    : 1 - Math.pow(-2 * progress + 2, 2) / 2;
}

interface TacticalIndicatorProps {
  possession: "Home" | "Away";
  ballZone: string;
}

function TacticalIndicator({ possession, ballZone }: TacticalIndicatorProps) {
  const isAttacking = ballZone.includes("attacking");
  const defendingZone = ballZone.includes("defensive");

  return (
    <div className="flex items-center gap-2 text-[10px]">
      {possession === "Home" ? (
        <Zap className="w-3 h-3 text-blue-300" />
      ) : (
        <Shield className="w-3 h-3 text-red-300" />
      )}
      <span className="text-white/80">
        {isAttacking ? "Attacking" : defendingZone ? "Defending" : "Midfield"}
      </span>
    </div>
  );
}

interface TacticalAnalysisPanelProps {
  snapshot: MatchSnapshot;
  isVisible: boolean;
}

function TacticalAnalysisPanel({
  snapshot,
  isVisible,
}: TacticalAnalysisPanelProps) {
  const analysis = useMemo(() => {
    return analyzePrimaryPattern({
      homePlayStyle: snapshot.home_team.play_style,
      awayPlayStyle: snapshot.away_team.play_style,
      possession: snapshot.possession,
      ballZone: snapshot.ball_zone,
      homePossession_pct: snapshot.home_possession_pct,
      awayPossession_pct: snapshot.away_possession_pct,
      formation_home: snapshot.home_team.formation,
      formation_away: snapshot.away_team.formation,
    });
  }, [snapshot]);

  if (!isVisible) return null;

  return (
    <div className="border-t border-gray-200 dark:border-navy-700 pt-4">
      <h3 className="text-xs font-heading font-bold uppercase tracking-widest text-gray-500 dark:text-gray-400 mb-3">
        Tactics
      </h3>

      <div className="space-y-3">
        {/* Home Team */}
        <div className="rounded-lg border border-blue-300 dark:border-blue-900 bg-blue-50/50 dark:bg-blue-900/20 p-2">
          <p className="text-xs font-heading font-bold text-gray-800 dark:text-gray-200 mb-1">
            {snapshot.home_team.name}
          </p>
          <p className="text-[10px] text-gray-700 dark:text-gray-300 leading-tight">
            {analysis.home}
          </p>
          <div className="mt-2 flex gap-1">
            <MetricBadge
              label="Press"
              value={analysis.homePattern.pressureIntensity}
            />
            <MetricBadge
              label="Compact"
              value={analysis.homePattern.compactness}
            />
          </div>
        </div>

        {/* Away Team */}
        <div className="rounded-lg border border-red-300 dark:border-red-900 bg-red-50/50 dark:bg-red-900/20 p-2">
          <p className="text-xs font-heading font-bold text-gray-800 dark:text-gray-200 mb-1">
            {snapshot.away_team.name}
          </p>
          <p className="text-[10px] text-gray-700 dark:text-gray-300 leading-tight">
            {analysis.away}
          </p>
          <div className="mt-2 flex gap-1">
            <MetricBadge
              label="Press"
              value={analysis.awayPattern.pressureIntensity}
            />
            <MetricBadge
              label="Compact"
              value={analysis.awayPattern.compactness}
            />
          </div>
        </div>
      </div>
    </div>
  );
}

interface MetricBadgeProps {
  label: string;
  value: number;
}

function MetricBadge({ label, value }: MetricBadgeProps) {
  const getColor = () => {
    if (value > 75) return "bg-red-600 text-white";
    if (value > 50) return "bg-yellow-600 text-white";
    return "bg-green-600 text-white";
  };

  return (
    <span
      className={`px-2 py-0.5 rounded text-[9px] font-heading font-bold ${getColor()}`}
    >
      {label} {value}
    </span>
  );
}
