extern crate globwalk;
extern crate skim;
use serde::Deserialize;
use skim::prelude::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Pkg {
    name: String,
}

fn read_pkg_from_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u: Pkg = serde_json::from_reader(reader)?;

    Ok(u.name)
}

struct PkgItem {
    text: String,
    output: String,
}

impl SkimItem for PkgItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.text)
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.output)
    }
}

pub fn main() {
    let options = SkimOptionsBuilder::default().build().unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    let walker = globwalk::GlobWalkerBuilder::from_patterns(
        env::current_dir().unwrap(),
        &["**/package.json", "!node_modules", "!.*"],
    )
    .max_depth(4)
    .follow_links(true)
    .build()
    .unwrap()
    .filter_map(Result::ok);

    for img in walker {
        let name = read_pkg_from_file(img.path());
        if let Ok(n) = name {
            tx.send(Arc::new(PkgItem {
                text: n,
                output: format!("{}", img.path().display()),
            }))
            .unwrap()
        }
    }

    let selected_items = Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    for item in selected_items.iter() {
        let path = format!("{}", item.output());
        let dir = Path::new(&path).parent().unwrap();
        println!("{}", dir.display())
    }
}
