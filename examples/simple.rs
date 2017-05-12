extern crate tableau;
extern crate console;

use tableau::{Table, Cell};
use console::{Alignment, style};


fn main() {
    let mut table = Table::new();

    table.add_head_row()
        .add(Cell::new(style("ID").bold()))
        .add(Cell::new(style("Username").bold()))
        .add(Cell::new(style("Active").bold()));

    table.add_row()
        .add(Cell::new(style(1).cyan()))
        .add(Cell::new("john_doe"))
        .add(Cell::new("yes").align(Alignment::Center));

    table.add_row()
        .add(Cell::new(style(2).cyan()))
        .add(Cell::new("jane_doe"))
        .add(Cell::new("yes").align(Alignment::Center));

    table.add_row()
        .add(Cell::new(style(3).cyan()))
        .add(Cell::new("tazzzz"))
        .add(Cell::new("no").align(Alignment::Center));

    table.add_row()
        .add(Cell::new(style(4).cyan()))
        .add(Cell::new("tazzzzzzzzzzzzzzzzz"))
        .add(Cell::new("no").align(Alignment::Center));

    table.add_row()
        .add(Cell::new(style(4).cyan()))
        .add(Cell::new("This cell spans two. Because reasons.").colspan(2));

    table.display();
}
