# Instruções do acervo Faculdade

- Preserve os arquivos acadêmicos originais; não os sobrescreva nem os mova durante a catalogação.
- Trate notas, frequência, documentos e credenciais como dados privados locais. Nunca inclua senhas, cookies ou tokens em Markdown, JSON, logs ou Git.
- O Estuda UVA usa Rust/Tauri para ler o acervo e mantém o conteúdo acessível sem login no Canvas.
- Ao editar um registro acadêmico, mantenha as representações SQLite, Markdown e JSON sincronizadas e preserve conflitos para revisão humana.
- Diferencie dados confirmados por fonte oficial de estimativas, material didático e campos desconhecidos. Registre fonte e data para notas, frequência e progresso.
- Execute verificações focadas antes de concluir mudanças e relate claramente o que foi e o que não foi validado.
- Conteúdo versionado é texto: PDF e documento (Word, Pages) viram Markdown com página e fonte; diagramas e imagens viram Mermaid ou descrição textual. Binários ficam fora do Git. `node tools/acervo/build.mjs --verificar-texto` aponta o que ainda é binário.
