use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label};
use gtk4_layer_shell::{Layer, LayerShell, Edge};
use gtk4::gdk::Display;
use gtk4::{CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};

fn main() {
    let app = Application::builder()
        .application_id("com.example.layer-shell-app")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .build();

    let provider = CssProvider::new();
    provider.load_from_data(
        "
        window.my-shell-bar {
            background-color: rgba(0, 0, 0, 0.5);
        }
        
        /* This ensures the internal drawing area is also transparent */
        window.my-shell-bar > contents {
            background: transparent;
        }
        "
    );

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.add_css_class("my-shell-bar");
    window.init_layer_shell();
    window.set_namespace(Some("necroshell"));
    
    window.set_layer(Layer::Overlay);

    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);

    window.auto_exclusive_zone_enable();
    
    let label = Label::new(Some("𝖓𝖎𝖍𝖎𝖑"));
    label.set_margin_top(10);
    label.set_margin_bottom(10);
    
    window.set_child(Some(&label));
    
    window.present();
}