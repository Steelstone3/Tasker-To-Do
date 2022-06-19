use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder().application_id("org.tasker.todo").build();

    app.connect_activate(build_ui);
    app.run();
}

pub fn build_ui(app: &Application) {
    let builder = gtk::Builder::from_string(include_str!("./ui/tasker.ui"));

    let window: ApplicationWindow = builder.object("tasker").expect("Could not get object `window` from builder.");
    
    window.set_application(Some(app));
    window.present();
}