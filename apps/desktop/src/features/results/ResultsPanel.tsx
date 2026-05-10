import type { MechanicsResult } from "../../types";

export function ResultsPanel({ result }: { result: MechanicsResult | null }) {
  return (
    <section className="panel results-panel" aria-label="Results">
      <div className="panel-title">Results</div>
      {result ? (
        <table>
          <thead>
            <tr>
              <th>ID</th>
              <th>Kind</th>
              <th>Entity</th>
              <th>Value</th>
            </tr>
          </thead>
          <tbody>
            {result.results.map((item) => (
              <tr key={item.id}>
                <td>{item.id}</td>
                <td>{item.kind}</td>
                <td>{item.entity_ref}</td>
                <td>{item.value} {item.unit}</td>
              </tr>
            ))}
          </tbody>
        </table>
      ) : (
        <p className="muted">Run the bounded preview mechanics path to populate result summaries.</p>
      )}
    </section>
  );
}
