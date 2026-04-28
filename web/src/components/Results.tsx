import { Scheme } from "../types";

type Props = { light: Scheme; dark: Scheme };

export default function Results({ light, dark }: Props) {
  const text = JSON.stringify({ light, dark }, null, 2);
  return (
    <details className="controls">
      <summary>results json</summary>
      <div className="controls-body">
        <textarea value={text} readOnly rows={12} spellCheck={false} />
      </div>
      <div className="row-control">
        <button onClick={() => navigator.clipboard.writeText(text)}>
          copy
        </button>
      </div>
    </details>
  );
}
