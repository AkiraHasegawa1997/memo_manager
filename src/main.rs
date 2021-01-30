use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use std::vec::Vec;

#[derive(Serialize, Deserialize)]
struct Doc {
    name: String,
    memo: String,
}

trait Json: Sized {
    fn read_json<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>>;
    fn write_json<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>>;
    fn add_memo(&mut self, a_name: String, a_memo: String);
    fn find_doc(&self, a_name: String) -> Option<&Doc>;
}
impl Json for Vec<Doc> {
    fn read_json<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        // reference: https://docs.serde.rs/serde_json/de/fn.from_reader.html (2021-01-30)
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let doc = serde_json::from_reader(reader)?;
        Ok(doc)
    }

    fn write_json<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        use std::io::Write;
        let mut file = std::fs::File::create(path)?;
        file.write_all(serde_json::to_string_pretty(&self)?.as_bytes())?;
        Ok(())
    }

    fn add_memo(&mut self, a_name: String, a_memo: String) {
        if self.iter().all(|x| !x.name.eq(&a_name)) {
            self.push(Doc {
                name: a_name,
                memo: a_memo,
            });
        } else {
            eprintln!("Duplicated name is found: {}", a_name);
        }
    }

    fn find_doc(&self, a_name: String) -> Option<&Doc> {
        self.iter().find(|&x| x.name.eq(&a_name))
    }
}

fn main() {
    use std::env;
    let show_usage = || {
        eprintln!(
        "Usage \nadd: {0} <json_path> <pdf_path> <memo_string>\nread: {0} <json_path> <pdf_path>",
        std::env::args().nth(0).unwrap()
        );
        std::process::exit(1);
    };
    let json_path = env::args().nth(1).unwrap_or_else(show_usage);
    let pdf_path = env::args().nth(2).unwrap_or_else(show_usage);
    let pdf_file_name = Path::new(&pdf_path)
        .file_stem()
        .expect("invalid file name")
        .to_str()
        .expect("invalid file name")
        .to_string();

    let mut json = match Json::read_json(&json_path) {
        Ok(json) => json,
        Err(_) => Vec::new(),
    };

    if let Some(new_memo) = env::args().nth(3) {
        json.add_memo(pdf_file_name, new_memo);
        json.write_json(&json_path).expect("failed to write json");
    } else if let Some(doc) = json.find_doc(pdf_file_name) {
        println!("{}", doc.memo)
    } else {
        show_usage();
    }
}
