use std::io;
use std::fmt;
use std::io::Write;
use std::cmp::max;

use console::{Alignment, Style, pad_str, measure_text_width};

use style::TableStyle;
use utils::evenly_split_width;

/// Represents a cell in a table.
pub struct Cell {
    contents: Option<String>,
    colspan: usize,
    style: Option<Style>,
    alignment: Option<Alignment>,
}

/// Represents a row in a table.
pub struct Row {
    cells: Vec<Cell>,
    is_head: bool,
    effective_cell_count: usize,
}

/// A helper for formatting tables.
pub struct Table {
    rows: Vec<Row>,
    style: TableStyle,
}

impl Table {
    /// Creates a new empty table.
    pub fn new() -> Table {
        Table {
            rows: vec![],
            style: Default::default(),
        }
    }

    /// Adds a new row to the table and returns a mutable reference to it.
    pub fn add_row(&mut self) -> &mut Row {
        let row = Row {
            cells: vec![],
            is_head: false,
            effective_cell_count: 0,
        };
        let idx = self.rows.len();
        self.rows.push(row);
        &mut self.rows[idx]
    }

    /// Adds a new header row to a table and returns a mutable reference to it.
    pub fn add_head_row(&mut self) -> &mut Row {
        let mut rv = self.add_row();
        rv.is_head = true;
        rv
    }

    fn inner_dimensions(&self) -> (usize, usize) {
        let mut width = 0;
        for row in &self.rows {
            width = max(width, row.calculate_widths(
                &self.style).into_iter().sum());
        }
        (width, self.rows.len())
    }

    fn column_widths(&self) -> Vec<usize> {
        let mut widths = vec![];
        let mut off = 0;
        for row in &self.rows {
            for cell in row.cells.iter().enumerate() {
                let row_widths = row.calculate_widths(&self.style);
                if row_widths.len() > widths.len() {
                    widths.resize(row_widths.len(), 0);
                }
                for (idx, width) in row_widths.into_iter().enumerate() {
                    widths[idx] = max(widths[idx], width);
                }
            }
        }
        widths
    }

    /// Shortcut to format a table to stdout.
    pub fn display(&self) {
        write!(io::stdout(), "{}", self).unwrap();
        io::stdout().flush().unwrap();
    }
}

impl Row {
    /// Adds a new call to a table.
    ///
    /// This returns a mutable reference to self so that calls can easily
    /// be chained.
    pub fn add_cell<C: Into<Cell>>(&mut self, cell: C) -> &mut Row {
        let cell = cell.into();
        self.effective_cell_count += cell.colspan;
        self.cells.push(cell);
        self
    }

    fn calculate_widths(&self, style: &TableStyle) -> Vec<usize> {
        let mut cols = vec![];
        let last_cell = self.cells.len() - 1;
        for (idx, cell) in self.cells.iter().enumerate() {
            let width = cell.inner_width() + (style.padding_left + style.padding_right) as usize;
            cols.extend_from_slice(&evenly_split_width(width, cell.colspan));
        }
        cols
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn line(f: &mut fmt::Formatter, w: &[usize], c: Option<char>, c_left: Option<char>,
                c_mid: Option<char>, c_right: Option<char>) -> fmt::Result
        {
            use std::fmt::Write;
            let mut rv = String::new();
            if let Some(s) = c_left {
                rv.push(s);
            }
            if let Some(c) = c {
                for (idx, &width) in w.iter().enumerate() {
                    if idx > 0 {
                        rv.push(c_mid.unwrap_or(c));
                    }
                    for _ in 0..width {
                        rv.push(c);
                    }
                }
            }
            if let Some(s) = c_right {
                rv.push(s);
            }
            if !rv.is_empty() {
                write!(f, "{}\n", rv)?;
            }
            Ok(())
        }

        let s = &self.style;
        let widths = self.column_widths();
        let last_row = self.rows.len() - 1;
        let mut was_head = false;
        for (idx, row) in self.rows.iter().enumerate() {
            let mut buf = String::new();
            if row.is_head || was_head {
                line(f, &widths[..], s.head, s.head_left, s.head_mid, s.head_right);
            } else if idx == 0 {
                line(f, &widths[..], s.top, s.top_left, s.top_mid, s.top_right);
            } else {
                line(f, &widths[..], s.mid, s.mid_left, s.mid_mid, s.mid_right);
            }
            let last_cell = row.cells.len() - 1;
            let mut offset = 0;
            for (base_idx, c) in row.cells.iter().enumerate() {
                let idx = base_idx + offset;
                if idx == 0 {
                    if let Some(c) = s.left {
                        buf.push(c);
                    }
                } else {
                    if let Some(c) = s.middle {
                        buf.push(c);
                    }
                }
                for _ in 0..s.padding_left {
                    buf.push(' ');
                }
                let text = c.contents.as_ref().map(|x| x.as_str()).unwrap_or("");
                let style = c.style.as_ref().unwrap_or_else(|| {
                    if row.is_head {
                        &s.default_header_style
                    } else {
                        &s.default_text_style
                    }
                });
                let inner_width = measure_text_width(text);
                let cell_width = 
                    widths[idx..idx + c.colspan].into_iter().sum::<usize>() -
                    (s.padding_left + s.padding_right) as usize +
                    if s.middle.is_some() {
                        c.colspan - 1
                    } else {
                        0
                    };
                let alignment = c.alignment.unwrap_or_else(|| {
                    if row.is_head {
                        Alignment::Center
                    } else {
                        Alignment::Left
                    }
                });
                buf.push_str(&pad_str(&style.apply_to(text).to_string(),
                    cell_width, alignment, None));
                for _ in 0..s.padding_right {
                    buf.push(' ');
                }
                if base_idx == last_cell {
                    if let Some(c) = s.right {
                        buf.push(c);
                    }
                }
                offset += c.colspan - 1;
            }
            write!(f, "{}\n", buf);
            if idx == last_row {
                line(f, &widths[..], s.bottom, s.bottom_left, s.bottom_mid, s.bottom_right);
            }
            was_head = row.is_head;
        }

        Ok(())
    }
}

impl Cell {
    /// Creates a new cell with some given text.
    pub fn new<S: ToString>(val: S) -> Cell {
        Cell {
            contents: Some(val.to_string()),
            colspan: 1,
            style: None,
            alignment: None,
        }
    }

    fn inner_width(&self) -> usize {
        match self.contents {
            Some(ref s) => measure_text_width(s),
            None => 0,
        }
    }

    /// Sets the alignment for the cell.
    pub fn align(&mut self, alignment: Alignment) -> &mut Cell {
        self.alignment = Some(alignment);
        self
    }

    /// Sets the colspan for the cell.
    pub fn colspan(&mut self, span: usize) -> &mut Cell {
        self.colspan = span;
        self
    }
}

impl<S: ToString> From<S> for Cell {
    fn from(text: S) -> Cell {
        Cell::new(text.to_string())
    }
}
