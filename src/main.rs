use gtk::{prelude::*, Application, ApplicationWindow};

fn main() {
    let application = Application::builder()
        .application_id("dev.syndeecat.SyndeeBar")
        .build();
    application.connect_activate(|application| {
        activate(application);
    });
    application.run();
}

fn activate(application: &Application) {
    let main_window = ApplicationWindow::builder()
        .application(application)
        .build();
    gtk_layer_shell::init_for_window(&main_window);
    gtk_layer_shell::set_layer(&main_window, gtk_layer_shell::Layer::Top);
    gtk_layer_shell::auto_exclusive_zone_enable(&main_window);
    // let display = Display::default().expect("error get display");
    main_window.set_app_paintable(true);
}
