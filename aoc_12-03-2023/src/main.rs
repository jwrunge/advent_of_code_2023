use std::fs;

fn main() {
    let schematic = fs::read_to_string("schematic.txt").expect("Error reading file");
    
    let mut rows = schematic.lines();
    for row_idx in 0..rows.count() {
        let row = rows.nth(row_idx).unwrap();
        let mut cols = row.chars();     

        for col_idx in 0..cols.count() {
            let col = cols.nth(col_idx).unwrap();
            println!("row: {}, col: {}, char: {}", row_idx, col_idx, col);
        }
    }
}
