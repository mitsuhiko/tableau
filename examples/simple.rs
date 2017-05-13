extern crate tableau;
extern crate console;

use tableau::Table;
use console::{Alignment, Style};


fn main() {
    let mut table = Table::new();
    table.column_style(0, Style::new().cyan());
    table.column_alignment(1, Alignment::Center);

    table.add_head_row()
        .add_cell("ID")
        .add_cell("Username")
        .add_cell("Active");

    table.add_row()
        .add_cell(1)
        .add_cell("john_doe")
        .add_cell("yes");

    table.add_row()
        .add_cell(2)
        .add_cell("jane_doe")
        .add_cell("yes");

    table.add_row()
        .add_cell(3)
        .add_cell("tazzzz")
        .add_cell("no");

    table.add_row()
        .add_cell(4)
        .add_cell("tazzzzzzzzzzzzzzzzz")
        .add_cell("no");

    table.display();
}
