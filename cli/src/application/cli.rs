use std::env;
use std::process;

pub struct Config {
    pub items_per_page: u8
}

pub fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut config = Config {
        items_per_page: 200,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--items" => {
                i += 1;
                
                if let Some(v) = args.get(i) {
                    if let Ok(parsed) = v.parse::<u8>() {
                        config.items_per_page = parsed;

                        continue;
                    }

                    println!("Erro: --items requer um número");
                    process::exit(1);
                }
            }
            _ => {
                println!("Erro: argumento desconhecido '{}'", args[i]);
                process::exit(1);
            }
        }
    }

    config
}