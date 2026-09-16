import { forwardRef, type ButtonHTMLAttributes } from 'react';
import { cn } from '@/lib/cn';

type Variant = 'primary' | 'secondary' | 'ghost' | 'danger';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: Variant;
}

const styles: Record<Variant, string> = {
  primary: 'bg-brand text-brand-ink hover:opacity-90',
  secondary: 'bg-surface-raised text-ink border border-border hover:bg-surface',
  ghost: 'bg-transparent text-ink hover:bg-surface-raised',
  danger: 'bg-danger text-white hover:opacity-90',
};

/** Accessible button: 44px min target, visible focus, forced-colors border. */
export const Button = forwardRef<HTMLButtonElement, ButtonProps>(function Button(
  { variant = 'primary', className, type, ...props },
  ref,
) {
  return (
    <button
      ref={ref}
      type={type ?? 'button'}
      className={cn(
        'fc-border inline-flex min-h-touch min-w-touch items-center justify-center gap-2 rounded-md px-4 py-2 text-sm font-semibold',
        'disabled:cursor-not-allowed disabled:opacity-50',
        styles[variant],
        className,
      )}
      {...props}
    />
  );
});
