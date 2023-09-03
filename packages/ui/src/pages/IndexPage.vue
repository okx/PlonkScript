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
import o1 from '../assets/output.json';
import { QTableColumn } from 'quasar';
import LeaderLine from 'leader-line-new';
import {
  RowFieldType,
  getColumnDefinition,
  getColumns,
  MockProverData,
  getRowsAndRegions,
  RowField,
  RowFieldWithPosition,
  getPermutationLines,
} from 'src/services/ConstraintSystem';

const data = o1 as unknown as MockProverData;

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
const cols = getColumnDefinition(data);
const columns: Ref<QTableColumn[]> = ref(getColumns(cols));

const showTooltip = ref(false);
const showConstraints = ref(false);

function toggleConstraints() {
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (showConstraints.value) line.show();
    else line.hide();
  }
}

const rows: Ref<Record<string, RowFieldWithPosition>[]> = ref([]);
const rmap: Ref<Record<string, string>[]> = ref([]);
const rmapcolor: Ref<Record<string, string>> = ref({});

const colorList = ['red', 'blue', 'wheat', 'green'];
const rr = getRowsAndRegions(data, cols, colorList);
rows.value = rr.rows;
rmap.value = rr.rmap;
rmapcolor.value = rr.rmapcolor;

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

const cellBadges = ref<Record<string, Record<string, Element>>>({});
const lines: LeaderLine[] = [];

setTimeout(() => {
  const plines = getPermutationLines(
    data,
    cellBadges.value,
    columns.value,
    rows.value
  );

  for (let i = 0; i < plines.length; i++) {
    const { fromValue, toValue, from, to } = plines[i];

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
