import { QTableColumn } from 'quasar';

export type RowFieldType =
  | 'Unknown'
  | 'Unassigned'
  | 'Assigned'
  | 'Instance'
  | 'Poison'
  | 'Selector'
  | 'Gates';
export interface RowField {
  type: RowFieldType;
  raw?: string;
  value?: string | string[];
}
export interface RowFieldWithPosition extends RowField {
  // x: number;
  // y: number;
  index: number;
  region: string;
}

export type RotationType = ['Rotation', string];
export type PolynomialExpression =
  | ['Constant', string]
  // | ["Selector", ]
  | {
      type: 'Fixed' | 'Advice' | 'Instance';
      column_index: string;
      query_index: string;
      rotation: RotationType;
    }
  | ['Negated', PolynomialExpression]
  | ['Sum', PolynomialExpression, PolynomialExpression]
  | ['Product', PolynomialExpression, PolynomialExpression]
  | ['Scaled', PolynomialExpression, string];

export type CellValue = string[] | string;

export interface MockProverData {
  type: string;
  k: string;
  n: string;
  cs: ConstraintSystem;
  regions: RegionsEntity[];
  current_region: string;
  fixed: CellValue[][];
  advice: CellValue[][];
  instance: string[][];
  selectors: string[][];
  permutation: MockProverDataPermutation;
  usable_rows: string;
}

export interface ConstraintSystem {
  type: string;
  num_fixed_columns: string;
  num_advice_columns: string;
  num_instance_columns: string;
  num_selectors: string;
  selector_map: ColumnType[];
  gates: GatesEntity[];
  advice_queries: ColumnType[][];
  num_advice_queries?: string[] | null;
  instance_queries: ColumnType[][];
  fixed_queries: ColumnType[][];
  permutation: PermutationType;
  lookups?: undefined;
  constants: ColumnType[];
  minimum_degree: string;
}
export interface ColumnType {
  type: 'Column';
  index: string;
  column_type: 'Fixed' | 'Advice' | 'Instance';
}
export interface GatesEntity {
  type: string;
  name: string;
  constraint_names: string[];
  polys: PolynomialExpression[];
  queried_selectors: string[][];
  queried_cells: QueriedCellsEntity[];
}
export interface QueriedCellsEntity {
  type: string;
  column: ColumnType;
  rotation: string[];
}
export interface PermutationType {
  type: string;
  columns: ColumnType[];
}
export interface RegionsEntity {
  type: string;
  name: string;
  columns: ColumnType[];
  rows: (string | string[])[];
  enabled_selectors: EnabledSelector[];
  cells: [ColumnType, string][];
}
export type EnabledSelector = ['Selector', ...string[]];

export interface MockProverDataPermutation {
  type: string;
  columns: ColumnType[];
  mapping: string[][][];
  aux: string[][][];
  sizes: string[][];
}

function quoteIfIncludeAddSub(exp: string): string {
  if (exp.indexOf('+') >= 0 || exp.indexOf('-') >= 0) {
    return `(${exp})`;
  }

  return exp;
}

export function stringifyGate(polys: PolynomialExpression): string {
  if (Array.isArray(polys)) {
    if (polys[0] == 'Constant') return BigInt(polys[1]).toString();
    if (polys[0] == 'Negated') return `-${stringifyGate(polys[1])}`;
    if (polys[0] == 'Sum') {
      const second = stringifyGate(polys[2]);
      return `${stringifyGate(polys[1])}${
        second.startsWith('-') ? '' : '+'
      }${second}`;
    }
    if (polys[0] == 'Product') {
      return `${quoteIfIncludeAddSub(
        stringifyGate(polys[1])
      )}*${quoteIfIncludeAddSub(stringifyGate(polys[2]))}`;
    }
    if (polys[0] == 'Scaled') return `${stringifyGate(polys[1])}^^^${polys[2]}`;
  }

  // console.log('object polys', polys);

  //TODO: standardize column name getting
  const rotationHint = polys.rotation[1] == '0' ? '' : `[${polys.rotation[1]}]`;
  return `${polys.type[0].toLowerCase()}_${polys.column_index}${rotationHint}`;
}

export function convertGatesToStringifyDictionary(
  data: MockProverData
): Record<string, string> {
  const gates: Record<string, string> = {};
  for (let i = 0; i < data.cs.gates.length; i++) {
    const gate = data.cs.gates[i];
    gates[gate.name] = gate.polys
      .map((poly) => stringifyGate(poly as PolynomialExpression))
      .join(', ');
  }

  return gates;
}

interface ColumnDefinition {
  num: number;
  name: string;
  label: string;
  field: string;
}
export function getColumnDefinition(data: MockProverData): ColumnDefinition[] {
  return [
    {
      num: Number(data.cs.num_instance_columns),
      name: 'instance',
      label: 'i',
      field: 'i',
    },
    {
      num: Number(data.cs.num_advice_columns),
      name: 'advice',
      label: 'a',
      field: 'a',
    },
    {
      num: Number(data.cs.num_fixed_columns),
      name: 'fixed',
      label: 'f',
      field: 'f',
    },
    {
      num: Number(data.cs.num_selectors),
      name: 'selector',
      label: 's',
      field: 's',
    },
  ];
}
export function getColumns(cols: ColumnDefinition[]): QTableColumn[] {
  const columns: QTableColumn[] = [];

  for (let k = 0; k < cols.length; k++) {
    const col = cols[k];

    for (let i = 0; i < Number(col.num); i++) {
      columns.push({
        name: `${col.name}-${i}`,
        label: `${col.label}_${i}`,
        align: 'center',
        field: `${col.field}${i}`,
        sortable: false,
      });
    }
  }

  columns.push({
    name: 'gates',
    label: 'gates',
    align: 'center',
    field: 'gates',
    sortable: false,
  });
  return columns;
}

export function getRowsAndRegions(
  data: MockProverData,
  cols: ColumnDefinition[],
  colorList = ['red', 'blue', 'wheat', 'green']
) {
  const rows: Record<string, RowFieldWithPosition>[] = [];
  const rmap: Record<string, string>[] = [];
  const rmapcolor: Record<string, string> = {};
  const gates = convertGatesToStringifyDictionary(data);

  for (let j = 0; j < Number(data.n); j++) {
    const rrow: Record<string, string> = {};
    rmap.push(rrow);
  }

  for (let k = 0; k < data.regions.length; k++) {
    const region = data.regions[k];
    const rname = region.name;

    // for cells
    for (let i = 0; i < region.cells.length; i++) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const cell = region.cells[i] as any;
      const colname = `${cell[0].column_type.toLowerCase()}-${cell[0].index}`;
      const rowidx = Number(cell[1]);

      rmap[rowidx][colname] = rname;

      if (!rmapcolor[rname])
        rmapcolor[rname] =
          colorList[Object.keys(rmapcolor).length % colorList.length];
    }

    // for selectors
    for (let i = 0; i < region.enabled_selectors.length; i++) {
      const sel = region.enabled_selectors[i];
      const colname = `selector-${sel[1]}`;
      for (let j = 3; j < sel.length; j++) {
        const rowidx = Number(sel[j]);
        rmap[rowidx][colname] = rname;

        if (!rmapcolor[rname])
          rmapcolor[rname] =
            colorList[Object.keys(rmapcolor).length % colorList.length];
      }
    }
    for (const key in region.enabled_selectors) {
      if (Object.prototype.hasOwnProperty.call(region.enabled_selectors, key)) {
      }
    }
  }

  for (let j = 0; j < Number(data.n); j++) {
    const obj: Record<string, RowFieldWithPosition> = {};
    for (let k = 0; k < cols.length; k++) {
      const col = cols[k];

      for (let i = 0; i < Number(col.num); i++) {
        const cell =
          col.name == 'instance'
            ? prettifyCell(data.instance[i][j], col.name)
            : col.name == 'advice'
            ? prettifyCell(data.advice[i][j], col.name)
            : col.name == 'fixed'
            ? prettifyCell(data.fixed[i][j], col.name)
            : col.name == 'selector'
            ? prettifyCell(data.selectors[i][j], col.name)
            : prettifyCell(undefined, col.name);
        obj[`${col.field}${i}`] = {
          ...cell,
          index: j,
          region: rmap[j][`${col.name}-${i}`],
        };
      }
    }

    obj.gates = {
      type: 'Gates',
      index: j,
      region: '',
      value: getGatesDesc(data.selectors.map((_) => _[j])),
    };

    rows.push(obj);
  }

  function getGatesDesc(selector: string[]) {
    return data.cs.gates
      .filter((_) =>
        _.queried_selectors.some((s) => selector[Number(s[1])] == 'true')
      )
      .map((_) => _.name);
  }
  return { rows, gates, rmap, rmapcolor };
}

function prettifyCell(
  obj: object | string | string[] | undefined,
  column: string
): RowField {
  if (obj === undefined) {
    return {
      type: 'Unknown',
    };
  }
  if (typeof obj === 'string') {
    if ((obj == 'true' || obj == 'false') && column == 'selector') {
      return {
        type: 'Selector',
        raw: obj,
        value: obj,
      };
    }
    if (obj.startsWith('0x') && column == 'instance') {
      return {
        type: 'Instance',
        raw: obj,
        value: shortenCellValue(obj),
      };
    }
    return {
      type: obj == 'Unassigned' ? 'Unassigned' : 'Unknown',
      raw: obj,
    };
  }
  if (Array.isArray(obj) && obj.length == 2) {
    return {
      type:
        obj[0] == 'Assigned'
          ? 'Assigned'
          : obj[0] == 'Poison'
          ? 'Poison'
          : 'Unknown',
      value: shortenCellValue(obj[1]),
      raw: JSON.stringify(obj),
    };
  }

  return {
    type: 'Unknown',
    raw: JSON.stringify(obj),
  };
}

function shortenCellValue(value: string): string {
  try {
    const v = BigInt(value);
    if (v <= 9999999) {
      return v.toString();
    }

    if (value.length > 8) {
      return (
        value.substring(0, 4) +
        '..' +
        value.substring(value.length - 2, value.length)
      );
    }

    return value;
  } catch (e) {
    console.warn('cannot process value', value, e);
    return value;
  }
}

export function getPermutationLines(
  data: MockProverData,
  cellBadges: Record<string, Record<string, Element>>,
  columns: QTableColumn[],
  rows: Record<string, RowField>[]
): {
  from: Element;
  to: Element;
  equal: boolean;
  fromValue: string | undefined;
  toValue: string | undefined;
}[] {
  const colDict: Record<string, string> = columns.reduce(
    (pv, cv) => ({ ...pv, [cv.name]: cv.field }),
    {}
  );

  const mapping = data.permutation.mapping;
  const cols = data.permutation.columns;
  const lines = [];

  for (let c = 0; c < mapping.length; c++) {
    const mcol = mapping[c];
    for (let r = 0; r < mcol.length; r++) {
      const mrow = mcol[r];
      const col = Number(mrow[0]);
      const row = Number(mrow[1]);

      // from pointed address(col, row) to current cell(c, r)
      const tocolname = `${cols[c].column_type.toLowerCase()}-${cols[c].index}`;
      const fromcolname = `${cols[col].column_type.toLowerCase()}-${
        cols[col].index
      }`;
      if (fromcolname == tocolname && row == r) continue;
      const from = cellBadges[fromcolname][row];
      const to = cellBadges[tocolname][r];

      let fromValue = rows[row][colDict[fromcolname]].value;
      fromValue = Array.isArray(fromValue) ? fromValue.join(', ') : fromValue;
      let toValue = rows[r][colDict[tocolname]].value;
      toValue = Array.isArray(toValue) ? toValue.join(', ') : toValue;

      lines.push({ from, to, equal: fromValue == toValue, fromValue, toValue });
    }
  }

  return lines;
}
