import pack from './legal_aid.json';

/** One legal-aid resource from the bundled knowledge pack. */
export interface LegalAidEntry {
  name: string;
  region: string;
  phone: string;
  sourceUrl: string;
  /** ISO date the team verified this entry, or null while it is a placeholder. */
  verifiedOn: string | null;
}

const entries = pack.entries as LegalAidEntry[];

/**
 * Legal-aid entries to display. In release builds only verified entries are
 * shown; in development the unverified placeholders are shown so the team can
 * see what needs filling in.
 */
export function visibleLegalAid(): LegalAidEntry[] {
  if (import.meta.env.PROD) {
    return entries.filter((e) => e.verifiedOn !== null);
  }
  return entries;
}
