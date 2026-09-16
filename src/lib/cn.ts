/** Join truthy class-name fragments (a tiny classnames helper). */
export function cn(...parts: (string | false | null | undefined)[]): string {
  return parts.filter(Boolean).join(' ');
}
