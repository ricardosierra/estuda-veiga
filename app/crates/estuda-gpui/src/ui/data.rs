//! Conteúdo do layout do canvas "Estuda UVA" (`Estuda UVA.dc.html`), copiado literalmente.
//! Esses valores são o protótipo; a ligação com o acervo real acontece em `ui::AppView`.

/// Disciplinas de 2026.2: (nome, nome curto, observação).
pub const SUBJECTS: [(&str, &str, &str); 7] = [
    ("Cálculo de Uma Variável", "Cálculo", ""),
    (
        "Bases de Programação",
        "Bases de Programação",
        "No Canvas aparece em “Inscrições anteriores”; não indica cancelamento",
    ),
    ("Linguagens Formais e Autômatos", "LFA", ""),
    (
        "Paradigmas de Linguagens de Programação",
        "Paradigmas",
        "Arquivos do módulo são de ASI1 · confirmar",
    ),
    ("Estatística para Computação", "Estatística", ""),
    (
        "Projeto de Extensão I – Ciência da Computação",
        "Extensão I – CC",
        "",
    ),
    (
        "Projeto de Extensão I",
        "Extensão I – 21CIV2262A",
        "Turma 21CIV2262A; Canvas mostra Engenharia Civil · confirmar",
    ),
];

pub const OPEN_QUESTIONS: [(&str, &str); 4] = [
    (
        "Significado de “P4”",
        "Só aparece no texto de um e-book. Não é tratado como avaliação.",
    ),
    (
        "Turma 21CIV2262A",
        "Canvas mostra Engenharia Civil para Projeto de Extensão I.",
    ),
    ("Paradigmas × ASI1", "Arquivos do módulo são de ASI1."),
    ("Calendário 2027.1", "Não publicado até outubro de 2026."),
];

/// Raias do quadro: (título, origem, descrição, estado).
pub const LANES: [(&str, &str, &str, &str); 4] = [
    ("AVA 1", "Canvas", "Atividade avaliativa online, com prazo.", "desconhecido"),
    ("AVA 2", "Canvas", "Atividade avaliativa online, com prazo.", "desconhecido"),
    ("Avaliação 2", "Sistema de Provas", "Prova regular.", "a confirmar"),
    ("Avaliação 3", "Sistema de Provas", "Prova substituta.", "a confirmar"),
];

/// Documentos da biblioteca: (tipo, título, disciplina, origem, estado, filtro).
pub const DOCS: [(&str, &str, &str, &str, &str, &str); 6] = [
    ("PDF→MD", "Registro de avaliações 2026.2", "Todas", "Portal", "convertido", "Portal"),
    ("PDF→MD", "Calendário Acadêmico 2026.1", "Todas", "Calendário Acadêmico", "reaproveitado local", "Portal"),
    ("PDF→MD", "Módulo — arquivos de ASI1", "Paradigmas (confirmar)", "Canvas", "convertido", "Canvas"),
    ("E-book", "E-book com ocorrência de “P4” no texto", "Disciplina a identificar", "Canvas", "convertido", "Canvas"),
    ("Nota", "Anotações pessoais — LFA", "Linguagens Formais e Autômatos", "Estudo pessoal", "local", "Pessoal"),
    ("PDF→MD", "Espaço Curso — Ciência da Computação", "Espaço de apoio, não matéria", "Canvas 53418", "na fila", "Canvas"),
];

/// Cartões de estudo: (pergunta, resposta, baralho).
pub const CARDS: [(&str, &str, &str); 4] = [
    (
        "O que define um autômato finito determinístico?",
        "Para cada estado e símbolo há exatamente uma transição; aceita linguagens regulares.",
        "LFA",
    ),
    (
        "Paradigma funcional: qual a ideia central?",
        "Computar avaliando funções, evitando estado mutável e efeitos colaterais.",
        "Paradigmas",
    ),
    (
        "Média vs. mediana: quando preferir a mediana?",
        "Quando há valores extremos que distorcem a média.",
        "Estatística",
    ),
    (
        "O que é o limite de f(x) quando x → a?",
        "O valor do qual f(x) se aproxima quando x se aproxima de a, sem precisar atingi-lo.",
        "Cálculo",
    ),
];

/// Questionário de treino: (pergunta, opções, índice correto).
pub const QUIZ: [(&str, [&str; 4], usize); 3] = [
    (
        "Qual linguagem NÃO é regular?",
        ["a*b*", "{aⁿbⁿ | n ≥ 0}", "(ab)*", "a|b"],
        1,
    ),
    (
        "Qual estrutura é típica do paradigma imperativo?",
        ["Atribuição de variáveis", "Cláusulas Horn", "Cálculo lambda puro", "Unificação"],
        0,
    ),
    (
        "A variância de um conjunto constante é:",
        ["1", "A média", "0", "Indefinida"],
        2,
    ),
];

/// Grade semanal do plano: (dia, [(horário, matéria)]).
pub const WEEK: [(&str, &[(&str, &str)]); 7] = [
    ("Seg", &[("19:00", "Cálculo")]),
    ("Ter", &[("19:00", "LFA")]),
    ("Qua", &[("20:00", "Estatística")]),
    ("Qui", &[("19:00", "Paradigmas")]),
    ("Sex", &[("19:30", "Bases de Programação")]),
    ("Sáb", &[("10:00", "Revisão + flashcards"), ("15:00", "Extensão I")]),
    ("Dom", &[]),
];

/// Resumos gerados: (título, texto, origem).
pub const SUMMARIES: [(&str, &str, &str); 3] = [
    (
        "Autômatos finitos",
        "AFD e AFN reconhecem as mesmas linguagens; todo AFN pode ser convertido em AFD pela construção de subconjuntos.",
        "Anotações pessoais — LFA",
    ),
    (
        "Calendário 2026.1",
        "A2 e A3 aparecem como provas, com agendamento feito no Sistema de Provas.",
        "Calendário Acadêmico 2026.1",
    ),
    (
        "Módulo ASI1",
        "Conteúdo listado sob Paradigmas no Canvas. Vínculo com a disciplina ainda a confirmar.",
        "Módulo — arquivos de ASI1",
    ),
];

/// Itens da checklist de Organizar.
pub const CHECKS: [&str; 5] = [
    "Ler o enunciado completo",
    "Anotar prazo (fonte: Canvas)",
    "Revisar materiais ligados",
    "Escrever rascunho",
    "Entregar no Canvas (por você)",
];

/// Preferências de privacidade: (título, descrição, fixa).
pub const PREFS: [(&str, &str, bool); 4] = [
    ("Nunca guardar credenciais", "Senha, cookie, token e CPF não vão para disco, logs ou Git", true),
    ("Trava de banco", "Toda alteração no SQLite pede sua aprovação antes", true),
    ("Somente leitura nos portais", "Sem agendar, enviar, solicitar ou confirmar", true),
    ("Reaproveitar material local", "Requisições condicionais (ETag / Last-Modified)", false),
];

/// Fontes da sincronização: (nome, escopo, ícone).
pub const SOURCES: [(&str, &str, &str); 5] = [
    ("Portal do Aluno", "Login manual · escolha da matrícula Ativo", "shield"),
    ("Sistema Acadêmico", "Notas, frequência oficial e histórico", "school"),
    ("Canvas", "AVA 1, AVA 2, prazos e notas de atividades", "globe"),
    ("Sistema de Provas", "Datas e situação das provas · sem agendar", "calendar"),
    ("Calendário Acadêmico", "Datas oficiais · 2027.1 ainda não publicado", "clock"),
];

pub const STEPS: [&str; 4] = ["Entender", "Revisar material", "Resolver junto", "Conferir"];

/// Títulos e subtítulos do cabeçalho por tela.
pub fn title_for(screen: &str) -> (&'static str, &'static str) {
    match screen {
        "painel" => ("Painel", "Visão geral de 2026.2 · dados sem fonte aparecem como desconhecido"),
        "academico" => ("Acadêmico", "AVA 1, AVA 2, provas e tipos de avaliação por disciplina"),
        "sync" => ("Sincronização", "Portal, Canvas, Sistema de Provas e Calendário · somente leitura"),
        "biblioteca" => ("Biblioteca", "Acervo local em Markdown · busca em memória"),
        "leitor" => ("Leitor PDF→MD", "Conversão leve, PDF bruto não fica em memória"),
        "estudo" => ("Estudo", "Plano semanal, flashcards e resumos do acervo"),
        "auxilio" => ("Auxílio", "Tutor, organização e simulados para atividades e provas"),
        _ => ("Usuário", "Perfil, matrículas e privacidade"),
    }
}
