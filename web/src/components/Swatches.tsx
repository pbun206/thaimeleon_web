import { useState } from "react";
import { Scheme } from "../types";

function labelColor(label: string, isDark: boolean): string {
  if (
    label.startsWith("surface") ||
    label === "base" ||
    label === "base high" ||
    label.startsWith("bg")
  ) {
    return "var(--fg)";
  }
  if (isDark && (label.startsWith("rg") || label === "muted")) {
    return "var(--fg)";
  }
  return "var(--surface-low)";
}

function Swatch({
  hex,
  label,
  isDark,
}: {
  hex: string;
  label: string;
  isDark: boolean;
}) {
  const [copied, setCopied] = useState(false);
  const copy = () => {
    navigator.clipboard.writeText(hex);
    setCopied(true);
    setTimeout(() => setCopied(false), 900);
  };
  return (
    <button
      className={"swatch" + (copied ? " copied" : "")}
      style={{ background: hex, color: labelColor(label, isDark) }}
      onClick={copy}
      title={`${label} ${hex}`}
    >
      <span className="hex">{copied ? "copied" : label}</span>
    </button>
  );
}

function Row({
  colors,
  labels,
  isDark,
}: {
  colors: string[];
  labels: string[];
  isDark: boolean;
}) {
  return (
    <div className="row">
      {colors.map((c, i) => (
        <Swatch key={i} hex={c} label={labels[i]} isDark={isDark} />
      ))}
    </div>
  );
}

export default function Swatches({ scheme }: { scheme: Scheme }) {
  const s = scheme.surfaces;
  const surfaceRow = [
    s.surface_low,
    s.base,
    s.base_high,
    s.surface_high,
    s.surface_higher,
    s.surface_highest,
    s.muted,
    s.subtext,
    s.text,
  ];
  const surfaceLabels = [
    "surface low",
    "base",
    "base high",
    "surface high",
    "surface higher",
    "surface highest",
    "muted",
    "subtext",
    "text",
  ];
  const namedKeys = [
    "red",
    "orange",
    "yellow",
    "green",
    "cyan",
    "blue",
    "purple",
    "magenta",
  ] as const;
  const named = (g: typeof scheme.fg_named) => namedKeys.map((k) => g[k]);
  const namedLabels = (prefix: string) =>
    namedKeys.map((k) => `${prefix} ${k}`);
  const accentLabels = (prefix: string) =>
    Array.from({ length: 6 }, (_, i) => `${prefix} ${i + 1}`);

  const isDark = !scheme.is_light_theme;
  return (
    <div className="scheme" style={{ background: s.base, color: s.text }}>
      <Row colors={surfaceRow} labels={surfaceLabels} isDark={isDark} />
      <Row
        colors={scheme.high_contrast_fg_accents}
        labels={accentLabels("hcfg")}
        isDark={isDark}
      />
      <Row
        colors={scheme.fg_accents}
        labels={accentLabels("fg")}
        isDark={isDark}
      />
      <Row
        colors={scheme.rg_accents}
        labels={accentLabels("rg")}
        isDark={isDark}
      />
      <Row
        colors={scheme.bg_accents}
        labels={accentLabels("bg")}
        isDark={isDark}
      />
      <details className="named-hues">
        <summary>named hues</summary>
        <Row
          colors={named(scheme.high_contrast_fg_named)}
          labels={namedLabels("hcfg")}
          isDark={isDark}
        />
        <Row
          colors={named(scheme.fg_named)}
          labels={namedLabels("fg")}
          isDark={isDark}
        />
        <Row
          colors={named(scheme.rg_named)}
          labels={namedLabels("rg")}
          isDark={isDark}
        />
        <Row
          colors={named(scheme.bg_named)}
          labels={namedLabels("bg")}
          isDark={isDark}
        />
      </details>
    </div>
  );
}
