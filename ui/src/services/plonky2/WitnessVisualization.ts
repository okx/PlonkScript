import { QTableColumn } from 'quasar';
import { RowFieldWithPosition, Witness } from './DefaultModels';

export function getColumns(cols: number): QTableColumn[] {
  const columns: QTableColumn[] = [];
  columns.push({
    name: 'index',
    label: 'idx',
    align: 'center',
    field: 'index',
    sortable: false,
  });

  for (let i = 0; i < cols; i++) {
    columns.push({
      name: `${i}`,
      label: `${i}`,
      align: 'center',
      field: `${i}`,
      sortable: false,
    });
  }

  return columns;
}

export function getRows(
  witness: Witness
): Record<string, RowFieldWithPosition>[] {
  const rows: Record<string, RowFieldWithPosition>[] = [];
  const arr = witness.values;
  const rm = witness.representative_map;
  const colnum = parseInt(witness.num_wires);
  const rownum = parseInt(witness.degree);
  const total = colnum * rownum;

  for (let r = 0; r < rownum; r++) {
    const row: Record<string, RowFieldWithPosition> = {};
    (row as any)['index'] = r;
    for (let c = 0; c < colnum; c++) {
      const idx = r * colnum + c;
      const rmval = parseInt(rm[idx]);
      row[c.toString()] = {
        value: arr[idx],
        index: r,
        raw_index: idx,
        representative_map: rmval,
        row: Math.floor(rmval / colnum),
        col: rmval % colnum,
      };
    }

    rows.push(row);
  }

  {
    const row: Record<string, RowFieldWithPosition> = {};
    for (let i = 0; i < arr.length - total; i++) {
      const idx = i + total;
      const rmval = parseInt(rm[idx]);
      row[i] = {
        value: arr[idx],
        index: rownum,
        raw_index: idx,
        representative_map: rmval,
        row: Math.floor(rmval / colnum),
        col: rmval % colnum,
      };
    }

    (row as any)['index'] = row[0].row;
    rows.push(row);
  }

  return rows;
}
