<template>
  <q-page class="row">
    <div class="q-pa-md">
      <q-checkbox v-model="showTooltip" label="Show Tooltip" />
      <q-checkbox
        v-model="showConstraints"
        label="Show Constraints"
        @click="toggleConstraints()"
      />

      <table>
        <tr v-for="(c, k) in rmapcolor" :key="k">
          <td :style="`border: 1px solid ${c};`">
            {{ k }}
          </td>
        </tr>
      </table>

      <q-table
        :rows="rows"
        :columns="columns"
        row-key="name"
        flat
        bordered
        dense
        :pagination="pagination"
        :hide-pagination="true"
      >
        <template v-slot:body-cell="props">
          <q-td
            :props="props"
            :class="
              'bg-' +
              getColorByColName(props.col.name) +
              ' ' +
              getBorderOfRegion(props.value, props.col)
            "
            :style="'border-color: ' + rmapcolor[props.value.region] + ';'"
          >
            <q-badge
              :color="getColorByType(props.value.type, props.value.value)"
              :label="props.value.type == 'Selector' ? '' : props.value.value"
              :ref="
                (el) => {
                  const elel = (el as any)?.$el;
                  if (!elel) return;
                  const col = props.col.name;
                  if (!cellBadges[col]) cellBadges[col] = {};
                  cellBadges[col][props.value.index] = elel;
                }
              "
            >
              <q-tooltip :delay="showTooltip ? 0 : 100000">
                {{
                  `${
                    props.value.region ? `Region: ${props.value.region}\n` : ''
                  }Raw: ${props.value.raw}`
                }}
              </q-tooltip>
            </q-badge>
          </q-td>
        </template>
        <template v-slot:header-cell="props">
          <q-th
            :props="props"
            :class="'bg-' + getColorByColName(props.col.name)"
          >
            {{ props.col.label }}
          </q-th>
        </template>
      </q-table>
    </div>
  </q-page>
</template>

<script setup lang="ts">
import { Ref, ref } from 'vue';
import data from '../assets/output.json';
import { QTableColumn } from 'quasar';
import LeaderLine from 'leader-line-new';

type RowFieldType =
  | 'Unknown'
  | 'Unassigned'
  | 'Assigned'
  | 'Instance'
  | 'Poison'
  | 'Selector'
  | 'Gates';
interface RowField {
  type: RowFieldType;
  raw?: string;
  value?: string;
}
interface RowFieldWithPosition extends RowField {
  // x: number;
  // y: number;
  index: number;
  region: string;
}

type RotationType = ['Rotation', string];
type PolynomialExpression =
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

const gates: Ref<Record<string, string>> = ref({});

function quoteIfIncludeAddSub(exp: string): string {
  if (exp.indexOf('+') >= 0 || exp.indexOf('-') >= 0) {
    return `(${exp})`;
  }

  return exp;
}

function stringifyGate(polys: PolynomialExpression): string {
  if (Array.isArray(polys)) {
    if (polys[0] == 'Constant') return polys[1];
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

for (let i = 0; i < data.cs.gates.length; i++) {
  const gate = data.cs.gates[i];
  gates.value[gate.name] = gate.polys
    .map((poly) => stringifyGate(poly as PolynomialExpression))
    .join(', ');
}
console.log(gates.value);

function getColorByColName(col: string): string {
  col = col.slice(0, col.indexOf('-'));
  return col == 'instance'
    ? 'grey'
    : col == 'advice'
    ? 'deep-orange-3'
    : col == 'fixed'
    ? 'light-blue-2'
    : col == 'selector'
    ? 'indigo-2'
    : col == 'gate'
    ? 'cyan-2'
    : 'negative';
}

function getColorByType(type: RowFieldType, value = ''): string {
  return type == 'Unassigned'
    ? 'grey'
    : type == 'Assigned'
    ? 'teal'
    : type == 'Instance'
    ? 'darkslategray'
    : type == 'Poison'
    ? 'purple'
    : type == 'Selector'
    ? value == 'true'
      ? 'green'
      : 'indigo-3'
    : type == 'Gates'
    ? 'cyan-8'
    : 'negative';
}

console.log(data);
const pagination = ref({
  page: 1,
  rowsPerPage: -1,
});
const columns: Ref<QTableColumn[]> = ref([]);

const showTooltip = ref(false);
const showConstraints = ref(false);

function toggleConstraints() {
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (showConstraints.value) line.show();
    else line.hide();
  }
}

const cols = [
  {
    num: data.cs.num_instance_columns,
    name: 'instance',
    label: 'i',
    field: 'i',
  },
  {
    num: data.cs.num_advice_columns,
    name: 'advice',
    label: 'a',
    field: 'a',
  },
  {
    num: data.cs.num_fixed_columns,
    name: 'fixed',
    label: 'f',
    field: 'f',
  },
  {
    num: data.cs.num_selectors,
    name: 'selector',
    label: 's',
    field: 's',
  },
];

for (let k = 0; k < cols.length; k++) {
  const col = cols[k];

  for (let i = 0; i < Number(col.num); i++) {
    columns.value.push({
      name: `${col.name}-${i}`,
      label: `${col.label}_${i}`,
      align: 'center',
      field: `${col.field}${i}`,
      sortable: false,
    });
  }
}

columns.value.push({
  name: 'gates',
  label: 'gates',
  align: 'center',
  field: 'gates',
  sortable: false,
});

const rows: Ref<Record<string, RowFieldWithPosition>[]> = ref([]);
const rmap: Ref<Record<string, string>[]> = ref([]);

function getBorderOfRegion(
  row: RowFieldWithPosition,
  col: QTableColumn
): string {
  let border = '';
  const y = row.index;
  const x = col.name;
  const name = row.region;
  if (!name) return '';
  if (rmap.value[y - 1] && rmap.value[y - 1][x] == name)
    border += ' no_border_top';
  else border += ' cell_border_top';
  if (rmap.value[y + 1] && rmap.value[y + 1][x] == name)
    border += ' no_border_bottom';
  else border += ' cell_border_bottom';
  const colidx = columns.value.findIndex((_) => _.name == x);
  const left = columns.value[colidx - 1]?.name;
  if (left && rmap.value[y][left] == name) border += ' no_border_left';
  else border += ' cell_border_left';
  const right = columns.value[colidx + 1]?.name;
  if (right && rmap.value[y][right] == name) border += ' no_border_right';
  else border += ' cell_border_right';

  return border;
}

const colorList = ['red', 'blue', 'wheat', 'green'];
const rmapcolor: Ref<Record<string, string>> = ref({});

for (let j = 0; j < Number(data.n); j++) {
  const rrow: Record<string, string> = {};
  rmap.value.push(rrow);
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

    rmap.value[rowidx][colname] = rname;

    if (!rmapcolor.value[rname])
      rmapcolor.value[rname] =
        colorList[Object.keys(rmapcolor.value).length % colorList.length];
  }

  // for selectors
  // for (let i = 0; i < region.enabled_selectors.length; i++) {
  //   const sel = region.enabled_selectors[i];

  // }
  for (const key in region.enabled_selectors) {
    if (Object.prototype.hasOwnProperty.call(region.enabled_selectors, key)) {
      const sel = (region.enabled_selectors as { [key: string]: string[] })[
        key
      ];
      const colname = `selector-${sel[0]}`;
      const rowidx = Number(sel[2]);
      rmap.value[rowidx][colname] = rname;

      if (!rmapcolor.value[rname])
        rmapcolor.value[rname] =
          colorList[Object.keys(rmapcolor.value).length % colorList.length];
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
        region: rmap.value[j][`${col.name}-${i}`],
      };
    }
  }

  obj.gates = {
    type: 'Gates',
    index: j,
    region: '',
    value: getGatesDesc(data.selectors.map((_) => _[j])),
  };

  rows.value.push(obj);
}

function getGatesDesc(selector: string[]) {
  console.log(selector);
  return data.cs.gates
    .filter((_) =>
      _.queried_selectors.some((s) => selector[Number(s[1])] == 'true')
    )
    .map((_) => gates.value[_.name])
    .join('\n');
}

console.log(rows.value, rmap.value, rmapcolor.value);

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
        value: BigInt(obj).toString(),
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
      value: BigInt(obj[1]).toString(),
      raw: JSON.stringify(obj),
    };
  }

  return {
    type: 'Unknown',
    raw: JSON.stringify(obj),
  };
}

const cellBadges = ref<Record<string, Record<string, Element>>>({});
const lines: LeaderLine[] = [];
const colDict: Record<string, string> = columns.value.reduce(
  (pv, cv) => ({ ...pv, [cv.name]: cv.field }),
  {}
);
setTimeout(() => {
  const mapping = data.permutation.mapping;
  const cols = data.permutation.columns;
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
      const from = cellBadges.value[fromcolname][row];
      const to = cellBadges.value[tocolname][r];

      const fromValue = rows.value[row][colDict[fromcolname]].value;
      const toValue = rows.value[r][colDict[tocolname]].value;

      const color = fromValue == toValue ? 'wheat' : 'crimson';
      const outlineColor = fromValue == toValue ? 'tan' : 'coral';

      const line = new LeaderLine(
        LeaderLine.mouseHoverAnchor(from as HTMLElement, 'fade', {
          style: {
            backgroundImage: null,
            backgroundColor: null,
            paddingRight: null,
          },
          hoverStyle: {
            backgroundColor: null,
          },
        }),
        to,
        {
          color,
          path: 'straight',
          size: 4,
          outline: true,
          endPlug: 'behind',
          outlineColor,
          dash: { animation: true, gap: 4 },
        }
      );
      lines.push(line);
    }
  }
}, 1000);
</script>

<style scoped lang="scss">
$pos: left, right, top, bottom;

@each $p in $pos {
  .no_border_#{$p} {
    border-#{$p}-width: 0 !important;
  }

  .cell_border_#{$p} {
    border-#{$p}-width: 1px;
  }
}
</style>
