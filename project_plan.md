# Roadmap — “Agentrix” (Rust ReAct Agent + MCP + CLI)

> Objetivo: construir um agente estilo LangGraph (ReAct) com streaming, LLM factory, MCP tools (server+client) e uma CLI simples. Vamos progredir em *blocos semânticos*, cada um com **entregáveis**, **testes manuais** e **commits** sugeridos. **Sem código agora** — só o trilho de execução.

---

## Fase 0 — Boot do workspace

* [ ] **Criar workspace Cargo** (`agentrix/`) com crates: `core`, `llm`, `mcp-server`, `cli`.
* [ ] **Configuração base**: `tokio`, `reqwest` (com `json` + `rustls-tls`), `serde`, `serde_json`, `anyhow`, `thiserror`, `tracing`, `tracing-subscriber`.
* [ ] **Lint/format**: `rustfmt`, `clippy`.
* [ ] **README** com visão geral + como rodar.
* 🧪 **Teste manual**: `cargo build -p cli` compila; `cargo test` vazio passando.
* 💾 **Commit**: `chore: init cargo workspace and deps`

---

## Fase 1 — Núcleo de tipos e eventos (core)

* [ ] **Tipos base**: `Message { role, content }`, `Role`, `LlmConfig`.
* [ ] **Eventos de streaming**: `Event::{StateEnter, StateExit, ModelDelta, ToolCall, ToolResult, FinalText, Error}`.
* [ ] **Contexto do grafo**: `GraphContext { llm, history, event_tx }`.
* [ ] **Interface de nó**: `AgentNode` (trait `async`).
* [ ] **Executor linear**: `Graph::new(...).run(ctx)`.
* 🧪 **Teste manual**: criar `Graph` com nó “fake” que emite `StateEnter/Exit`; ver eventos num `mpsc` local.
* 💾 **Commit**: `feat(core): base types and event bus`

---

## Fase 2 — CLI esqueleto (sem LLM)

* [ ] **CLI REPL**: ler linhas do `stdin`, manter `history` em memória.
* [ ] **Impressor de eventos**: task separada consumindo `mpsc::UnboundedReceiver<Event>`, imprimindo estado/erros.
* [ ] **Rodada vazia**: ao digitar algo, cria `Graph` com 1 nó “echo” que gera um `FinalText` com o input.
* 🧪 **Teste manual**: `cargo run -p cli`, digitar “olá” e ver `FinalText: olá`.
* 💾 **Commit**: `feat(cli): repl + event printer`

---

## Fase 3 — LLM trait + Factory (sem provider ainda)

* [ ] **Trait `Llm`**: método `stream_chat(system, messages, tools) -> Stream<Event>`.
* [ ] **`ToolSpec`**: descrição opcional para function-calling futuro.
* [ ] **Factory `LlmFactory::make(LlmConfig)`**: valida provider/modelo.
* [ ] **Helper** pra clonar como `Box<dyn Llm>` (ex.: `to_owned_box()`).
* 🧪 **Teste manual**: mock de LLM que streama “hello ”, “world” em `ModelDelta`.
* 💾 **Commit**: `feat(llm): trait and factory with mock`

---

## Fase 4 — Provider OpenAI (streaming)

* [ ] **OpenAI impl**: chamada `chat.completions` com `stream=true`, parsing SSE → `Event::ModelDelta` e `Event::ToolCall` (se houver).
* [ ] **Config** por env: `OPENAI_API_KEY`; `LlmConfig.model/provider`.
* [ ] **Erros**: mapear 4xx/5xx e rede (propagar em `Event::Error` ao stream).
* 🧪 **Teste manual**: CLI → pergunta simples; ver texto chegando em *chunks* (`ModelDelta`) e um `FinalText`.
* 💾 **Commit**: `feat(llm-openai): streaming SSE + error mapping`

---

## Fase 5 — ReAct mínimo (Plan → Decide)

* [ ] **`PlanNode`**: injeta instrução `system` básica no `history`.
* [ ] **`DecideNode`**: chama `llm.stream_chat(...)`, reenviando `ModelDelta` e consolidando `FinalText`.
* [ ] **Integração CLI**: rodar grafo `Plan -> Decide` a cada input do usuário.
* 🧪 **Teste manual**: perguntar “o que é Rust?”; ver stream e resposta final; `history` crescendo.
* 💾 **Commit**: `feat(core): minimal ReAct plan/decide flow`

---

## Fase 6 — MCP Server (tools de exemplo)

* [ ] **Crate `mcp-server`** com transporte `stdio`.
* [ ] **Tools**: `get_time()`, `echo(text)`. (Simples, só para provar integração).
* [ ] **Lifecycle**: sobe, serve e encerra limpeza.
* 🧪 **Teste manual**: rodar `mcp-server` sozinho; verificar que inicia sem erro.
* 💾 **Commit**: `feat(mcp-server): stdio server with basic tools`

---

## Fase 7 — MCP Client (invocação de tools)

* [ ] **Wrapper `McpClient`**: conectar via `spawn` do processo `mcp-server` (stdio).
* [ ] **API**: `call(name, args) -> serde_json::Value`.
* [ ] **Resiliência**: restart opcional se o child cair (deixar anotado para fase posterior).
* 🧪 **Teste manual**: binário temporário que chama `McpClient::call("get_time")` e imprime JSON.
* 💾 **Commit**: `feat(core): mcp client wrapper (spawn stdio child)`

---

## Fase 8 — Act/Observe Nodes + Tool streaming

* [ ] **`DecideNode`** detecta `Event::ToolCall` → encaminha para **`ActNode`**.
* [ ] **`ActNode`**: usa `McpClient` para executar; emite `Event::ToolResult`.
* [ ] **`ObserveNode`**: adiciona `tool_result` ao `history` e retorna para decisão (loop 1 passo).
* [ ] **Ciclo**: Plan → Decide (toolcall?) → Act → Observe → Decide (resposta final).
* 🧪 **Teste manual**: prompt que induza ferramenta (`get_time`); ver `toolcall` → `tool_result` → resposta incorporando o resultado.
* 💾 **Commit**: `feat(core): act/observe nodes with MCP tool execution`

---

## Fase 9 — UX de CLI (feedback visual simples)

* [ ] **Status**: `[state]` com `StateEnter/Exit`.
* [ ] **Tool feedback**: `[toolcall] name args` e `[tool_result] ok=…`.
* [ ] **Append de texto**: imprimir deltas sem quebrar linha; consolidar ao final.
* [ ] **Ctrl+C**: encerramento limpo; dreno de eventos.
* 🧪 **Teste manual**: conversar, ver fluxo limpo, sem “pular” cursor.
* 💾 **Commit**: `feat(cli): polished event printing and graceful shutdown`

---

## Fase 10 — Configuração e DX

* [ ] **Flags/ENV**: `--provider`, `--model`, `--reasoning`, `--mcp-bin` (caminho do server), `--verbose`.
* [ ] **`tracing`**: `RUST_LOG` controlando logs (debug para rede/MCP).
* [ ] **Mensagens de erro amigáveis** (sem vazar stack crua no modo padrão).
* 🧪 **Teste manual**: trocar modelos e nível de log via flags/env.
* 💾 **Commit**: `feat(cli): flags for provider/model and logging`

---

## Fase 11 — Persistência de sessão

* [ ] **Salvar/Carregar**: `--session file.json` (history + config).
* [ ] **Append**: ao enviar nova mensagem, atualiza arquivo.
* [ ] **Compatibilidade**: versão do schema no JSON.
* 🧪 **Teste manual**: iniciar conversa, sair, retornar com o mesmo `--session`.
* 💾 **Commit**: `feat(core/cli): session persistence (save/load)`

---

## Fase 12 — Testes automatizados

* [ ] **Tests unit**: serialização de `Event`, `Message`, `LlmConfig`.
* [ ] **Test double** de LLM (mock stream) para testar ReAct sem rede.
* [ ] **Test e2e** (feature flag `offline`): grafo com fake LLM + fake MCP.
* 🧪 **CI local**: `cargo test --all`.
* 💾 **Commit**: `test: unit and offline e2e for react flow`

---

## Fase 13 — Robustez de rede e retries

* [ ] **OpenAI**: retry com backoff para `429`/`5xx`; respeito a `Retry-After`.
* [ ] **Timeouts** configuráveis.
* [ ] **MCP**: fallback se o child morrer (restart limitado).
* 🧪 **Teste manual**: simular falhas (desligar rede/derrubar server) e observar comportamento.
* 💾 **Commit**: `feat(llm/mcp): retries, timeouts and graceful failures`

---

## Fase 14 — Tools mais úteis

* [ ] **`http_get(url)`** (com whitelist simples).
* [ ] **`fs_read(path)`** (sandbox/whitelist).
* [ ] **`python_eval`** (se quiser; isolado, com limite).
* 🧪 **Teste manual**: perguntar algo que exige web/file; observar toolcall/result.
* 💾 **Commit**: `feat(mcp-server): http_get and fs_read tools`

---

## Fase 15 — Observabilidade & métricas

* [ ] **Traços**: spans por request/grafo/nó.
* [ ] **Métricas simples**: contadores de toolcalls, latência de LLM, tokens (se disponível), erros.
* [ ] **Export**: log estruturado JSON opcional.
* 🧪 **Teste manual**: ativar modo métricas; ver números após conversa.
* 💾 **Commit**: `feat(obs): tracing spans and basic counters`

---

## Fase 16 — Empacotamento

* [ ] **Bin único** para CLI; `mcp-server` como segundo bin.
* [ ] **Make/Justfile** para comandos padrão (build, test, run).
* [ ] **Dockerfiles** (opcional).
* 🧪 **Teste manual**: rodar em máquina “limpa” com apenas `OPENAI_API_KEY`.
* 💾 **Commit**: `chore: release packaging and run scripts`

---

## Fase 17 — Extras (opcionais)

* [ ] **Prompt templates**/profiles (instruções por tarefa).
* [ ] **Corte de contexto** (janela deslizante).
* [ ] **Plugins de provider**: Azure, Anthropic (adapter).
* [ ] **Front-end**: expor WS/HTTP e streamar eventos (para UI).
* 💾 **Commit**: `feat: provider plugins / http bridge`

---

### Critérios de “pronto para demo”

* [ ] Perguntas via CLI com **streaming** fluido.
* [ ] Toolcall → MCP tool → ToolResult → resposta final incorporando resultado.
* [ ] Config por flags/env, logs úteis, session persistindo.
* [ ] Testes unitários básicos passando.

---

Quando disser “ok”, começamos na **Fase 0** e sigo te entregando **o menor corte de código testável** por etapa, com comandos e exemplos de uso.
