import { MockProverData } from './ConstraintSystem';

export function convertMockProverOutputToObject(
  content: string
): MockProverData {
  // protect original string
  const pairs: Record<string, string> = {};
  const reString = /".*[\{\}\[\]\(\)]+.*"/g;
  const matches = content.matchAll(reString);
  for (const m of matches) {
    const rnd = `"${Math.random()}"`;
    pairs[rnd] = m[0];
    content = content.replaceAll(m[0], rnd);
  }

  // convert `columns` set to array
  content = content.replaceAll(
    /(columns: )\{(\n(?: {16}.*\n)+)( {12}})/g,
    '$1[$2            ]'
  );

  // convert `enabled_selectors` set to array
  content = content.replaceAll(
    /(enabled_selectors: )\{(\n(?: {16}.*\n)+)( {12}})/g,
    '$1[$2            ]'
  );

  content = content.replaceAll(/({\s*)([^:]*)\s*\(/g, '$1"$2": (');

  content = content.replaceAll(/([\w\d]+) ?{/g, '{ type: $1,');
  content = content.replaceAll('): [', '');

  content = content.replaceAll(/([\w\d]+)\(/g, '[ "$1",');
  content = content.replaceAll(/\(/g, '[');
  content = content.replaceAll(/\)/g, ']');

  content = content.replace(/(['"])?([\w\d]+)(['"])?:\s*/g, '"$2": ');
  content = content.replace(/:\s*([\w\d\.]+)/g, ': "$1"');
  content = content.replace(/,(\s*[\}\]])/g, '$1');
  content = content.replace(/(0x[\da-f]+)/g, '"$1"');
  content = content.replace(/^(\s*)([\w\d]+)/gm, '$1"$2"');

  // restore protected string
  for (const key in pairs) {
    if (Object.prototype.hasOwnProperty.call(pairs, key)) {
      const val = pairs[key];
      content = content.replaceAll(key, val);
    }
  }

  return JSON.parse(content) as MockProverData;
}
