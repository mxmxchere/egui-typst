use editor::TypstWorld;
use iced::Length::Fill;
use iced::Padding;
use iced::advanced::image::Handle;
use iced::widget::{column, image, row, scrollable, text_editor};
use iced::{Element, widget::text_editor::Content};
use typst::layout::PagedDocument;

static FONT0: &[u8] = include_bytes!("../noto/NotoSans-Regular.ttf");
static START_TEXT: &str = "#show heading: set align(center)
#show heading: set text(size: 30pt)
#set par(justify: true)
= Hello World!
#lorem(500)";

// State
#[derive(Clone)]
struct State {
    editor_content: text_editor::Content,
    images: Vec<Handle>,
    world: TypstWorld,
}

impl Default for State {
    fn default() -> Self {
        let font = typst::text::Font::new(typst::foundations::Bytes::new(FONT0), 0).unwrap();
        let fonts = vec![font];
        State {
            editor_content: Content::with_text(START_TEXT),
            world: editor::TypstWorld::new(fonts, START_TEXT.to_string(), "main".to_string()),
            images: Vec::default(),
        }
    }
}

// Messages
#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl State {
    // update logic
    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(a) => self.editor_content.perform(a),
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
                    .map(|p| Handle::from_bytes(typst_render::render(p, 4.0).encode_png().unwrap()))
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
            scrollable(
                column(
                    self.images
                        .iter()
                        .map(|i| { image(&*i).filter_method(image::FilterMethod::Linear).into() })
                )
                .spacing(10)
                .padding(Padding {
                    top: 10.0,
                    bottom: 10.0,
                    left: 10.0,
                    right: 20.0 // this is a hack for the scrollbar
                })
                .width(Fill)
            ) // i'm a bit proud of this beast
        ]
        .into()
    }
}
fn main() -> iced::Result {
    iced::run(State::update, State::view)
}
