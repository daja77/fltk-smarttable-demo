use chrono::Utc;
use fltk::{prelude::*, *};
use fltk::app::Scheme::Gtk;
use fltk::group::Pack;
use fltk_table::{SmartTable, TableOpts};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

fn main() {
    let a = app::App::default().with_scheme(Gtk);

    let mut win = window::Window::default().with_size(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut pack = Pack::new(10, 10, WINDOW_WIDTH - 20, WINDOW_HEIGHT - 20, "");

    let mut table = SmartTable::default()
        .with_size(700, 500)
        .center_of_parent()
        .with_opts(TableOpts {
            rows: 2,
            cols: 3,
            editable: false,
            ..std::default::Default::default()
        });
    table.set_row_header_width(0);
    table.set_col_header_value(0, "Version");
    table.set_col_width(0, 170);
    table.set_col_header_value(1, "Date");
    table.set_col_width(1, 220);
    table.set_col_header_value(2, "Comment");
    table.set_col_width(2, 210);

    table.set_cell_value(0, 0, "1");
    table.set_cell_value(1, 0, "2");

    let date = Utc::today().naive_utc().to_string();
    table.set_cell_value(0, 1, date.clone().as_str());
    table.set_cell_value(1, 1, date.as_str());

    table.set_cell_value(0, 2, "first");
    table.set_cell_value(1, 2, "second");

    let app = a.clone();
    table.set_callback(|t| {
        if app::event() == enums::Event::Push && t.callback_context() == table::TableContext::Cell {
            println!("callback called!");
            let r = t.callback_row();
            println!("got called by row no: {}", r);
            let c = t.callback_col();
            println!("got called by col no: {}", c);
        }
    });
    pack.end();
    win.end();
    win.show();
    a.run().unwrap();
}
