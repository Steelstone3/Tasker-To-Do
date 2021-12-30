use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

//use crate::view::string_resources::string_resources::{APPLICATION_ID, APPLICATION_TITLE};

//mod view;

fn main() {
    let app = Application::builder().application_id("org.tasker.todo").build();

    app.connect_activate(build_ui);
    app.run();
}

pub fn build_ui(app: &Application) {
    let builder = gtk::Builder::from_string(include_str!("window.ui"));

    let window: ApplicationWindow = builder.object("window").expect("Could not get object `window` from builder.");
    
    window.set_application(Some(app));
    window.present();
}