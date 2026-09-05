# StreamBridge GitHub Issues CLI

## Pré-requisitos

- **Rust**: versão 1.98.0
- **Cargo**: versão 1.98.0

## Dependências
| Crate | Versão | Features |
|---|---|---|
| `chrono` | 0.4.45 | — |
| `dotenvy` | 0.15.7 | — |
| `parse_link_header` | 0.4.1 | — |
| `reqwest` | 0.13.4 | `json` |
| `serde` | 1.0.229 | `derive` |
| `serde_json` | 1.0.151 | — |
| `tokio` | 1.53.1 | `full` |

## Configuração do ambiente

Crie um **`.env`** na raiz do projeto com as variáveis abaixo:

| Variável | Obrigatório | Valor Padrão | Descrição |
|---|---|---|---|
| `GITHUB_API_URL` | Sim | — | URL base da API do GitHub |
| `PERSONAL_ACCESS_TOKEN` | Sim | — | Token de acesso pessoal, usado para autenticar as requisições à API |
| `DIRECTORY_TARGET` | Não | `issues` | Nome do diretório que ficarão os arquivos .json |

### 2. Obter o token de acesso do GitHub

Este projeto precisa de um **Personal Access Token (classic)** do GitHub para se autenticar à API do GitHub.

Crie um arquivo `.env` com as as variaveis de ambiente citadas na tabela acima.

1. Acesse GitHub → **Settings → Developer settings → Personal access tokens → Tokens (classic)**.
2. Clique em **Generate new token (classic)**.
3. Defina um nome, validade e os escopos necessários (ex.: `repo`, `read:packages`).
4. Copie o token gerado. PS: ele só é exibido uma vez.
5. Por fim, com o token copiado, cole na variável de ambiente `PERSONAL_ACCESS_TOKEN`. 

## Como rodar

Abra o terminal do seu sistema operacional e viaje até a pasta `cli` deste projeto. Em seguida, basta rodar os comandos abaixo.

```bash
cargo build
```

```bash
cargo run -- --user [usuario do repositorio] --repo [nome do repositorio] 
```

**Exemplo:**

```bash
cargo run -- --user octocat --repo Spoon-Knife --items 100
```

### Comandos

| Comando | Obrigatório | Valor Padrão | Descrição |
|---|---|---|---|
| `--user` | Sim | — | Usuário do repositório alvo |
| `--repo` | Sim | — | Nome do repositório alvo |
| `--items` | Não | 30 | Quantidade de itens que a API deve retornar |
| `--attemps` | Não | 3 | Quantidade máxima de tentativas em caso de falha na API |

## Nível alcançado

**Nível 3 - Extração incremental**   (completo)

### Implementado

- [x] **Nível 1 — Fundamento (obrigatório)**
    - [x] CLI que extrai issues de um repositório do GitHub
    - [x] Paginação completa (percorre todas as páginas)
    - [x] Autenticação via token pessoal (variável de ambiente)
    - [x] Grava saída em arquivo local (JSONL, CSV ou NDJSON)
    - [x] Seleciona subconjunto de campos relevantes
    - [x] Logs de progresso (páginas lidas, registros extraídos, conclusão)
    - [x] README com setup e instruções de execução

- [x] **Nível 2 — Robustez**
    - [x] Tratamento de rate limit (respeita headers + retry-after)
    - [x] Retry com backoff exponencial (teto configurável)
    - [x] Distinção entre erro transitório e permanente (429 vs 404/401)
    - [x] Timeouts de requisição configurados
    - [x] Tipos de erro com significado de domínio

- [x] **Nível 3 - Extração incremental**
    - [x] Watermark persistido em disco: a aplicação guarda o maior updated_at já extraído.
    - [x] Em execuções subsequentes, usa o parâmetro since para extrair apenas o que mudou desde a última execução.
    - [ ] Escrita e atualização de watermark de forma que uma falha parcial não deixe o estado corrompido.
    - [ ] 

## Decisões de Design

**Decisão:** Dividir o projeto em dois módulos: **application** e **domain**.

**Por quê:** Facilitar a implementação de novas features e futuras manutenções.

---

**Decisão:** Uso da biblioteca **parse_link_header**.

**Por quê:** Optei por utilizar essa biblioteca para pegar a próxima página através do header com mais facilidade, ao invés de perder muito tempo para recriar o mesmo algoritmo.

---

**Decisão:** Uso de comandos CLI para passar o usuario e repositório alvo, ao invés de variaveis de ambiente.

**Por quê:** Achei melhor utilizar comandos CLI por ser algo mais dinâmico e rápido do que ter que abrir o projeto e reconfigurar as variaveis de ambiente.

---

**Decisão:** Caminho do watermark marretado no código.

**Por quê:** No PDF do desafio está escrito para não marretar esse tipo de coisa no código, mas, já que o watermark é algo que ficaria escondido do usuário final, acredito que não seja necessária uma variável de ambiente configurável para isso.

## Uso de IA

Fiz uso de IA exclusivamente para tirar dúvidas conceituais sobre pontos que não entendi apenas com a leitura das documentações.

Os tópicos consultados foram:

- Ownership e Borrowing
- Closures
- Uso do `match`
- entender como lidar com rate limit



## O que faria se tivesse mais tempo

Estudaria mais sobre a linguagem e boas práticas. Alguns dos maiores impecilhos nesse projeto foi entender como funciona Ownership e Borrowing, o que acabou me fazendo gastar muito tempo em bugs simples por não entender o conceito. 