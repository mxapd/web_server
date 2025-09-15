use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct Html {
    content: String,
}

impl Html {
    pub fn from_file(filepath: String) -> Result<Html, Box<dyn Error>> {
        let file = File::open(filepath)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let html = Html { content: contents };

        Ok(html)
    }

    pub fn from_string(html_string: String) -> Html {
        Html {
            content: html_string,
        }
    }

    pub fn new() -> Html {
        Html {
            content: String::new(),
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.content.into_bytes()
    }
}
