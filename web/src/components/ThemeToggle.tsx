export type Mode = "auto" | "light" | "dark";

type Props = { mode: Mode; onChange: (m: Mode) => void };

const opts: Mode[] = ["light", "dark", "auto"];

export default function ThemeToggle({ mode, onChange }: Props) {
  return (
    <div className="theme-toggle">
      <span className="title">thaimeleon web</span>
      <div className="theme-toggle-buttons">
        {opts.map((m) => (
          <button
            key={m}
            className={mode === m ? "active" : ""}
            onClick={() => onChange(m)}
          >
            {m}
          </button>
        ))}
      </div>
    </div>
  );
}
