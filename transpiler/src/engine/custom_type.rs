use crate::system::*;
use crate::CONTEXT;

pub fn register_custom_type(engine: &mut rhai::Engine) {
    let _ = &mut engine
        .register_type_with_name::<Column>("Column")
        .register_indexer_get(Column::get_field)
        .register_indexer_set(Column::set_field)
        .register_type_with_name::<Cell>("Cell")
        .register_get("value", Cell::get_value);
}

impl Column {
    pub fn get_field(&mut self, index: i64) -> Cell {
        let name = self.get_field_name(index);
        unsafe {
            if CONTEXT.cells.contains_key(&name) {
                return CONTEXT.cells[&name].clone();
            }
        }

        Cell {
            name,
            index,
            value: Some("0".to_string()),
            column: self.clone(),
        }
    }
    pub fn set_field(&mut self, index: i64, value: Cell) {
        let name = self.get_field_name(index);
        let cell = Cell {
            name: name.clone(),
            index,
            value: value.value,
            ..value
        };
        unsafe {
            let entry = CONTEXT.cells.entry(name).or_insert(cell.clone()); //.and_modify(||cell);
            *entry = cell;
        }
    }

    fn get_field_name(&self, index: i64) -> String {
        let region = unsafe { CONTEXT.regions.last().unwrap().clone() };
        format!("{}[{}]_{}_{}", self.name, index, region.name, region.id)
    }
}

impl Cell {
    fn get_value(&mut self) -> String {
        self.value.clone().unwrap()
    }
}
