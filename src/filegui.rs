
pub mod filegui {

    use eframe::egui;
    use apps_boot::filereading::{BootFile, FileErr};
    #[derive(Default)]
    struct MyEguiApp {
        name: String,
        showerror: bool,
        showsuccess: bool,
        errormessage: String
    }
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
            let window = egui::CentralPanel::default();
            window.show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.heading("Boot a file");
                    ui.text_edit_singleline(&mut self.name);
                    if ui.button("Run File").clicked() {
                    let _run_file = match BootFile::Run(&self.name).call() {
                            Ok(_) => self.showsuccess = true,
                            Err(error) => {
                                self.showerror = true;
                                self.errormessage = match error {
                                    FileErr::IoError(err) => err.to_string(),
                                    FileErr::TomlError(err) => err.to_string()
                                };

                            }
                    };

                    };
                    
                    if self.showerror == true {
                        ui.label(self.errormessage.clone());
                    };
                    if self.showsuccess == true {
                        ui.label("File executed correctly");
                    };


                    })
                });


        }
       
    }

    pub fn create_gui_app() {
        let native_options = eframe::NativeOptions::default();
        let _ = eframe::run_native("Boot File", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
    }

}