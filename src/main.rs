use glib::clone;
use gtk4::prelude::*;
use gtk4::{
    glib, Application, ApplicationWindow, Box, Button, DropDown, Entry, Orientation, TextView,
};
use std::process::Command;

const APP_ID: &str = "org.zero.UnitCalc";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let tv_expr = TextView::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .hexpand(true)
        .vexpand(true)
        .build();
    // TODO: history
    // let dd_expr = DropDown::default();
    let tv_ans = TextView::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .hexpand(true)
        .vexpand(true)
        .build();

    let btn_calc = Button::builder()
        .label("Calculate")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    btn_calc.set_tooltip_text(Some("Calculate the Selection or Complete"));

    let btn_units = Button::builder()
        .label("Units")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    btn_units.set_tooltip_text(Some("Get a List of Compatible Units"));

    // let txt_ans = Entry::builder().hexpand(true).build();
    let txt_unit = Entry::builder().hexpand(true).build();

    btn_calc.connect_clicked(clone!(
        #[weak]
        tv_expr,
        #[weak]
        tv_ans,
        #[weak]
        txt_unit,
        move |_| {
            // if something is selected only use that
            let buf = tv_expr.buffer();
            let mut mark = buf.iter_at_mark(&buf.selection_bound());
            let mut ins = buf.iter_at_mark(&buf.get_insert());
            let is_selection = mark != ins;
            if !is_selection {
                (mark, ins) = (buf.start_iter(), buf.end_iter());
            };
            let expr = buf.text(&mark, &ins, false);
            let unit = txt_unit.text();
            let args = if unit.trim().is_empty() {
                vec!["-t", &expr]
            } else {
                vec!["-t", &expr, &unit]
            };
            match Command::new("units").args(&args).output() {
                Ok(out) => {
                    tv_ans
                        .buffer()
                        .set_text(String::from_utf8_lossy(&out.stdout).trim());
                }
                Err(e) => {
                    tv_ans.buffer().set_text(&e.to_string());
                }
            }
        }
    ));
    btn_units.connect_clicked(clone!(
        #[weak]
        tv_expr,
        #[weak]
        tv_ans,
        move |_| {
            let b = tv_expr.buffer();
            let expr = b.text(&b.start_iter(), &b.end_iter(), false);
            match Command::new("units")
                .args(["--conformable", &expr])
                .output()
            {
                Ok(out) => {
                    tv_ans
                        .buffer()
                        .set_text(String::from_utf8_lossy(&out.stdout).trim());
                }
                Err(e) => {
                    tv_ans.buffer().set_text(&e.to_string());
                }
            }
        }
    ));

    let bb2 = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .build();
    bb2.append(&btn_units);
    bb2.append(&txt_unit);
    bb2.append(&btn_calc);

    let bb1 = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .build();
    bb1.append(&tv_expr);
    bb1.append(&tv_ans);

    let bb = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .build();
    bb.append(&bb1);
    bb.append(&bb2);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Unit Calc")
        .child(&bb)
        .build();
    window.present();
}
