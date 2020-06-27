extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Button, Calendar};

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 350);

        let window2 = ApplicationWindow::new(app);
        window2.set_title("second");
        window2.set_default_size(400, 400);

        let button = Button::new_with_label("Click me!");
        let btn = Button::new_with_label("Press me!");

        let calendar = Calendar::new();
        let mut counter = 0;
        button.connect_clicked(|_| {
            println!("Clicked!");
        });

        btn.connect_clicked(move |but| {
            println!("Pressed!");
            let mut label = String::from(&counter.to_string());
            but.set_label(&label);
        });

        //window.add(&button);
        let a = &btn;
        window.add(&calendar);
        window2.add(a);
        btn.show_all();
        window.show_all();
        window2.show_all();
    });

    application.run(&[]);
}
