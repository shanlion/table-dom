
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_cell_data(row: usize, col: usize) -> String {
    format!("R{}C{}", row, col)
}

#[wasm_bindgen]
pub fn get_visible_cells(scroll_top: usize, row_height: usize, viewport_height: usize, total_rows: usize) -> Vec<usize> {
    let start_row = scroll_top / row_height;
    let visible_rows = viewport_height / row_height + 1;
    let mut result = Vec::new();
    for r in start_row..(start_row + visible_rows).min(total_rows) {
        result.push(r);
    }
    result
}
