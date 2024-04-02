use std::{fs::File, io::Read, process};
use language::Language;
use serde::Deserialize;
use serde_json;

pub mod language;

#[derive(Deserialize)]
pub struct Languages {
    #[serde(rename = "linguagens")]
    pub languages: [Language; 25]
}

impl Languages {
    pub fn new() -> Languages {
        let json: String = match File::open("assets/linguagens.json") {
            Ok(mut file) => {
                let mut str_file = String::new();
                match file.read_to_string(&mut str_file) {
                    Ok(_) => str_file,
                    Err(_) => {
                        println!("Não foi possivel ler o arquivo.");
                        process::exit(2);
                    }
                }
            },
            Err(_) => {
                println!("Arquivo json de linguagens não encontrado.");
                process::exit(1);
            }
        };

        match serde_json::from_str(&json.as_str()) {
            Ok(value) => value, 
            Err(_) => {
                println!("Arquivo json invalido.");
                process::exit(1);
            }
        }
    }
}
