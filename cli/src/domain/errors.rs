use core::num;
use std::io;

use reqwest::header;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("falha ao configurar o cliente HTTP: {0}")]
    ClientBuildFailed(#[from] reqwest::Error),

    #[error("Rate Limit excedido. Tente novamente em {0} minutos.")]
    RateLimitError(i64),

    #[error("Não foi possível converter o timestamp: {0}")]
    TimestampParseError(num::ParseIntError),

    #[error("Não foi possível extrair o timestamp: {0}")]
    TimestampExtractError(header::ToStrError),

    #[error("Erro ao montar o json")]
    SerializingError(),

    #[error("Erro ao separar as issues: {0}")]
    FetchIssuesFailed(serde_json::Error),

    #[error("Tempo limite excedido: {0}")]
    TimeoutError(String),

    #[error("Ocorreu um problema durante a requisição, tente novamente.")]
    RequestError,

    #[error("{0} deve estar definido.")]
    EnvironmentVariableMissingError(String),

    #[error("Ocorreu um erro ao salvar as issues no arquivo json: {0}")]
    WriteFileError(#[from] io::Error),
}
