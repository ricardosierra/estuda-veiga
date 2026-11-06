# Instruções do acervo Faculdade

- Preserve os arquivos acadêmicos originais; não os sobrescreva nem os mova durante a catalogação.
- **Política de publicação (decisão do dono, 6/11/2026):** todo conteúdo pode ir ao acervo público e ao Git, inclusive notas, frequência, matrículas e trabalhos, **desde que separado** do catálogo (em `privado/`, ver abaixo). Não trate esses dados como segredo nem se recuse a versioná-los. **Continuam proibidos, sem exceção:** senhas, cookies, tokens, `.env`, bancos SQLite (`*.sqlite3*`), perfil do Chromium e binários originais (`originais/`). RA, CPF e RG seguem fora dos registros até o dono decidir o contrário.
- O Estuda UVA usa Rust/Tauri para ler o acervo e mantém o conteúdo acessível sem login no Canvas.
- Ao editar um registro acadêmico, mantenha as representações SQLite, Markdown e JSON sincronizadas e preserve conflitos para revisão humana.
- Diferencie dados confirmados por fonte oficial de estimativas, material didático e campos desconhecidos. Registre fonte e data para notas, frequência e progresso.
- Execute verificações focadas antes de concluir mudanças e relate claramente o que foi e o que não foi validado.
- Conteúdo versionado é texto: PDF e documento (Word, Pages) viram Markdown com página e fonte; diagramas e imagens viram Mermaid ou descrição textual. Binários ficam fora do Git. `node tools/acervo/build.mjs --verificar-texto` aponta o que ainda é binário.
- Dados pessoais do aluno (matrícula, notas, frequência, trajetória, trabalhos entregues, planejamento) ficam em `privado/`, no mesmo repositório. O restante é matéria e instituição. `dados/` nunca contém dado pessoal.


Requisitos:
- Memory Ram First - Primeiro otimizar a memória RAM, tudo sempre ir pra ela pra ser fluido e rapido só depois em lote, persistir em disco ou rede.
- Operações de arquivo e SQLite fora do loop de renderização.
- Tarefas de rede assíncronas.
- Cancelamento explícito.
- Atualização incremental da interface.
- Cache de estado compartilhado quando necessário.
- Evitar locks longos.
- Nunca manter Mutex bloqueado através de .await.
- Otimizar consumo de memória, cpu, disco e rede.
- Encerrar corretamente tarefas e recursos ao fechar o aplicativo.
- Tratamento de falhas sem encerrar toda a aplicação.