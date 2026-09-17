import { Info, Scale } from 'lucide-react';
import type { HelpTier, TierDecision, TierReason } from '@/lib/bindings';
import { useTranslation } from '@/i18n/useTranslation';
import type { TranslationKey } from '@/i18n';
import { cn } from '@/lib/cn';
import { GetHelpCard } from './GetHelpCard';

const tierKey: Record<HelpTier, TranslationKey> = {
  information: 'tier.information',
  generalGuidance: 'tier.generalGuidance',
  needsLawyer: 'tier.needsLawyer',
};

const reasonKey: Record<TierReason, TranslationKey> = {
  courtOrPolice: 'tier.reason.courtOrPolice',
  shortDeadline: 'tier.reason.shortDeadline',
  scamSignal: 'tier.reason.scamSignal',
  strategyQuestion: 'tier.reason.strategyQuestion',
  baseline: 'tier.reason.baseline',
};

/** The help-tier banner: icon + word + reasons. For the "needs a lawyer" tier
 * it also shows the get-help card. Never conveys tier by colour alone. */
export function HelpTierBanner({ decision }: { decision: TierDecision }) {
  const { t } = useTranslation();
  const isLawyer = decision.tier === 'needsLawyer';
  const Icon = isLawyer ? Scale : Info;

  return (
    <section
      aria-labelledby="tier-heading"
      className={cn(
        'fc-border rounded-lg border-2 p-4',
        isLawyer ? 'border-danger bg-danger/10' : 'border-border bg-surface-raised',
      )}
    >
      <h2 id="tier-heading" className="inline-flex items-center gap-2 text-lg font-bold">
        <Icon
          aria-hidden="true"
          className={cn('h-5 w-5', isLawyer ? 'text-danger' : 'text-info')}
        />
        {t(tierKey[decision.tier])}
      </h2>
      {isLawyer && <p className="mt-1 text-sm">{t('tier.needsLawyer.detail')}</p>}
      <ul className="mt-2 list-inside list-disc text-sm text-ink-muted">
        {decision.reasons.map((r) => (
          <li key={r}>{t(reasonKey[r])}</li>
        ))}
      </ul>
      {isLawyer && (
        <div className="mt-3">
          <GetHelpCard />
        </div>
      )}
    </section>
  );
}
