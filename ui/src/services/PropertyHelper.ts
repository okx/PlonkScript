import { HashOut } from './plonky2/DefaultModels';

interface Property {
  name: string;
  value: string;
}

interface ArrayProperty {
  name: string;
  value: string[];
}

export function convertRustObjectToPropertyArray(
  obj: object | undefined
): Property[] {
  if (!obj) return [];
  const properties: Property[] = [];
  for (const [key, value] of Object.entries(obj)) {
    if (key == 'type') continue;
    if (
      typeof value === 'string' ||
      typeof value === 'number' ||
      typeof value === 'boolean'
    ) {
      properties.push({
        name: toTitleCase(key),
        value: value.toString(),
      });
    }
  }
  return properties;
}

export function filterRustObjectWithArrayProperties(
  obj: object | undefined
): ArrayProperty[] {
  if (!obj) return [];
  const properties: ArrayProperty[] = [];
  for (const [key, value] of Object.entries(obj)) {
    if (key == 'type') continue;
    if (
      Array.isArray(value) &&
      !Array.isArray(value.at(0)) &&
      !Array.isArray(value.at(1))
    ) {
      properties.push({
        name: toTitleCase(key),
        value: value,
      });
    }
  }
  return properties;
}

function toTitleCase(s: string) {
  return s
    .replace(/^num_/g, '')
    .replace(/^_*(.)|_+(.)/g, (s, c, d) =>
      c ? c.toUpperCase() : ' ' + d.toUpperCase()
    );
}

export function hashoutToHex(hashout: HashOut | undefined): string {
  if (!hashout) return '';
  return hashout.elements
    .map((_) => decimalToHex(_))
    .join('')
    .toUpperCase();
}

function decimalToHex(d: string | number, padding = 8) {
  let hex = Number(d).toString(16);
  padding =
    typeof padding === 'undefined' || padding === null
      ? (padding = 8)
      : padding;

  while (hex.length < padding) {
    hex = '0' + hex;
  }

  return hex;
}
