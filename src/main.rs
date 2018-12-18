extern crate gtk;

use gtk::prelude::*;

fn main() {
    // Init GTK
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    // We get the file content.
    let glade_src = include_str!("glade/main_form.glade");
    // Then we call the Builder call.
    let builder = gtk::Builder::new_from_string(glade_src);

    // Our window id is "window1".
    let window: gtk::Window = builder.get_object("main_window").unwrap();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.show_all();

    // We start the gtk main loop.
    gtk::main();
}