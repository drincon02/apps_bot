
pub mod filegui {

    use eframe::egui;
    #[derive(Default)]
    struct MyEguiApp {}
    impl MyEguiApp {
        fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
        }
    }

    impl eframe::App for MyEguiApp {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            let mut window = egui::Window::new("Boot File");
            let mut open_window = true;
            window = window.open(&mut open_window);
            window.show(ctx, |ui| {
                ui.heading("Hello how are you doing")
            });


        }
       
    }

    pub fn create_gui_app() {
        let native_options = eframe::NativeOptions::default();
        let _ = eframe::run_native("Boot File", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
    }

}