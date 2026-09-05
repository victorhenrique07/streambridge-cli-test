use std::env;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub items_per_page: u8,
    pub repository_user: String,
    pub repository_name: String,
    pub retry_attempts: u8,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            items_per_page: 30,
            repository_user: String::new(),
            repository_name: String::new(),
            retry_attempts: 3,
        }
    }
}

pub fn parse_args() -> Config {
    let mut config: Config = Config::default();

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Uso: {} [--items <numero>]", args[0]);
        println!();
        println!("Parâmetros:");
        println!("  --items      (opcional) Número de itens por página");
        println!("  --user       (obrigatório) Usuário dono do repositório.");
        println!("  --repo       (obrigatório) Nome do repositório.");
        println!("  --update     (opcional) Atualiza o json de issues.");
        println!(
            "  --attempts   (opcional) Quantidade máxima de tentativas em caso de falha na API"
        );
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
                    config.repository_user = arg.to_string();

                    i += 1;
                    continue;
                }

                println!("Argumento faltando para --user.");
                println!("Ex: --user [username]");
                process::exit(1);
            }
            "--repo" => {
                i += 1;

                if let Some(arg) = args.get(i) {
                    config.repository_name = arg.to_string();

                    i += 1;
                    continue;
                }

                println!("Argumento faltando para --repo.");
                println!("Ex: --repo [repositório]");
                process::exit(1);
            }
            "--attempts" => {
                i += 1;

                if let Some(arg) = args.get(i) {
                    if let Ok(parsed) = arg.parse::<u8>() {
                        config.retry_attempts = parsed;

                        i += 1;
                        continue;
                    }

                    println!("Erro: --attemps requer um número");
                    process::exit(1);
                }

                println!("Argumento faltando para --attempts.");
                println!("Ex: --attempts [tentativas]");
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
