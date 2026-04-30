import { useEffect, useState } from "react";
import init, { generate_scheme } from "./wasm/thaimeleon_lib.js";
import { Config, DEFAULT_CONFIG, Scheme } from "./types";
import ImagePicker, { decodeFromUrl } from "./components/ImagePicker";
import Swatches from "./components/Swatches";
import Controls from "./components/Controls";
import Results from "./components/Results";
import ThemeToggle, { Mode } from "./components/ThemeToggle";
import Info from "./components/Info";

type Pair = { light: Scheme; dark: Scheme } | null;

function applyScheme(s: Scheme) {
  const r = document.documentElement.style;
  r.setProperty("--bg", s.surfaces.base);
  r.setProperty("--fg", s.surfaces.text);
  r.setProperty("--subtext", s.surfaces.subtext);
  r.setProperty("--muted", s.surfaces.muted);
  r.setProperty("--surface-low", s.surfaces.surface_low);
  r.setProperty("--surface-high", s.surfaces.surface_high);
  r.setProperty("--surface-highest", s.surfaces.surface_highest);
  r.setProperty("--faint", s.surfaces.faint);
  r.setProperty("--hairline", s.surfaces.muted);
  r.setProperty("--rg-accent-1", s.rg_accents[0]);
  r.setProperty("--fg-accent-1", s.fg_accents[0]);
  r.setProperty("--fg-red", s.fg_named.red);
}

export default function App() {
  const [pixels, setPixels] = useState<{
    rgba: Uint8Array;
    width: number;
    height: number;
    url: string;
  } | null>(null);
  const [schemes, setSchemes] = useState<Pair>(null);
  const [config, setConfig] = useState<Config>(DEFAULT_CONFIG);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [defaultFailed, setDefaultFailed] = useState(false);
  const [prefersDark, setPrefersDark] = useState(
    () => window.matchMedia("(prefers-color-scheme: dark)").matches,
  );
  const [mode, setMode] = useState<Mode>(
    () => (localStorage.getItem("themeMode") as Mode) || "dark",
  );

  useEffect(() => {
    localStorage.setItem("themeMode", mode);
  }, [mode]);

  useEffect(() => {
    let cancelled = false;
    decodeFromUrl("default.webp")
      .then((p) => {
        if (!cancelled) setPixels(p);
      })
      .catch(() => {
        if (!cancelled) setDefaultFailed(true);
      });
    return () => {
      cancelled = true;
    };
  }, []);

  const effectiveDark = mode === "dark" || (mode === "auto" && prefersDark);

  useEffect(() => {
    document.documentElement.classList.toggle("light", !effectiveDark);
  }, [effectiveDark]);

  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const fn = (e: MediaQueryListEvent) => setPrefersDark(e.matches);
    mq.addEventListener("change", fn);
    return () => mq.removeEventListener("change", fn);
  }, []);

  useEffect(() => {
    if (!pixels) return;
    setLoading(true);
    setError(null);
    let cancelled = false;
    init()
      .then(() => {
        if (cancelled) return;
        try {
          const light = generate_scheme(
            pixels.rgba,
            pixels.width,
            pixels.height,
            1,
            config,
          ) as Scheme;
          const dark = generate_scheme(
            pixels.rgba,
            pixels.width,
            pixels.height,
            0,
            config,
          ) as Scheme;
          if (!cancelled) setSchemes({ light, dark });
        } catch (e) {
          if (!cancelled) {
            const msg = e instanceof Error ? e.message : String(e);
            setError(`scheme generation failed: ${msg}`);
          }
        } finally {
          if (!cancelled) setLoading(false);
        }
      })
      .catch((e) => {
        if (!cancelled) {
          setError(`wasm init failed: ${String(e)}`);
          setLoading(false);
        }
      });
    return () => {
      cancelled = true;
    };
  }, [pixels, config]);

  useEffect(() => {
    if (!schemes) return;
    applyScheme(effectiveDark ? schemes.dark : schemes.light);
  }, [schemes, effectiveDark]);

  const picker = (
    <ImagePicker
      onImage={(p) => {
        setPixels(p);
        setError(null);
      }}
      onError={setError}
      preview={pixels?.url}
    />
  );

  return (
    <main>
      <ThemeToggle mode={mode} onChange={setMode} />
      {schemes ? (
        <>
          {picker}
          {loading && <p className="loading">generating…</p>}
          {error && <p className="error">{error}</p>}
          {!loading && (
            <Swatches scheme={effectiveDark ? schemes.dark : schemes.light} />
          )}
          {!loading && <Results light={schemes.light} dark={schemes.dark} />}
          <Controls config={config} onChange={setConfig} />
          <Info />
        </>
      ) : defaultFailed ? (
        <>
          {picker}
          {error && <p className="error">{error}</p>}
        </>
      ) : (
        <p className="loading">loading…</p>
      )}
    </main>
  );
}
