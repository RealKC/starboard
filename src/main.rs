use app_background::AppBackground;
use clock::ClockGadget;
use utils::{Drawable, Gadget, NetworkRuntime};
use weather::WeatherGadget;

mod app_background;

pub struct StarboardApp {
    background: AppBackground,
    clock_gadget: ClockGadget,
    weather_gadget: WeatherGadget,
}

impl StarboardApp {
    fn new(network_runtime: NetworkRuntime, egui_ctx: egui::Context) -> Self {
        Self {
            background: Default::default(),
            clock_gadget: ClockGadget::new(&network_runtime, &egui_ctx),
            weather_gadget: WeatherGadget::new(&network_runtime, &egui_ctx),
        }
    }
}

impl eframe::App for StarboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Area::new("background")
            .interactable(false)
            .fixed_pos(ctx.available_rect().center() - self.background.size() / 2.0)
            .order(egui::Order::Background)
            .show(ctx, |ui| self.background.draw(ui));

        self.clock_gadget.render(ctx);
        self.weather_gadget.render(ctx);
    }
}

fn setup_network_runtime() -> NetworkRuntime {
    let (tx, rx) = tokio::sync::oneshot::channel();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        tx.send(runtime.handle().clone());
        runtime.block_on(async { tokio::time::sleep(std::time::Duration::from_secs(30)).await });
    });

    rx.blocking_recv().unwrap()
}

fn main() {
    let options = eframe::NativeOptions::default();
    let network_runtime = setup_network_runtime();

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::new(StarboardApp::new(network_runtime, cc.egui_ctx.clone()))),
    );
}
