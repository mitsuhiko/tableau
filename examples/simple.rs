extern crate tableau;
extern crate console;

use tableau::Table;
use console::{Alignment, style};


fn main() {
    let mut table = Table::new();

    table.add_head_row()
        .add_cell("ID")
        .add_cell("Username")
        .add_cell("Active");

    table.add_row()
        .add_cell(style(1).cyan())
        .add_cell("john_doe")
        .add_cell("yes");

    table.add_row()
        .add_cell(style(2).cyan())
        .add_cell("jane_doe")
        .add_cell("yes");

    table.add_row()
        .add_cell(style(3).cyan())
        .add_cell("tazzzz")
        .add_cell("no");

    table.add_row()
        .add_cell(style(4).cyan())
        .add_cell("tazzzzzzzzzzzzzzzzz")
        .add_cell("no");

    table.display();
}
