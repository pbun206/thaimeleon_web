import { useState } from "react";
import { Scheme } from "../types";
import { Format, formatScheme } from "../colorFormat";

type Props = { light: Scheme; dark: Scheme };

const FORMATS: Format[] = ["rgb", "rgba", "oklch"];

export default function Results({ light, dark }: Props) {
  const [fmt, setFmt] = useState<Format>("rgb");
  const [copied, setCopied] = useState(false);
  const text = JSON.stringify(
    { light: formatScheme(light, fmt), dark: formatScheme(dark, fmt) },
    null,
    2,
  );
  const copy = () => {
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 900);
  };
  return (
    <details className="controls">
      <summary>results json</summary>
      <div className="row-control format-toggle">
        {FORMATS.map((f) => (
          <button
            key={f}
            className={fmt === f ? "active" : ""}
            onClick={() => setFmt(f)}
          >
            {f}
          </button>
        ))}
      </div>
      <div className="controls-body">
        <textarea value={text} readOnly rows={12} spellCheck={false} />
      </div>
      <div className="row-control">
        <button onClick={copy}>{copied ? "copied!" : "copy"}</button>
      </div>
    </details>
  );
}
