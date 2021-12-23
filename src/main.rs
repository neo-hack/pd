extern crate skim;
use skim::prelude::*;
use std::error::Error;
use serde::{Deserialize};
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Pkg {
    name: String
}

fn read_pkg_from_file<P: AsRef<Path>>(path: P) -> Result<Cow<'static, str>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u: Pkg = serde_json::from_reader(reader)?;

    Ok(Cow::Owned(u.name))
}

struct PkgItem {
    text: String,
}

impl SkimItem for PkgItem {
    fn text(&self) -> Cow<str> {
        let name = read_pkg_from_file(&self.text).unwrap();
        name
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.text)
    }
    
}

pub fn main() {
    let options = SkimOptionsBuilder::default()
        .query(Some("package.json"))
        .build()
        .unwrap();

    let (tx, _): (SkimItemSender, SkimItemReceiver) = unbounded();

    tx.send(Arc::new(PkgItem { text: "a".to_string() })).unwrap();

    let selected_items = Skim::run_with(&options, None)
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}{}", item.output(), "\n");
    }
}

// extern crate skim;
// use skim::prelude::*;

// struct MyItem {
//     inner: String,
// }

// impl SkimItem for MyItem {
//     fn text(&self) -> Cow<str> {
//         Cow::Borrowed(&self.inner)
//     }

//     fn preview(&self, _context: PreviewContext) -> ItemPreview {
//         if self.inner.starts_with("color") {
//             ItemPreview::AnsiText(format!("\x1b[31mhello:\x1b[m\n{}", self.inner))
//         } else {
//             ItemPreview::Text(format!("hello:\n{}", self.inner))
//         }
//     }
// }

// pub fn main() {
//     let options = SkimOptionsBuilder::default()
        
//         .height(Some("50%"))
//         .multi(true)
//         .preview(Some("")) // preview should be specified to enable preview window
//         .build()
//         .unwrap();

//     let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
//     let _ = tx_item.send(Arc::new(MyItem {
//         inner: "color aaaa".to_string(),
//     }));
//     let _ = tx_item.send(Arc::new(MyItem {
//         inner: "bbbb".to_string(),
//     }));
//     let _ = tx_item.send(Arc::new(MyItem {
//         inner: "ccc".to_string(),
//     }));
//     drop(tx_item); // so that skim could know when to stop waiting for more items.

//     let selected_items = Skim::run_with(&options, Some(rx_item))
//         .map(|out| out.selected_items)
//         .unwrap_or_else(|| Vec::new());

//     for item in selected_items.iter() {
//         print!("{}{}", item.output(), "\n");
//     }
// }
