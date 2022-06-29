use adw::{Application, ApplicationWindow, HeaderBar};
use arti_client::{TorClient, TorClientConfig};
use glib::clone;
use gtk::Box;
use gtk::{prelude::*, Orientation};
use tor_rtcompat::PreferredRuntime;

const APP_ID: &str = "org.hmmm.artig";

#[tokio::main]
async fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

async fn start_arti() {
    let config = TorClientConfig::default();

    let runtime = PreferredRuntime::current();

    let tor_client = TorClient::create_bootstrapped(config).await.unwrap();

    arti::socks::run_socks_proxy(runtime.unwrap().clone(), tor_client, 9051)
        .await
        .unwrap();
}

fn build_ui(app: &Application) {
    let label = gtk::Label::builder().label("Not Connected").build();

    let image = gtk::Image::builder()
        .file("/app/share/icons/hicolor/256x256/apps/toroff.png")
        .pixel_size(288)
        .build();

    let button = gtk::Button::builder()
        .label("Start Arti")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let content = Box::new(Orientation::Vertical, 0);

    content.append(&HeaderBar::new());
    content.append(&image);
    content.append(&label);
    content.append(&button);

    button.connect_clicked(move |button| {
        let main_context = gtk::glib::MainContext::default();

        main_context.spawn_local(clone!(@weak button => async move {
            start_arti().await;
        }));

        image.set_from_file(Some("/app/share/icons/hicolor/256x256/apps/toron.png"));

        label.set_label("Connected Tor at 127.0.0.1:9051");

        button.set_sensitive(false);
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Arti GUI")
        .content(&content)
        .icon_name("org.hmmm.artig")
        .build();

    window.present();
}
