use std::env;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub items_per_page: u8,
    pub repository_user: String,
    pub repository_name: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            items_per_page: 100,
            repository_user: String::from(""),
            repository_name: String::from(""),
        }
    }
}

pub fn parse_args() -> Config {
    let mut config: Config = Config::default();

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!(
            "Uso: {} --url <url_do_repositorio> [--items <numero>]",
            args[0]
        );
        println!();
        println!("Parâmetros:");
        println!("  --user    (obrigatório) Usuário do repositório");
        println!("  --repo    (obrigatório) Nome do repositório");
        println!("  --items   (opcional) Número de itens por página");
        process::exit(1);
    }

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--items" => {
                i += 1;

                if let Some(arg) = args.get(i) {
                    if let Ok(parsed) = arg.parse::<u8>() {
                        config.items_per_page = parsed;

                        i += 1;
                        continue;
                    }
                }
                println!("Erro: --items requer um número");
                process::exit(1);
            }
            "--user" => {
                i += 1;

                if let Some(arg) = args.get(i) {
                    let Ok(parsed) = arg.parse::<String>();
                    config.repository_user = parsed;

                    i += 1;
                    continue;
                }

                println!("Erro: --user requer o usuário do dono do repositório.");
                process::exit(1);
            }
            "--repo" => {
                i += 1;

                if let Some(arg) = args.get(i) {
                    let Ok(parsed) = arg.parse::<String>();
                    config.repository_name = parsed;

                    i += 1;
                    continue;
                }

                println!("Erro: --repo requer o nome do repositório.");
                process::exit(1);
            }
            _ => {
                println!("Erro: argumento desconhecido '{}'", args[i]);
                process::exit(1);
            }
        }
    }

    config
}
