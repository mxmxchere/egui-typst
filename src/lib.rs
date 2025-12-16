use chrono::{DateTime, Datelike, Duration, Utc};
use comemo::track;
use std::collections::HashMap;
use typst::foundations::Datetime;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt};
#[derive(Clone)]
pub struct VirtualFS(HashMap<FileId, Source>);

impl VirtualFS {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn insert_file(&mut self, id: FileId, contents: String) {
        let s = Source::new(id, contents);
        self.0.insert(id, s);
    }
    pub fn get_source(&self, id: FileId) -> typst::diag::FileResult<Source> {
        if let Some(f) = self.0.get(&id) {
            // this clone feels inevitable
            typst::diag::FileResult::Ok(f.clone())
        } else {
            // This should not be accessed denied, TODO
            typst::diag::FileResult::Err(typst::diag::FileError::AccessDenied)
        }
    }
    pub fn get_bytes(&self, id: FileId) -> typst::diag::FileResult<typst::foundations::Bytes> {
        if let Some(s) = self.0.get(&id) {
            // this is expensive, but maybe not so bad...
            let s_c: Vec<u8> = s.text().into();
            Ok(typst::foundations::Bytes::new(s_c))
        } else {
            typst::diag::FileResult::Err(typst::diag::FileError::AccessDenied)
        }
    }
}
#[derive(Clone)]
pub struct TypstWorld {
    //<'a> {
    library: LazyHash<Library>, //main_source_id: FileId,
    now: DateTime<Utc>,
    fs: VirtualFS,
    //book: &'a LazyHash<FontBook>,
    //    file_resolvers: &'a [Box<dyn FileResolver + Send + Sync + 'static>],
    fonts: Vec<Font>,
    initial_id: FileId,
    book: LazyHash<FontBook>,
}

impl TypstWorld {
    pub fn new(fonts: Vec<Font>, initial: String, initial_id: String) -> Self {
        // None is probably not the play here, TODO
        let id = FileId::new(None, VirtualPath::new(initial_id));
        let mut fs = VirtualFS::new();
        fs.insert_file(id, initial);
        let mut book = FontBook::new();
        for f in fonts.clone() {
            book.push(f.info().clone());
        }
        let book = LazyHash::new(book);
        Self {
            library: LazyHash::new(typst::Library::default()),
            now: Utc::now(),
            fs,
            fonts,
            // Probably we should be able to change it once the thing is throughclickable
            initial_id: id,
            book,
        }
    }
}
impl typst::World for TypstWorld {
    fn library(&self) -> &typst::utils::LazyHash<typst::Library> {
        &self.library
    }
    fn book(&self) -> &typst::utils::LazyHash<typst::text::FontBook> {
        &self.book
    }
    fn main(&self) -> typst::syntax::FileId {
        self.initial_id
    }
    fn source(&self, id: typst::syntax::FileId) -> typst::diag::FileResult<typst::syntax::Source> {
        self.fs.get_source(id)
    }
    fn file(
        &self,
        id: typst::syntax::FileId,
    ) -> typst::diag::FileResult<typst::foundations::Bytes> {
        self.fs.get_bytes(id)
    }
    fn today(&self, offset: Option<i64>) -> Option<typst::foundations::Datetime> {
        let mut now = self.now;
        if let Some(offset) = offset {
            now += Duration::hours(offset);
        }
        let date = now.date_naive();
        let year = date.year();
        let month = (date.month0() + 1) as u8;
        let day = (date.day0() + 1) as u8;
        Datetime::from_ymd(year, month, day)
    }
    fn font(&self, index: usize) -> Option<typst::text::Font> {
        // technically i don't like this Some wrapping when we could use get as one should,
        // but typst_as_lib did it this way so let's accept it
        Some(self.fonts[index].clone())
    }
}

#[track]
impl TypstWorld {
    pub fn update_file(&mut self, path: String, content: String) {
        let id = FileId::new(None, VirtualPath::new(path));
        self.fs.insert_file(id, content);
    }
}

use operational_transform::*;
pub fn stringify(o: &Operation) -> String {
    match o.clone() {
        Operation::Delete(i) => format!("D;{}\n", i),
        Operation::Retain(i) => format!("R;{}\n", i),
        Operation::Insert(s) => format!("I;{}\n", s),
    }
}

pub fn destringify(s: String) -> Option<Operation> {
    let str = s.as_bytes();
    if str.len() < 4 {
        return None;
    }
    match str[0] {
        68 => Some(Operation::Delete(
            s.strip_suffix("\n")
                .unwrap()
                .strip_prefix("D;")
                .unwrap()
                .parse::<u64>()
                .unwrap(),
        )),
        82 => Some(Operation::Retain(
            s.trim().strip_prefix("R;").unwrap().parse::<u64>().unwrap(),
        )),
        73 => Some(Operation::Insert(
            s.strip_prefix("I;").unwrap().trim().to_string(),
        )),
        _ => None,
    }
}
