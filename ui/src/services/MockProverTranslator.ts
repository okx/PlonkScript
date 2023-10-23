import { MockProverData } from './ConstraintSystem';

export function convertMockProverOutputToObject(
  content: string
): MockProverData {
  return JSON.parse(convertMockProverOutputToJson(content)) as MockProverData;
}

export function convertMockProverOutputToJson(content: string): string {
  // protect original string
  const pairs: Record<string, string> = {};
  // const reString = /".*[\{\}\[\]\(\)]+.*"/g;
  const reString = /".*"/g;
  const matches = content.matchAll(reString);
  for (const m of matches) {
    const rnd = `"${Math.random()}"`;
    pairs[rnd] = m[0];
    content = content.replaceAll(m[0], rnd);
  }

  // combine hash map of tuple and array
  content = content.replaceAll('): [', '');
  content = content.replaceAll(/^(\s*)\): (\d+),/gm, '$1    $2\n$1),');

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

  // convert `celss` set to array
  content = content.replaceAll(
    /(cells: )\{(\n(?: {16}.*\n)+)( {12}})/g,
    '$1[$2            ]'
  );

  content = content.replaceAll(/({\s*)([^:]*)\s*\(/g, '$1"$2": (');

  content = content.replaceAll(/([\w\d]+) ?{/g, '{ type: $1,');

  content = content.replaceAll(/([\w\d]+)\(/g, '[ "$1",');

  // quote range value
  content = content.replace(/(\d+\.\.\d+)/g, '"$1"');

  // quote special number like: 17049590034418533166 + 12587749074617431523*a,
  content = content.replace(/(\d+ \+ \d+\*a)/g, '"$1"');

  // convert brackets
  content = content.replaceAll(/\(/g, '[');
  content = content.replaceAll(/\)/g, ']');

  // standardize quoted key
  content = content.replace(/(['"])?([\w\d]+)(['"])?:\s*/g, '"$2": ');

  // quote value to string
  content = content.replace(/:\s*([\w\d\.]+)/g, ': "$1"');

  // remove trailing comma
  content = content.replace(/,(\s*[\}\]])/g, '$1');

  // hex to string
  content = content.replace(/(0x[\da-f]+)/g, '"$1"');

  // prop key to string
  content = content.replace(/^(\s*)([\w\d]+)/gm, '$1"$2"');

  // restore protected string
  for (const key in pairs) {
    if (Object.prototype.hasOwnProperty.call(pairs, key)) {
      const val = pairs[key];
      content = content.replaceAll(key, val);
    }
  }

  return content;
}
