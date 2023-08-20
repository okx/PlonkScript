<template>
  <q-page class="row">
    <div class="q-pa-md">
      <table>
        <tr v-for="(c, k) in rmapcolor" :key="k">
          <td :style="`border: 1px solid ${c};`">{{ k }}</td>
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
              :title="`${
                props.value.region ? `Region: ${props.value.region}\n` : ''
              }Raw: ${props.value.raw}`"
            />
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

type RowFieldType =
  | 'Unknown'
  | 'Unassigned'
  | 'Assigned'
  | 'Poison'
  | 'Selector';
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
    : 'negative';
}

function getColorByType(type: RowFieldType, value = ''): string {
  return type == 'Unassigned'
    ? 'grey'
    : type == 'Assigned'
    ? 'teal'
    : type == 'Poison'
    ? 'purple'
    : type == 'Selector'
    ? value == 'true'
      ? 'green'
      : 'indigo-3'
    : 'negative';
}

console.log(data);
const pagination = ref({
  page: 1,
  rowsPerPage: -1,
});
const columns: Ref<QTableColumn[]> = ref([]);

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
          ? prettifyCell(data.instance[i][j])
          : col.name == 'advice'
          ? prettifyCell(data.advice[i][j])
          : col.name == 'fixed'
          ? prettifyCell(data.fixed[i][j])
          : col.name == 'selector'
          ? prettifyCell(data.selectors[i][j])
          : prettifyCell(undefined);
      obj[`${col.field}${i}`] = {
        ...cell,
        index: j,
        region: rmap.value[j][`${col.name}-${i}`],
      };
    }
  }

  rows.value.push(obj);
}

console.log(rows.value, rmap.value, rmapcolor.value);

function prettifyCell(obj: object | string | string[] | undefined): RowField {
  if (obj === undefined) {
    return {
      type: 'Unknown',
    };
  }
  if (typeof obj === 'string') {
    if (obj == 'true' || obj == 'false') {
      return {
        type: 'Selector',
        raw: obj,
        value: obj,
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
