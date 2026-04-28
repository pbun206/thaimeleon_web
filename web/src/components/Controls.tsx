import { useEffect, useState } from "react";
import { Config, DEFAULT_CONFIG, ThemeConfig } from "../types";

type Props = { config: Config; onChange: (c: Config) => void };

type DpsKey =
  | "set_2_dps_contrast"
  | "set_3_dps_contrast"
  | "set_4_dps_contrast"
  | "set_5_dps_contrast";

const DPS_KEYS: DpsKey[] = [
  "set_2_dps_contrast",
  "set_3_dps_contrast",
  "set_4_dps_contrast",
  "set_5_dps_contrast",
];

export default function Controls({ config, onChange }: Props) {
  const [draft, setDraft] = useState<Config>(config);
  const [jsonText, setJsonText] = useState(JSON.stringify(config, null, 2));
  const [jsonError, setJsonError] = useState<string | null>(null);

  useEffect(() => {
    setDraft(config);
    setJsonText(JSON.stringify(config, null, 2));
  }, [config]);

  const dirty = JSON.stringify(draft) !== JSON.stringify(config);

  const updateDps = (theme: "light" | "dark", key: DpsKey, value: number) => {
    const themeConfig: ThemeConfig = { ...draft[theme], [key]: value };
    setDraft({ ...draft, [theme]: themeConfig });
  };

  const applyConfig = () => {
    onChange(draft);
    setJsonText(JSON.stringify(draft, null, 2));
    setJsonError(null);
  };

  const applyJson = () => {
    try {
      const parsed = JSON.parse(jsonText) as Partial<Config>;
      const merged: Config = {
        ...DEFAULT_CONFIG,
        ...parsed,
        light: { ...DEFAULT_CONFIG.light, ...(parsed.light ?? {}) },
        dark: { ...DEFAULT_CONFIG.dark, ...(parsed.dark ?? {}) },
      };
      const k = Number(merged.k_means_count);
      if (!Number.isFinite(k) || k < 1 || k > 255) {
        throw new Error("k_means_count must be 1..255");
      }
      merged.k_means_count = Math.round(k);
      onChange(merged);
      setDraft(merged);
      setJsonText(JSON.stringify(merged, null, 2));
      setJsonError(null);
    } catch (e) {
      setJsonError(String(e));
    }
  };

  return (
    <details className="controls">
      <summary>advanced configuration</summary>
      {(["light", "dark"] as const).map((theme) => (
        <div key={theme} className="dps-group">
          <div className="dps-group-label">{theme} dps contrast</div>
          {DPS_KEYS.map((k) => {
            const min = 20;
            const max = 90;
            const v = draft[theme][k];
            const pct = Math.max(0, Math.min(1, (v - min) / (max - min))) * 100;
            return (
              <div className="row-control" key={k}>
                <span>{k.replace(/_/g, " ")}</span>
                <input
                  type="range"
                  min={min}
                  max={max}
                  step={0.5}
                  value={v}
                  style={{ "--p": `${pct}%` } as React.CSSProperties}
                  onChange={(e) => updateDps(theme, k, Number(e.target.value))}
                />
                <span>{v}</span>
              </div>
            );
          })}
        </div>
      ))}
      <div className="controls-body">
        <textarea
          value={jsonText}
          onChange={(e) => setJsonText(e.target.value)}
          rows={20}
          spellCheck={false}
        />
      </div>
      <div className="row-control">
        <button onClick={applyConfig} disabled={!dirty}>
          apply config{dirty ? " *" : ""}
        </button>
        <button onClick={applyJson}>apply json</button>
        <button onClick={() => onChange(DEFAULT_CONFIG)}>reset</button>
        <button
          onClick={() =>
            navigator.clipboard.writeText(JSON.stringify(config, null, 2))
          }
        >
          copy
        </button>
      </div>
      {jsonError && <p className="error">{jsonError}</p>}
    </details>
  );
}
