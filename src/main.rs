use editor::TypstWorld;
use iced::Length::Fill;
use iced::Padding;
use iced::advanced::image::Handle;
use iced::widget::image::FilterMethod;
use iced::widget::{column, container, image, row, scrollable, slider, text, text_editor, toggler};
use iced::{Element, widget::text_editor::Content};
use typst::layout::PagedDocument;

static FONT0: &[u8] = include_bytes!("../noto/NotoSans-Regular.ttf");
static START_TEXT: &str = include_str!("../demo.typ");

// State
#[derive(Clone)]
struct State {
    editor_content: text_editor::Content,
    images: Vec<Handle>,
    world: TypstWorld,
    pixel_per_pt: f32,
    aliasing: bool,
}

impl Default for State {
    fn default() -> Self {
        let font = typst::text::Font::new(typst::foundations::Bytes::new(FONT0), 0).unwrap();
        let fonts = vec![font];
        State {
            editor_content: Content::with_text(START_TEXT),
            world: editor::TypstWorld::new(fonts, START_TEXT.to_string(), "main".to_string()),
            images: Vec::default(),
            pixel_per_pt: 1.0,
            aliasing: true,
        }
    }
}

// Messages
#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    PPP(f32),
    AliasingToggled(bool),
}

impl State {
    // update logic
    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(a) => self.editor_content.perform(a),
            Message::PPP(f) => self.pixel_per_pt = f,
            Message::AliasingToggled(b) => self.aliasing = b,
        }
        self.world
            .update_file("main".to_string(), self.editor_content.text());
        // TODO filter out cursor movements, compile only on changes
        let warned = typst::compile::<PagedDocument>(&self.world);
        match warned.output {
            Ok(doc) => {
                self.images = doc
                    .pages
                    .iter()
                    // TODO, maxmimum sexyness possible here would be to skip the png encoding step
                    // the first thing that happens next is that the png is decoded, would be cool
                    // to get the pixels directly to be painted by iced (or whatever is doing it)
                    // TODO, set an appropriate rendering accuracy here, depending on the window size
                    // maybe
                    .map(|p| {
                        Handle::from_bytes(
                            typst_render::render(p, self.pixel_per_pt)
                                .encode_png()
                                .unwrap(),
                        )
                    })
                    .collect();
            }
            Err(e) => println!("{:?}", e),
        };
    }

    //view logic
    // awesome function signature, view is read-only :)
    fn view(&self) -> Element<'_, Message> {
        row![
            column![
                text_editor(&self.editor_content)
                    .on_action(Message::Edit)
                    .height(Fill)
            ]
            .width(Fill),
            // display all byte-images in self.images with a little gap
            column![
                row![
                    text(format!("Pixel per pt.: {}", self.pixel_per_pt)),
                    slider(0.1..=5.0, self.pixel_per_pt, Message::PPP).step(0.01),
                    toggler(self.aliasing)
                        .label("Aliasing")
                        .on_toggle(Message::AliasingToggled)
                ]
                .spacing(10)
            ]
            .spacing(10)
            .padding(Padding {
                top: 10.0,
                bottom: 10.0,
                left: 10.0,
                right: 10.0
            })
            .width(Fill)
            .push(
                container(
                    scrollable(
                        column(self.images.iter().map(|i| {
                            image(&*i)
                                .filter_method(match self.aliasing {
                                    true => FilterMethod::Linear,
                                    false => FilterMethod::Nearest,
                                })
                                .into()
                        }))
                        .spacing(10)
                    ) // i'm a bit proud of this beast
                )
                .center_x(Fill)
            )
        ]
        .into()
    }
}
fn main() -> iced::Result {
    iced::run(State::update, State::view)
}
