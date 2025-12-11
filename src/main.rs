use eframe::egui;
use egui::{Layout, Vec2, Widget, load::Bytes};
use typst_as_lib::TypstEngine;
use typst_library::layout::PagedDocument;
static FONT0: &[u8] = include_bytes!("../noto/NotoSans-Regular.ttf");
static URI: &str = "bytes://code.svg";
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    code: String,
    svg: Bytes,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut s = Self {
            svg: egui::load::Bytes::from(Vec::new()),
            code: "= Hello ".into(),
        };
        s.render_svg();
        s
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                if ui
                    .add_sized(
                        Vec2::new(ui.available_width() / 2.0, ui.available_height()),
                        egui::TextEdit::multiline(&mut self.code)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .lock_focus(true),
                    )
                    .changed()
                {
                    egui_extras::install_image_loaders(ctx);
                    self.render_svg();
                    ctx.forget_all_images();
                }
                let i = egui::Image::from_bytes(std::borrow::Cow::Borrowed(URI), self.svg.clone());
                ui.add_sized(ui.available_size(), i);
                ui.allocate_space(ui.available_size());
            });
        });
    }
}

impl MyApp {
    fn render_svg(&mut self) {
        //let engine = TypstEngine::builder()
        //    .main_file(self.code.clone())
        //    .fonts([FONT0])
        //    .build();
        //if let Ok(d) = engine.compile::<PagedDocument>().output {
        println!("{}", self.code);
        let font = typst::text::Font::new(typst::foundations::Bytes::new(FONT0), 0).unwrap();
        let fonts = [font];
        let world = editor::TypstWorld::new(&fonts, self.code.clone(), "main".to_string());
        if let Ok(d) = world.compile().output {
            let bytes: Vec<u8> = typst_svg::svg_merged(&d, typst_library::layout::Abs::zero())
                .clone()
                .as_bytes()
                .into();
            self.svg = egui::load::Bytes::from(bytes);
        };
    }
}
