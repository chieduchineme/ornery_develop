import React, { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Activity, Shield, Zap, Target } from "lucide-react";
import { MatchSnapshot } from "./types";
import {
  analyzePrimaryPattern,
  PatternParameters,
} from "../../utils/patternAnalyzer";

interface TacticalAnalysisProps {
  snapshot: MatchSnapshot;
  homeTeamColor: string;
  awayTeamColor: string;
}

export default function TacticalAnalysis({
  snapshot,
  homeTeamColor,
  awayTeamColor,
}: TacticalAnalysisProps) {
  const { t } = useTranslation();

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

  return (
    <div className="space-y-4">
      <div className="text-xs font-heading uppercase tracking-widest text-gray-500 dark:text-gray-400">
        {t("match.tacticalAnalysis")}
      </div>

      {/* Home Team */}
      <TacticalCard
        teamName={snapshot.home_team.name}
        pattern={analysis.homePattern}
        description={analysis.home}
        isAttacking={snapshot.possession === "Home"}
        teamColor={homeTeamColor}
      />

      {/* Away Team */}
      <TacticalCard
        teamName={snapshot.away_team.name}
        pattern={analysis.awayPattern}
        description={analysis.away}
        isAttacking={snapshot.possession === "Away"}
        teamColor={awayTeamColor}
      />

      {/* Pattern Details */}
      <div className="mt-6 pt-4 border-t border-gray-200 dark:border-navy-700">
        <h4 className="text-xs font-heading font-bold uppercase tracking-widest text-gray-600 dark:text-gray-400 mb-2">
          {t("match.matchContext")}
        </h4>
        <div className="space-y-2 text-xs text-gray-700 dark:text-gray-300">
          <div className="flex items-center justify-between">
            <span className="text-gray-600 dark:text-gray-400">Ball Zone:</span>
            <span className="font-medium">{snapshot.ball_zone}</span>
          </div>
          <div className="flex items-center justify-between">
            <span className="text-gray-600 dark:text-gray-400">Phase:</span>
            <span className="font-medium">{snapshot.phase}</span>
          </div>
        </div>
      </div>
    </div>
  );
}

interface TacticalCardProps {
  teamName: string;
  pattern: PatternParameters;
  description: string;
  isAttacking: boolean;
  teamColor: string;
}

function TacticalCard({
  teamName,
  pattern,
  description,
  isAttacking,
  teamColor,
}: TacticalCardProps) {
  const getIntensityColor = (value: number) => {
    if (value > 75)
      return "text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20";
    if (value > 50)
      return "text-orange-600 dark:text-orange-400 bg-orange-50 dark:bg-orange-900/20";
    if (value > 25)
      return "text-yellow-600 dark:text-yellow-400 bg-yellow-50 dark:bg-yellow-900/20";
    return "text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/20";
  };

  return (
    <div
      className="rounded-lg border-2 p-3 transition-colors"
      style={{
        borderColor: isAttacking ? teamColor : teamColor + "40",
        backgroundColor:
          isAttacking ? teamColor + "08" : "transparent",
      }}
    >
      <div className="flex items-start justify-between gap-2 mb-2">
        <div>
          <h3 className="font-heading font-bold text-sm text-gray-800 dark:text-gray-200">
            {teamName}
          </h3>
          <p className="text-xs text-gray-600 dark:text-gray-400">
            {pattern.name}
          </p>
        </div>
        <div className="text-xs font-heading px-2 py-1 rounded bg-gray-100 dark:bg-navy-700 text-gray-700 dark:text-gray-300">
          {isAttacking ? "ATK" : "DEF"}
        </div>
      </div>

      <p className="text-xs text-gray-700 dark:text-gray-300 mb-3">
        {description}
      </p>

      {/* Pattern Metrics */}
      <div className="space-y-1.5 text-xs">
        <MetricBar
          icon={<Zap className="w-3 h-3" />}
          label="Pressure"
          value={pattern.pressureIntensity}
          color={getIntensityColor(pattern.pressureIntensity)}
        />
        <MetricBar
          icon={<Shield className="w-3 h-3" />}
          label="Compactness"
          value={pattern.compactness}
          color={getIntensityColor(pattern.compactness)}
        />
        <MetricBar
          icon={<Activity className="w-3 h-3" />}
          label="Aggressiveness"
          value={pattern.aggressiveness}
          color={getIntensityColor(pattern.aggressiveness)}
        />
        <MetricBar
          icon={<Target className="w-3 h-3" />}
          label="Directness"
          value={pattern.directness}
          color={getIntensityColor(pattern.directness)}
        />
      </div>
    </div>
  );
}

interface MetricBarProps {
  icon: React.ReactNode;
  label: string;
  value: number;
  color: string;
}

function MetricBar({ icon, label, value, color }: MetricBarProps) {
  const getColorValue = (): string => {
    if (color.includes("text-red")) return "#dc2626";
    if (color.includes("text-orange")) return "#ea580c";
    if (color.includes("text-yellow")) return "#ca8a04";
    return "#16a34a";
  };

  return (
    <div className="flex items-center gap-2">
      <span className="text-gray-500 dark:text-gray-400">{icon}</span>
      <span className="w-16 text-gray-600 dark:text-gray-400">{label}:</span>
      <div className="flex-1 h-1.5 bg-gray-200 dark:bg-navy-700 rounded-full overflow-hidden">
        <div
          className="h-full bg-current transition-all duration-500"
          style={{
            width: `${value}%`,
            backgroundColor: getColorValue(),
          }}
        />
      </div>
      <span className={`w-8 text-right font-heading font-bold text-xs ${color}`}>
        {value}
      </span>
    </div>
  );
}
