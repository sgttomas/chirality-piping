import type { MechanicsResult } from "../../types";

export function ResultsPanel({ result }: { result: MechanicsResult | null }) {
  const groups = result ? groupResults(result.results) : [];
  return (
    <section className="panel results-panel" aria-label="Results" data-testid="results-panel">
      <div className="panel-title">Results</div>
      {result ? (
        <div className="result-groups">
          {groups.map((group) => (
            <section
              key={group.title}
              className="result-group"
              aria-label={`${group.title} results`}
              data-testid={`result-group-${group.title.toLowerCase()}`}
            >
              <h3>{group.title}</h3>
              <table>
                <thead>
                  <tr>
                    <th>ID</th>
                    <th>Entity</th>
                    <th>Location</th>
                    <th>Value</th>
                  </tr>
                </thead>
                <tbody>
                  {group.items.map((item) => (
                    <tr key={item.id} data-testid={`result-row-${item.id}`}>
                      <td>{item.id}</td>
                      <td>{item.entity_ref}</td>
                      <td>{item.metadata?.location ?? "summary"}</td>
                      <td>
                        {item.value} {item.unit}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </section>
          ))}
        </div>
      ) : (
        <p className="muted">Run the bounded preview mechanics path to populate result summaries.</p>
      )}
    </section>
  );
}

function groupResults(resultItems: MechanicsResult["results"]) {
  const specs = [
    { title: "Displacement", match: (kind: string) => kind === "displacement_magnitude" },
    { title: "Reaction", match: (kind: string) => kind === "reaction_resultant" },
    { title: "Force", match: (kind: string) => kind.includes("_force") },
    { title: "Moment", match: (kind: string) => kind.includes("_moment") },
    { title: "Stress", match: (kind: string) => kind.includes("stress") }
  ];

  return specs
    .map((spec) => ({
      title: spec.title,
      items: resultItems.filter((item) => spec.match(item.kind))
    }))
    .filter((group) => group.items.length > 0);
}
