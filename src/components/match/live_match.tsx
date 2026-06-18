import { useEffect, useMemo, useState } from "react";
import * as PIXI from "pixi.js";
import MatchLive from "./MatchLive";
import { GameStateData } from "../../store/gameStore";
import { MatchEvent, MatchSnapshot } from "./types";

interface I18nLike {
  t?: (key: string) => string;
}

interface SquadPlayer {
  
  id: number | string;
  position: string;
  last_name: string;
  sub_minute?: number | null;
  subbed_off_minute?: number | null;
  is_player_of_the_match?: boolean;
}

interface GoalEvent {
  player_id: number | string;
  player_name: string;
  minute: number;
  is_auto_goal?: boolean;
}

interface PlaybackGoal extends GoalEvent {
  time: number;
}

interface PlaybackPlayer {
  id: number;
  position: string;
  is_home: boolean;
  shirt_number: number | string;
  last_name: string;
}

type PositionSample = [timestamp: number, x: number, y: number, z?: number];

interface PlaybackEvent {
  timestamp: number;
  category: "ball" | "player" | string;
  description: string;
}

interface MatchChunk {
  ball?: PositionSample[];
  players?: Record<string, PositionSample[]>;
  events?: PlaybackEvent[];
}

interface MatchMetadata {
  chunk_count?: number;
  chunk_duration_ms?: number;
}

interface LiveMatchProps {
  snapshot: MatchSnapshot;
  gameState: GameStateData;
  userSide: "Home" | "Away" | null;
  isSpectator: boolean;
  importantEvents: MatchEvent[];
  onSnapshotUpdate: (snap: MatchSnapshot) => void;
  onImportantEvent: (evt: MatchEvent) => void;
  onHalfTime: () => void;
  onFullTime: () => void;
}

interface TemplateLiveMatchViewProps {
  lang: string;
  league_slug: string;
  league_name: string;
  home_team_slug: string;
  home_team_name: string;
  home_goal_events?: GoalEvent[];
  home_goals: number;
  away_goals: number;
  away_team_slug: string;
  away_team_name: string;
  away_goal_events?: GoalEvent[];
  player_of_the_match_id?: number;
  player_of_the_match_name?: string;
  home_squad_main?: SquadPlayer[];
  home_squad_subs?: SquadPlayer[];
  away_squad_main?: SquadPlayer[];
  away_squad_subs?: SquadPlayer[];
  match_recordings_enabled?: boolean;
  match_id: number | string;
  match_time_ms: number;
  goals_json?: PlaybackGoal[];
  players_json?: PlaybackPlayer[];
  home_color_background: string;
  home_color_foreground: string;
  away_color_background: string;
  away_color_foreground: string;
  i18n?: I18nLike;
}

type TemplateLiveMatchProps =
  | LiveMatchProps
  | TemplateLiveMatchViewProps
  | (Partial<LiveMatchProps> & Partial<TemplateLiveMatchViewProps>);

const t = (i18n: I18nLike | undefined, key: string): string => (i18n?.t ? i18n.t(key) : key);
const cx = (...classes: Array<string | false | null | undefined>): string => classes.filter(Boolean).join(" ");

function PlayerRow({
  lang,
  player,
  substitute = false,
}: {
  lang: string;
  player: SquadPlayer;
  substitute?: boolean;
}) {
  const subbedIn = player.sub_minute != null;
  const subbedOff = player.subbed_off_minute != null;

  return (
    <a
      href={`/${lang}/players/${player.id}`}
      className={cx(
        "fm-squad-player",
        substitute && (subbedIn ? "fm-squad-player-subbed" : "fm-squad-player-bench"),
        subbedOff && "fm-squad-player-off",
        player.is_player_of_the_match && "fm-squad-player-motm",
      )}
    >
      {substitute && subbedIn && (
        <span className="fm-sub-badge fm-sub-in">
          <i className="fa fa-arrow-up" /> {player.sub_minute}'
        </span>
      )}
      <small>{player.position}</small>
      <span className="fm-sp-name">{player.last_name}</span>
      {player.is_player_of_the_match && <span className="fm-motm-badge">{"\u2605"}</span>}
      {subbedOff && (
        <span className="fm-sub-badge fm-sub-out">
          <i className="fa fa-arrow-down" /> {player.subbed_off_minute}'
        </span>
      )}
    </a>
  );
}

function Squad({
  lang,
  team_slug,
  team_name,
  side,
  main = [],
  subs = [],
  i18n,
}: {
  lang: string;
  team_slug: string;
  team_name: string;
  side: "home" | "away";
  main?: SquadPlayer[];
  subs?: SquadPlayer[];
  i18n?: I18nLike;
}) {
  return (
    <div className={`fm-match-squad fm-match-squad-${side}`}>
      <div className="fm-squad-hdr">
        <span className={`fm-squad-hdr-dot fm-squad-hdr-dot-${side}`} />
        <a href={`/${lang}/teams/${team_slug}`} className="fm-squad-title">
          {team_name}
        </a>
      </div>
      <div className="fm-squad-section">
        {main.map((player) => (
          <PlayerRow key={player.id} lang={lang} player={player} />
        ))}
      </div>
      <div className="fm-squad-divider">{t(i18n, "substitutes")}</div>
      <div className="fm-squad-section">
        {subs.map((player) => (
          <PlayerRow key={player.id} lang={lang} player={player} substitute />
        ))}
      </div>
    </div>
  );
}

function GoalList({ lang, id, goals = [] }: { lang: string; id: string; goals?: GoalEvent[] }) {
  const [expanded, setExpanded] = useState(goals.length <= 5);
  const hiddenCount = Math.max(goals.length - 5, 0);

  return (
    <>
      <div className={cx("fm-sb-goals", expanded && "fm-sb-goals-expanded")} id={id}>
        {goals.map((goal, index) => (
          <a
            key={`${goal.player_id}-${goal.minute}-${index}`}
            href={`/${lang}/players/${goal.player_id}`}
            className="fm-sb-goal"
          >
            {goal.player_name} {goal.minute}'{goal.is_auto_goal ? " (OG)" : ""}
          </a>
        ))}
      </div>
      {goals.length > 5 && (
        <button className="fm-sb-expander" type="button" onClick={() => setExpanded((value) => !value)}>
          {expanded ? "-" : `... +${hiddenCount}`}
        </button>
      )}
    </>
  );
}

function MatchPlayback({
  i18n,
  match_id,
  match_time_ms,
  goals_json = [],
  players_json = [],
  home_color_background,
  home_color_foreground,
  away_color_background,
  away_color_foreground,
}: {
  i18n?: I18nLike;
  match_id: number | string;
  match_time_ms: number;
  goals_json?: PlaybackGoal[];
  players_json?: PlaybackPlayer[];
  home_color_background: string;
  home_color_foreground: string;
  away_color_background: string;
  away_color_foreground: string;
}) {
  useEffect(() => {
    let cancelled = false;
    let cleanup: () => void = () => {};

    const maxWidth = 1400;
    const maxHeight = 950;
    const aspectRatio = 16 / 10;
    const targetFps = 30;
    const frameInterval = 1000 / targetFps;
    const fieldOffsetX = 62;
    const fieldOffsetY = 60;
    const scaleX = 1260 / 840;
    const scaleY = 810 / 545;

    let app: PIXI.Application | null = null;
    let background: PIXI.Sprite | null = null;
    let gameContainer: PIXI.Container | null = null;
    let currentTime = 0;
    let isPlaying = false;
    let lastTickTime = 0;
    let lastFrameTime = 0;
    let ballData: PositionSample[] = [];
    let playerPositions: Record<string, PositionSample[]> = {};
    let playerGraphics: Record<string, PIXI.Container> = {};
    let playerIds: string[] = [];
    let ballObj: PIXI.Container | null = null;
    let loadedChunks = new Set<number>();
    let loadingChunks = new Set<number>();
    let totalChunks = 0;
    let chunkDurationMs = 300000;
    let playerMap: Record<number, PlaybackPlayer> = {};
    let eventData: PlaybackEvent[] = [];
    let lastEventIdx = 0;
    let sliderEl: HTMLElement | null = null;
    let progressEl: HTMLElement | null = null;
    let displayEl: HTMLElement | null = null;
    let lastBallIdx = 0;
    let lastPlayerIdx: Record<string, number> = {};

    const translateToField = (x: number, y: number) => ({
      x: x * scaleX + fieldOffsetX,
      y: y * scaleY + fieldOffsetY,
    });

    const getChunkNumber = (time: number) => Math.floor(time / chunkDurationMs);
    const hexToInt = (hex: string) => Number.parseInt(String(hex).replace("#", ""), 16);

    const homeBg = hexToInt(home_color_background);
    const homeFg = hexToInt(home_color_foreground);
    const awayBg = hexToInt(away_color_background);
    const awayFg = hexToInt(away_color_foreground);

    function sampleTime(item: PositionSample | PlaybackEvent): number {
      return Array.isArray(item) ? item[0] : item.timestamp;
    }

    function mergeSorted<T extends PositionSample | PlaybackEvent>(existing: T[], incoming: T[]): T[] {
      if (existing.length === 0) return incoming;
      const firstIncoming = incoming[0];
      const lastExisting = existing[existing.length - 1];
      if (firstIncoming && lastExisting && sampleTime(firstIncoming) >= sampleTime(lastExisting)) {
        return existing.concat(incoming);
      }

      const merged: T[] = new Array(existing.length + incoming.length);
      let i = 0;
      let j = 0;
      let k = 0;
      while (i < existing.length && j < incoming.length) {
        const existingItem = existing[i];
        const incomingItem = incoming[j];
        if (!existingItem || !incomingItem) break;
        merged[k++] = sampleTime(existingItem) <= sampleTime(incomingItem) ? existing[i++] : incoming[j++];
      }
      while (i < existing.length) {
        const item = existing[i++];
        if (item) merged[k++] = item;
      }
      while (j < incoming.length) {
        const item = incoming[j++];
        if (item) merged[k++] = item;
      }
      return merged.slice(0, k);
    }

    function createPlayerGraphic(x: number, y: number, player: PlaybackPlayer): PIXI.Container {
      const container = new PIXI.Container();
      container.position.set(x - 10, y - 10);

      const isGK = player.position === "GK";
      const fillColor = isGK ? 0xf7e300 : player.is_home ? homeBg : awayBg;
      const textColor = isGK ? "black" : player.is_home ? home_color_foreground : away_color_foreground;
      const borderColor = isGK ? 0x000000 : player.is_home ? homeFg : awayFg;

      const border = new PIXI.Graphics();
      border.circle(6, 6, 19).fill(borderColor);
      container.addChild(border);

      const circle = new PIXI.Graphics();
      circle.circle(6, 6, 16).fill(fillColor);
      container.addChild(circle);

      const numText = new PIXI.Text({
        text: String(player.shirt_number),
        style: new PIXI.TextStyle({
          fontFamily: "Arial, sans-serif",
          fontSize: 14,
          fontWeight: "bold",
          fill: textColor,
          align: "center",
        }),
      });
      numText.anchor.set(0.5);
      numText.position.set(6, 6);
      container.addChild(numText);

      const nameText = new PIXI.Text({
        text: player.last_name,
        style: new PIXI.TextStyle({
          fontFamily: "Verdana, sans-serif",
          fontSize: 17,
          fill: "white",
          wordWrap: false,
          align: "center",
        }),
      });
      nameText.anchor.set(0.5);
      nameText.position.set(10, 40);
      container.addChild(nameText);

      return container;
    }

    function mergeData(data: MatchChunk) {
      if (data.ball) ballData = mergeSorted(ballData, data.ball);

      if (data.players) {
        Object.entries(data.players).forEach(([id, positions]) => {
          if (!playerPositions[id]) {
            playerPositions[id] = positions;
            if (gameContainer && !playerGraphics[id] && positions.length > 0) {
              const player = playerMap[Number(id)];
              if (player) {
                const firstPosition = positions[0];
                if (!firstPosition) return;

                const pos = translateToField(firstPosition[1], firstPosition[2]);
                const gfx = createPlayerGraphic(pos.x, pos.y, player);
                playerGraphics[id] = gfx;
                playerIds.push(id);
                gameContainer.addChild(gfx);
              }
            }
          } else {
            playerPositions[id] = mergeSorted(playerPositions[id], positions);
          }
        });
      }

      if (data.events?.length) eventData = mergeSorted(eventData, data.events);
    }

    async function loadChunk(chunkNum: number) {
      if (
        !Number.isFinite(chunkNum) ||
        loadedChunks.has(chunkNum) ||
        loadingChunks.has(chunkNum) ||
        chunkNum < 0 ||
        chunkNum >= totalChunks
      ) {
        return;
      }

      loadingChunks.add(chunkNum);
      try {
        const res = await fetch(`/api/match/${match_id}/chunk/${chunkNum}`);
        mergeData((await res.json()) as MatchChunk);
        loadedChunks.add(chunkNum);
      } finally {
        loadingChunks.delete(chunkNum);
      }
    }

    function findIndexNear(arr: PositionSample[], time: number, hint: number) {
      const len = arr.length;
      function updatePositions(time: number) {
        lastBallIdx = findIndexNear(ballData, time, lastBallIdx);
        if (lastBallIdx >= 0 && ballObj) {
          const pos3 = interpolatePosition(ballData, lastBallIdx, time);
          const pos = translateToField(pos3[0], pos3[1]);
          ballObj.position.set(pos.x, pos.y);
          ballObj.scale.set(1 + Math.min((pos3[2] || 0) / 15, 0.4));
        }

        playerIds.forEach((id) => {
          const positions = playerPositions[id];
          const gfx = playerGraphics[id];
          if (!positions || !gfx) return;

          const pi = findIndexNear(positions, time, lastPlayerIdx[id] || 0);
          lastPlayerIdx[id] = pi;
          if (pi < 0) return;

          const first = positions[0];
          const last = positions[positions.length - 1];
          if (!first || !last) return;

          const firstTs = first[0];
          const lastTs = last[0];
          if (time > lastTs + 1000 || time < firstTs - 1000) {
            gfx.visible = false;
            return;
          }

          gfx.visible = true;
          const pos3 = interpolatePosition(positions, pi, time);
          const pos = translateToField(pos3[0], pos3[1]);

          gfx.position.set(pos.x - 10, pos.y - 10);
        });
      }
          if (!gfx || !gfx.visible || !player) return;
          const px = gfx.position.x + 10;
          const py = gfx.position.y + 10;
          if (player.is_home) homePositions.push({ x: px, y: py });
          else awayPositions.push({ x: px, y: py });
        });

        function summarize(positions: { x: number; y: number }[]) {
          if (positions.length === 0) return { cx: 0, cy: 0, compact: 9999 };
          const cx = positions.reduce((s, p) => s + p.x, 0) / positions.length;
          const cy = positions.reduce((s, p) => s + p.y, 0) / positions.length;
          const compact = positions.reduce((s, p) => s + Math.hypot(p.x - cx, p.y - cy), 0) / positions.length;
          return { cx, cy, compact };
        }

        const homeMetrics = summarize(homePositions);
        const awayMetrics = summarize(awayPositions);

        const homeSummary = describeTeamShape(homeStrategy || getStrategyForTeam(home_team_slug), homeMetrics.compact);
        const awaySummary = describeTeamShape(awayStrategy || getStrategyForTeam(away_team_slug), awayMetrics.compact);

        const txt = `T+${currentSecond}s — Home: ${homeSummary} | Away: ${awaySummary}`;
        if (!sideAnalysisEl) {
          const container = document.getElementById("match-play-area");
          if (container) {
            sideAnalysisEl = document.createElement("div");
            sideAnalysisEl.id = "side-analysis";
            sideAnalysisEl.className = "fm-side-analysis";
            sideAnalysisEl.style.position = "absolute";
            sideAnalysisEl.style.right = "8px";
            sideAnalysisEl.style.top = "8px";
            sideAnalysisEl.style.padding = "6px 8px";
            sideAnalysisEl.style.background = "rgba(0,0,0,0.6)";
            sideAnalysisEl.style.color = "white";
            sideAnalysisEl.style.fontSize = "12px";
            sideAnalysisEl.style.zIndex = "9999";
            container.appendChild(sideAnalysisEl);
          }
        }
        if (sideAnalysisEl) sideAnalysisEl.textContent = txt;
        console.log(txt);
      }
    }

    function computeTacticalOffset(
      id: string,
      playerPos3: [number, number, number],
      player: PlaybackPlayer | undefined,
      ballScreen: { x: number; y: number } | null,
    ) {
      // default no offset
      const out = { x: 0, y: 0 };
      if (!player) return out;

      // simple role detection
      const posLabel = (player.position || "").toUpperCase();

      if (!ballScreen) return out;

      const playerScreen = translateToField(playerPos3[0], playerPos3[1]);
      const dx = ballScreen.x - playerScreen.x;
      const dy = ballScreen.y - playerScreen.y;
      const dist = Math.sqrt(dx * dx + dy * dy);

      // goalkeeper stays near goal line
      if (posLabel.includes("GK")) {
        const goalX = player.is_home ? translateToField(20, playerPos3[1]).x : translateToField(820, playerPos3[1]).x;
        return { x: (goalX - playerScreen.x) * 0.25, y: 0 };
      }

      // defenders: keep formation but step to ball if close
      if (posLabel.includes("D") || posLabel.includes("CB") || posLabel.includes("LB") || posLabel.includes("RB")) {
        if (dist < 180) return { x: dx * 0.18, y: dy * 0.18 };
        return { x: 0, y: 0 };
      }

      // midfielders: support the ball, shift toward ball mildly
      if (posLabel.includes("M") || posLabel.includes("CM") || posLabel.includes("DM") || posLabel.includes("AM")) {
        if (dist < 300) return { x: dx * 0.22, y: dy * 0.22 };
        return { x: dx * 0.08, y: dy * 0.08 };
      }

      // forwards/attackers: push into space in direction of opponent goal when ball is in attacking half
      if (posLabel.includes("F") || posLabel.includes("ST") || posLabel.includes("FW")) {
        // if ball is ahead (towards opponent), push forward more
        const push = Math.min(1, Math.max(0, 300 / (dist + 1)));
        return { x: dx * 0.14 + (player.is_home ? 20 : -20) * push, y: dy * 0.06 };
      }

      // default small support shift
      if (dist < 250) return { x: dx * 0.12, y: dy * 0.12 };
      return out;
    }

    function formatMatchTime(ms: number) {
      const totalSeconds = Math.floor(ms / 1000);
      const minutes = Math.floor(totalSeconds / 60);
      const seconds = totalSeconds % 60;
      const halfDuration = match_time_ms / 2;

      if (ms < halfDuration) {
        return `${minutes}:${String(seconds).padStart(2, "0")} (1st)`;
      }

      const secondHalfSeconds = Math.floor((ms - halfDuration) / 1000);
      return `${Math.floor(secondHalfSeconds / 60)}:${String(secondHalfSeconds % 60).padStart(2, "0")} (2nd)`;
    }

    function logEvents(time: number) {
      while (lastEventIdx < eventData.length && eventData[lastEventIdx].timestamp <= time) {
        const evt = eventData[lastEventIdx];
        const timeStr = formatMatchTime(evt.timestamp);
        const color = evt.category === "ball" ? "#4fc3f7" : "#81c784";
        const label = evt.category === "ball" ? "Ball" : "Player";
        console.log(`%c[${timeStr}] ${label}: ${evt.description}`, `color: ${color}`);
        lastEventIdx++;
      }
    }

    function updateSlider() {
      if (match_time_ms <= 0 || !sliderEl || !progressEl || !displayEl) return;
      const pct = (currentTime / match_time_ms) * 100;
      sliderEl.style.left = `${pct}%`;
      progressEl.style.width = `${pct}%`;
      displayEl.textContent = formatMatchTime(currentTime);
    }

    function setLoadingSpinnerContent(wrapped = false) {
      const loadingSpinner = document.getElementById("loading-spinner");
      if (!loadingSpinner) return;

      loadingSpinner.replaceChildren();

      const icon = document.createElement("i");
      icon.className = "fas fa-spinner fa-spin fa-2x";

      const text = document.createElement("p");
      text.textContent = t(i18n, "loading_match");

      if (wrapped) {
        const wrapper = document.createElement("div");
        wrapper.className = "fm-match-not-ready";
        wrapper.append(icon, text);
        loadingSpinner.appendChild(wrapper);
        return;
      }

      loadingSpinner.append(icon, text);
    }

    function resizeApp() {
      if (!app || !background || !gameContainer) return;

      const container = document.getElementById("pixi-container");
      if (!container) return;

      app.canvas.style.width = "";
      app.canvas.style.height = "";

      const w = container.clientWidth;
      const h = w / aspectRatio;
      app.renderer.resize(w, h);

      const scale = Math.min(w / maxWidth, h / maxHeight);
      app.stage.scale.set(scale);
      app.stage.position.set((w - maxWidth * scale) / 2, (h - maxHeight * scale) / 2);
      background.width = maxWidth;
      background.height = maxHeight;

      app.canvas.style.width = "100%";
      app.canvas.style.height = "auto";
    }

    function tick(now: number) {
      if (!isPlaying) return;

      const elapsed = now - lastFrameTime;
      if (elapsed < frameInterval) return;
      lastFrameTime = now - (elapsed % frameInterval);

      if (lastTickTime === 0) lastTickTime = now;
      currentTime += now - lastTickTime;
      lastTickTime = now;

      if (currentTime >= match_time_ms) {
        currentTime = match_time_ms;
        isPlaying = false;
      }

      const chunkNeeded = getChunkNumber(currentTime);
      if (!loadedChunks.has(chunkNeeded)) loadChunk(chunkNeeded);

      const nextChunk = chunkNeeded + 1;
      if (nextChunk < totalChunks && !loadedChunks.has(nextChunk)) loadChunk(nextChunk);

      updatePositions(currentTime);
      logEvents(currentTime);
      updateSlider();
    }

    async function fetchMetadata(): Promise<MatchMetadata | null> {
      const metaRes = await fetch(`/api/match/${match_id}/metadata`);
      return metaRes.ok ? ((await metaRes.json()) as MatchMetadata) : null;
    }

    async function startMatch(metadata: MatchMetadata) {
      totalChunks = metadata.chunk_count || 0;
      chunkDurationMs = metadata.chunk_duration_ms || 300000;

      const chunkRes = await fetch(`/api/match/${match_id}/chunk/0`);
      const firstChunk = (await chunkRes.json()) as MatchChunk;
      mergeData(firstChunk);
      loadedChunks.add(0);

      app = new PIXI.Application();
      await app.init({
        antialias: true,
        autoDensity: true,
        resolution: window.devicePixelRatio,
        width: maxWidth,
        height: maxHeight,
        backgroundAlpha: 0,
      });

      if (cancelled) {
        app.destroy(true);
        return;
      }

      const container = document.getElementById("pixi-container");
      container?.appendChild(app.canvas);

      const bgTexture = await PIXI.Assets.load("/static/images/match/field.svg");
      background = new PIXI.Sprite(bgTexture);
      background.width = maxWidth;
      background.height = maxHeight;
      app.stage.addChild(background);

      gameContainer = new PIXI.Container();
      app.stage.addChild(gameContainer);

      players_json.forEach((player) => {
        playerMap[player.id] = player;
      });

      Object.entries(firstChunk.players || {}).forEach(([id, positions]) => {
        const player = playerMap[Number(id)];
        if (!player || positions.length === 0) return;

        const firstPosition = positions[0];
        if (!firstPosition) return;

        const pos = translateToField(firstPosition[1], firstPosition[2]);
        const gfx = createPlayerGraphic(pos.x, pos.y, player);
        playerGraphics[id] = gfx;
        playerIds.push(id);
        gameContainer.addChild(gfx);
      });

      const ballContainer = new PIXI.Container();
      const ballCircle = new PIXI.Graphics();
      ballCircle.circle(0, 0, 6).fill(0xffffff);
      ballCircle.circle(0, 0, 6).stroke({ width: 2, color: 0x000000 });
      ballContainer.addChild(ballCircle);

      const firstBallSample = firstChunk.ball?.[0];
      if (firstBallSample) {
        const bp = translateToField(firstBallSample[1], firstBallSample[2]);
        ballContainer.position.set(bp.x, bp.y);
      }

      gameContainer.addChild(ballContainer);
      ballObj = ballContainer;

      resizeApp();
      window.addEventListener("resize", resizeApp);

      document.getElementById("loading-spinner")?.style.setProperty("display", "none");
      document.getElementById("time-scrollbar-wrapper")?.style.setProperty("display", "block");

      const markersDiv = document.getElementById("goal-markers");
      goals_json.forEach((goal) => {
        const pct = (goal.time / match_time_ms) * 100;
        const player = players_json.find((p) => p.id === goal.player_id);
        const marker = document.createElement("div");
        const icon = document.createElement("div");

        marker.className = `goal-marker ${player?.is_home ?? true ? "goal-marker-home" : "goal-marker-away"}`;
        marker.style.left = `${pct}%`;
        icon.className = "goal-icon";
        icon.textContent = "\u26BD";
        marker.appendChild(icon);
        markersDiv?.appendChild(marker);
      });

      const track = document.getElementById("time-scrollbar-track");
      const onTrackMouseDown = async (event: MouseEvent) => {
        const trackEl = event.currentTarget as HTMLElement;
        const rect = trackEl.getBoundingClientRect();
        const pct = Math.max(0, Math.min(1, (event.clientX - rect.left) / rect.width));
        currentTime = pct * match_time_ms;
        lastEventIdx = 0;
        await loadChunk(getChunkNumber(currentTime));
        updatePositions(currentTime);
        updateSlider();
      };
      track?.addEventListener("mousedown", onTrackMouseDown);

      sliderEl = document.getElementById("time-slider");
      progressEl = document.getElementById("time-progress");
      displayEl = document.getElementById("time-display");

      isPlaying = true;
      lastTickTime = 0;
      app.ticker.add(() => tick(performance.now()));

      cleanup = () => {
        window.removeEventListener("resize", resizeApp);
        track?.removeEventListener("mousedown", onTrackMouseDown);
        app?.destroy(true, { children: true, texture: false });
      };
    }

    async function init() {
      let metadata = await fetchMetadata();
      if (metadata) {
        await startMatch(metadata);
        return;
      }

      setLoadingSpinnerContent(true);

      const pollInterval = setInterval(async () => {
        metadata = await fetchMetadata();
        if (!metadata) return;

        clearInterval(pollInterval);
        setLoadingSpinnerContent();
        await startMatch(metadata);
      }, 3000);

      cleanup = () => clearInterval(pollInterval);
    }

    init();

    return () => {
      cancelled = true;
      cleanup();
    };
  }, [
    away_color_background,
    away_color_foreground,
    goals_json,
    home_color_background,
    home_color_foreground,
    i18n,
    match_id,
    match_time_ms,
    players_json,
  ]);

  return (
    <div id="match-play-area">
      <div className="time-scrollbar-wrapper" id="time-scrollbar-wrapper" style={{ display: "none" }}>
        <div className="time-scrollbar-container">
          <div className="half-labels">
            <span className="half-label">{t(i18n, "first_half")}</span>
            <span className="half-label">{t(i18n, "second_half")}</span>
          </div>
          <div className="time-scrollbar-track" id="time-scrollbar-track">
            <div className="halftime-delimiter" style={{ left: "50%" }}>
              <div className="delimiter-line" />
              <div className="delimiter-label">{t(i18n, "ht")}</div>
            </div>
            <div id="goal-markers" />
            <div className="time-progress" id="time-progress" />
            <div className="time-scrollbar-slider" id="time-slider">
              <div className="time-display" id="time-display">
                0:00
              </div>
            </div>
          </div>
        </div>
      </div>
      <div id="match-container">
        <div className="loading-spinner" id="loading-spinner">
          <i className="fas fa-spinner fa-spin fa-2x" />
          <p>{t(i18n, "loading_match")}</p>
        </div>
        <div id="pixi-container" className="match-container" />
      </div>
    </div>
  );
}

function isLiveMatchProps(props: TemplateLiveMatchProps): props is LiveMatchProps {
  return Boolean((props as Partial<LiveMatchProps>).snapshot && (props as Partial<LiveMatchProps>).gameState);
}

export function TemplateLiveMatch(props: TemplateLiveMatchProps) {
  if (isLiveMatchProps(props)) {
    return (
      <MatchLive
        snapshot={props.snapshot}
        gameState={props.gameState}
        userSide={props.userSide}
        isSpectator={props.isSpectator}
        importantEvents={props.importantEvents}
        onSnapshotUpdate={props.onSnapshotUpdate}
        onImportantEvent={props.onImportantEvent}
        onHalfTime={props.onHalfTime}
        onFullTime={props.onFullTime}
      />
    );
  }

  return <TemplateLiveMatchView {...(props as TemplateLiveMatchViewProps)} />;
}

function TemplateLiveMatchView({
  lang,
  league_slug,
  league_name,
  home_team_slug,
  home_team_name,
  home_goal_events = [],
  home_goals,
  away_goals,
  away_team_slug,
  away_team_name,
  away_goal_events = [],
  player_of_the_match_id = 0,
  player_of_the_match_name,
  home_squad_main = [],
  home_squad_subs = [],
  away_squad_main = [],
  away_squad_subs = [],
  match_recordings_enabled,
  match_id,
  match_time_ms,
  goals_json = [],
  players_json = [],
  home_color_background,
  home_color_foreground,
  away_color_background,
  away_color_foreground,
  i18n,
}: TemplateLiveMatchViewProps) {
  const homeSquad = useMemo(
    () => (
      <Squad
        lang={lang}
        team_slug={home_team_slug}
        team_name={home_team_name}
        side="home"
        main={home_squad_main}
        subs={home_squad_subs}
        i18n={i18n}
      />
    ),
    [home_squad_main, home_squad_subs, home_team_name, home_team_slug, i18n, lang],
  );

  const awaySquad = useMemo(
    () => (
      <Squad
        lang={lang}
        team_slug={away_team_slug}
        team_name={away_team_name}
        side="away"
        main={away_squad_main}
        subs={away_squad_subs}
        i18n={i18n}
      />
    ),
    [away_squad_main, away_squad_subs, away_team_name, away_team_slug, i18n, lang],
  );

  return (
    <div className="fm-page fm-match-page">
      <div className="fm-match-scoreboard">
        <a href={`/${lang}/leagues/${league_slug}`} className="fm-sb-league">
          {league_name}
        </a>
        <div className="fm-sb-main">
          <div className="fm-sb-team fm-sb-team-home">
            <a href={`/${lang}/teams/${home_team_slug}`} className="fm-sb-team-name">
              {home_team_name}
            </a>
            <GoalList lang={lang} id="home-goals" goals={home_goal_events} />
          </div>
          <div className="fm-sb-score">
            <span className="fm-sb-digit">{home_goals}</span>
            <span className="fm-sb-sep" />
            <span className="fm-sb-digit">{away_goals}</span>
          </div>
          <div className="fm-sb-team fm-sb-team-away">
            <a href={`/${lang}/teams/${away_team_slug}`} className="fm-sb-team-name">
              {away_team_name}
            </a>
            <GoalList lang={lang} id="away-goals" goals={away_goal_events} />
          </div>
        </div>
        <div className="fm-sb-ft">{t(i18n, "full_time")}</div>
        {player_of_the_match_id > 0 && (
          <div className="fm-sb-motm">
            <span className="fm-motm-star">{"\u2605"}</span>
            <span className="fm-motm-label">{t(i18n, "player_of_the_match")}</span>
            <a href={`/${lang}/players/${player_of_the_match_id}`} className="fm-motm-name">
              {player_of_the_match_name}
            </a>
          </div>
        )}
      </div>

      <div className="container-fluid">
        <div className="row">
          <div className="col-lg-2 d-none d-lg-block">{homeSquad}</div>
          <div className="col-lg-8">
            {match_recordings_enabled ? (
              <MatchPlayback
                i18n={i18n}
                match_id={match_id}
                match_time_ms={match_time_ms}
                goals_json={goals_json}
                players_json={players_json}
                home_color_background={home_color_background}
                home_color_foreground={home_color_foreground}
                away_color_background={away_color_background}
                away_color_foreground={away_color_foreground}
                homeStrategy={getStrategyForTeam(home_team_slug)}
                awayStrategy={getStrategyForTeam(away_team_slug)}
                home_team_slug={home_team_slug}
                away_team_slug={away_team_slug}
              />
            ) : (
              <div className="fm-match-recording-disabled">
                <i className="fa fa-video-slash" />
                <p>{t(i18n, "match_recording_disabled")}</p>
                <p>{t(i18n, "match_recording_disabled_local")}</p>
                <p className="fm-match-recording-note">
                  <code>open_football --match-recording-enabled</code>
                </p>
                <a href="https://github.com/ZOXEXIVO/open-football" target="_blank" rel="noopener noreferrer">
                  {t(i18n, "match_recording_disabled_link")}
                </a>
              </div>
            )}
          </div>
          <div className="col-lg-2 d-none d-lg-block">{awaySquad}</div>
        </div>
        <div className="fm-match-squads-mobile d-lg-none">
          {homeSquad}
          {awaySquad}
        </div>
      </div>
    </div>
  );
}

export default function LiveMatchCompat(props: TemplateLiveMatchProps) {
  return <TemplateLiveMatch {...props} />;
}
