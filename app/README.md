# Estuda UVA

Aplicativo desktop local para organizar materiais acadêmicos do Canvas UVA, converter PDFs em Markdown por página e consultar prazos sem alterar atividades ou notas.

## Desenvolvimento

Requisitos: Rust estável compatível com a versão indicada no `Cargo.toml`, Node.js/npm apenas para o frontend e Chromium ou Chrome instalado.

```sh
npm install
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
npm run tauri dev
```

O login do Canvas acontece manualmente numa janela Chromium visível e num perfil separado do navegador pessoal. O app não solicita nem guarda senha. A automação navega e consulta conteúdo em modo de leitura.

## Agenda

A agenda persiste eventos no SQLite local em `app_data_dir`, separados por semestre e com link de origem. A sincronização consulta atribuições publicadas e prazos aplicáveis ao estudante nas matérias atuais exibidas pelo Canvas. Ela não abre páginas de entrega nem modifica atividades.

Em 9 de outubro de 2026, o próximo semestre registrado no planejamento é 2027.1. As fontes públicas consultadas ainda não apresentaram seu calendário oficial. Por isso, a tela mostra o período sem datas e oferece o link oficial da UVA; nenhum prazo é estimado a partir de semestres anteriores.

## Arquivos e privacidade

O app escreve textos e aulas em Markdown na pasta local do acervo configurada pelo estudante. PDFs brutos são temporários durante a conversão; dependências, builds, perfil do Chromium, banco SQLite e dados locais permanecem ignorados pelo Git. Cookies e credenciais não são gravados no banco ou nos arquivos versionados.
