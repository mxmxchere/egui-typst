use editor::TypstWorld;
use eframe::egui;
use egui::{Color32, Layout, Vec2, load::Bytes};
use typst::layout::{Page, PagedDocument};
static FONT0: &[u8] = include_bytes!("../noto/NotoSans-Regular.ttf");
static URI: &str = "bytes://code.svg";
static URI_2: &str = "bytes://code2.svg";
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::new(MyApp::new()))),
    )
}

struct MyApp {
    code: String,
    svg: [Option<Bytes>; 2],
    world: TypstWorld,
    y_offset: f32,
    current_page: usize,
}
impl MyApp {
    fn new() -> Self {
        let font = typst::text::Font::new(typst::foundations::Bytes::new(FONT0), 0).unwrap();
        let fonts = vec![font];
        let code = "= Hello";
        let world = editor::TypstWorld::new(fonts, code.into(), "main".to_string());
        let mut s = Self {
            svg: [None, None],
            code: code.into(),
            world: world,
            y_offset: 0.0,
            current_page: 0,
        };
        s.render_svg();
        s
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui_extras::install_image_loaders(ctx);
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
                    self.render_svg();
                    ctx.forget_all_images();
                }
                // This looks really awful stretched so something needs fix TODO
                let i = egui::Image::from_bytes(URI, self.svg[0].as_ref().unwrap().clone());
                let t = i.load_for_size(ctx, ui.available_size()).unwrap();
                let uv = egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0));
                let dest_rect = egui::Rect::from_min_size(
                    ui.available_rect_before_wrap().min - Vec2::new(0.0, self.y_offset),
                    ui.available_size(),
                );
                let height = ui.available_height();
                let rest_rect = egui::Rect::from_min_size(
                    ui.available_rect_before_wrap().min + Vec2::new(0.0, dest_rect.max.y),
                    ui.available_size(),
                );

                let r = ui.allocate_rect(dest_rect, egui::Sense::hover());

                let rr = ui.allocate_rect(rest_rect, egui::Sense::hover());
                let mut forget = false;
                if r.hovered() || rr.hovered() {
                    ctx.input(|i| {
                        let delta = i.raw_scroll_delta.y;
                        // stop scrolling once we reach end
                        if self.svg[0].is_some() {
                            if self.y_offset - delta > height {
                                if self.svg[1].is_some() {
                                    self.current_page += 1;
                                    self.y_offset = 0.0;
                                    self.render_svg();
                                    // This causes a deadlock
                                    // probably egui bug?
                                    //ctx.forget_all_images()
                                    forget = true;
                                }
                            } else if self.y_offset - delta < 0.0 {
                                if self.current_page != 0 {
                                    self.current_page -= 1;
                                    self.y_offset = height + (self.y_offset - delta);
                                    self.render_svg();
                                    forget = true;
                                }
                            } else {
                                self.y_offset -= delta;
                            }
                        }
                    });
                }
                // We should be more clever about exactly what we forget TODO
                // point is we should be able to shift them around cleverly
                // so that only whatever is out of sight is loading (and blinking which is harsh)
                if forget {
                    ctx.forget_all_images();
                }
                ui.painter()
                    .image(t.texture_id().unwrap(), dest_rect, uv, Color32::WHITE);
                if let Some(i) = self.svg[1].clone() {
                    if let Ok(t) =
                        egui::Image::from_bytes(URI_2, i).load_for_size(ctx, rest_rect.size())
                    {
                        ui.painter()
                            .image(t.texture_id().unwrap(), rest_rect, uv, Color32::WHITE);
                    }
                }
            });
        });
    }
}

impl MyApp {
    fn render_svg(&mut self) {
        self.world
            .update_file("main".to_string(), self.code.clone());
        let now = std::time::Instant::now();
        let warned = typst::compile::<PagedDocument>(&self.world);
        match warned.output {
            Ok(d) => {
                let duration = now.elapsed().as_millis();
                println!("Took {}ms to compile", duration);
                let current_pages: [Option<&Page>; 2] = [
                    d.pages.get(self.current_page),
                    d.pages.get(self.current_page + 1),
                ];
                if let Some(p) = current_pages[0] {
                    let bytes: Vec<u8> = typst_svg::svg(p).clone().as_bytes().into();
                    self.svg[0] = Some(egui::load::Bytes::from(bytes));
                } else {
                    self.svg[0] = None;
                }
                if let Some(p) = current_pages[1] {
                    let bytes: Vec<u8> = typst_svg::svg(p).clone().as_bytes().into();
                    self.svg[1] = Some(egui::load::Bytes::from(bytes));
                } else {
                    self.svg[1] = None;
                }
            }
            Err(e) => {
                eprintln!("{:?}", e)
            }
        };
    }
}
