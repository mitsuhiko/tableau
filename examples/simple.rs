extern crate tableau;
extern crate console;

use tableau::Table;
use console::{Alignment, Style};


fn main() {
    let mut table = Table::new();
    table.column_style(0, Style::new().cyan());
    table.column_alignment(1, Alignment::Center);

    table.add_head()
        .add("ID")
        .add("Username")
        .add("Active");

    table.add()
        .add(1)
        .add("john_doe")
        .add("yes");

    table.add()
        .add(2)
        .add("jane_doe")
        .add("yes");

    table.add()
        .add(3)
        .add("tazzzz")
        .add("no");

    table.add()
        .add(4)
        .add("tazzzzzzzzzzzzzzzzz")
        .add("no");

    table.display();
}
