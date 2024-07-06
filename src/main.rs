use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TextView, Button, FileChooserDialog, FileChooserAction, Box as GtkBox, Orientation};
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use glib::clone;

fn main() {
    let application = Application::new(Some("com.example.SimpleTextEditor"), Default::default());

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Simple Text Editor")
            .default_width(800)
            .default_height(600)
            .build();

        let text_view = TextView::new();
        let text_buffer = text_view.buffer();

        let save_button = Button::with_label("Save");
        save_button.connect_clicked(clone!(@weak text_buffer, @weak window => move |_| {
            let file_chooser = FileChooserDialog::new(Some("Save File"), Some(&window), FileChooserAction::Save, &[("Save", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
            file_chooser.show();

            file_chooser.connect_response(clone!(@weak text_buffer => move |dialog, response| {
                if response == gtk::ResponseType::Ok {
                    if let Some(file_path) = dialog.file().and_then(|f| f.path()) {
                        let mut file = File::create(file_path).expect("Cannot create file");
                        let text = text_buffer.text(&text_buffer.start_iter(), &text_buffer.end_iter(), false);
                        file.write_all(text.as_bytes()).expect("Cannot write to file.");
                    }
                }
                dialog.close();
            }));
        }));

        let open_button = Button::with_label("Open");
        open_button.connect_clicked(clone!(@weak text_buffer, @weak window => move |_| {
            let file_chooser = FileChooserDialog::new(Some("Open File"), Some(&window), FileChooserAction::Open, &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
            file_chooser.show();

            file_chooser.connect_response(clone!(@weak text_buffer => move |dialog, response| {
                if response == gtk::ResponseType::Ok {
                    if let Some(file_path) = dialog.file().and_then(|f| f.path()) {
                        let mut file = OpenOptions::new().read(true).open(file_path).expect("Cannot open file");
                        let mut contents = String::new();
                        file.read_to_string(&mut contents).expect("Cannot read file");
                        text_buffer.set_text(&contents);
                    }
                }
                dialog.close();
            }));
        }));

        let hbox = GtkBox::new(Orientation::Horizontal, 0);
        hbox.append(&open_button);
        hbox.append(&save_button);

        let vbox = GtkBox::new(Orientation::Vertical, 0);
        vbox.append(&hbox);
        vbox.append(&text_view);
        window.set_child(Some(&vbox));

        window.show();
    });

    application.run();
}