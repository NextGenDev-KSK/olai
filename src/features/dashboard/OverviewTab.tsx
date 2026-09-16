import type { Analysis } from '@/lib/bindings';
import { PaperCard } from './PaperCard';
import { DeadlineCountdown } from './DeadlineCountdown';
import { FindingsPanel } from '@/components/FindingsPanel';

/** The Overview tab: paper card, deadline countdowns, and any flagged text. */
export function OverviewTab({ analysis }: { analysis: Analysis }) {
  return (
    <div className="space-y-4">
      <PaperCard card={analysis.paperCard} />
      {analysis.deadlines.map((d) => (
        <DeadlineCountdown key={d.label} deadline={d} />
      ))}
      {analysis.findings.length > 0 && <FindingsPanel findings={analysis.findings} />}
    </div>
  );
}
