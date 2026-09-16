import { LifeBuoy, ExternalLink } from 'lucide-react';
import { visibleLegalAid } from '@/data/legalAid';

/** A "get help" card listing bundled legal-aid resources. Placeholder
 * (unverified) entries appear only in development; release hides them. */
export function GetHelpCard() {
  const entries = visibleLegalAid();
  return (
    <section
      aria-label="Get help"
      className="fc-border rounded-md border border-border bg-surface-raised p-4"
    >
      <h3 className="mb-2 inline-flex items-center gap-2 font-semibold">
        <LifeBuoy aria-hidden="true" className="h-4 w-4" />
        Free legal aid
      </h3>
      {entries.length === 0 ? (
        <p className="text-sm text-ink-muted">
          Search for your nearest District or State Legal Services Authority for free legal aid.
        </p>
      ) : (
        <ul className="space-y-2">
          {entries.map((e) => (
            <li key={e.name} className="text-sm">
              <p className="font-medium">{e.name}</p>
              <p className="text-ink-muted">{e.region}</p>
              <p className="text-ink-muted">{e.phone}</p>
              {e.verifiedOn === null ? (
                <p className="text-xs italic text-warning">Placeholder — not yet verified.</p>
              ) : (
                <a
                  href={e.sourceUrl}
                  className="inline-flex items-center gap-1 text-brand underline"
                  target="_blank"
                  rel="noreferrer"
                >
                  {e.sourceUrl}
                  <ExternalLink aria-hidden="true" className="h-3 w-3" />
                </a>
              )}
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
