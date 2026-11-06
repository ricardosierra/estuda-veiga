# Diagrama de classes — sistema de biblioteca

> Versão em texto (Mermaid) do `diag_classes.png`, transcrita a partir da imagem. Usada pelo AVA 2 em `../ava2-modelagem-uml-biblioteca.md`. A imagem original fica como referência visual local.

```mermaid
classDiagram
    class Usuario {
        <<abstract>>
        -String matricula
        -String nome
        -Date dataNascimento
        -String endereco
        -String telefone
        -String email
        +getMatricula() String
        +getNome() String
    }
    class Aluno {
        -String curso
        +getCurso() String
    }
    class Professor {
        -String titulacao
        +getTitulacao() String
    }
    class FuncionarioAdministrativo {
        -String setor
        +getSetor() String
    }
    class Bibliotecaria {
        -String matricula
        -String nome
        -String email
        +registrarEmprestimo() Emprestimo
        +registrarDevolucao() void
        +registrarReserva() Reserva
        +baixarExemplar() void
    }
    class Publicacao {
        <<abstract>>
        -int idPublicacao
        -String titulo
        -double valorAtualEmprestimo
        -double valorRessarcimento
        +getValorAtualEmprestimo() double
        +getValorRessarcimento() double
    }
    class Periodico {
        -String volumeNumero
        -String issn
        -Date dataPublicacao
    }
    class Livro {
        -String autor
        -String isbn
        -String editora
        -int edicao
    }
    class Emprestimo {
        -int idEmprestimo
        -Date dataEmprestimo
        -Date dataPrevistaDevolucao
        -Date dataRealDevolucao
        -double valorTotalEmprestimo
        +calcularValor() double
        +registrarDevolucao() void
        +verificarAtraso() boolean
    }
    class Reserva {
        -int idReserva
        -Date dataReserva
        -Date dataLimiteEmprestimo
        -StatusReserva statusReserva
        +confirmarReserva() void
        +cancelarReserva() void
    }
    class Exemplar {
        -String identificadorExemplar
        -Date dataAquisicao
        -Date dataBaixa
        -StatusExemplar status
        +getStatus() StatusExemplar
        +atualizarStatus(novoStatus StatusExemplar) void
    }
    class StatusReserva {
        <<enumeration>>
        PENDENTE
        CONFIRMADA
        CANCELADA
    }
    class StatusExemplar {
        <<enumeration>>
        DISPONIVEL
        EMPRESTADO
        RESERVADO
        BLOQUEADO
        BAIXADO_DO_ACERVO
    }

    Usuario <|-- Aluno
    Usuario <|-- Professor
    Usuario <|-- FuncionarioAdministrativo
    Publicacao <|-- Periodico
    Publicacao <|-- Livro
    Usuario "1" -- "0..*" Emprestimo : realiza
    Usuario "1" -- "0..*" Reserva : solicita
    Bibliotecaria "1" -- "0..*" Emprestimo : registra
    Bibliotecaria "1" -- "0..*" Reserva : registra
    Emprestimo "1" o-- "1..3" Exemplar : contempla
    Reserva "1" o-- "1..3" Exemplar : reserva
    Publicacao "1" *-- "1..*" Exemplar : possui
    Reserva --> StatusReserva
    Exemplar --> StatusExemplar
```

## Leitura textual das relações

- **Herança:** `Aluno`, `Professor` e `FuncionarioAdministrativo` são subclasses de `Usuario` (abstrata). `Periodico` e `Livro` são subclasses de `Publicacao` (abstrata).
- **Empréstimo:** um `Usuario` realiza 0..* empréstimos; uma `Bibliotecaria` registra 0..* empréstimos; cada empréstimo contempla de 1 a 3 `Exemplar`.
- **Reserva:** um `Usuario` solicita 0..* reservas; uma `Bibliotecaria` registra 0..*; cada reserva reserva de 1 a 3 `Exemplar`.
- **Composição:** cada `Publicacao` possui 1..* `Exemplar`.
- **Enumerações:** `Reserva` usa `StatusReserva`; `Exemplar` usa `StatusExemplar`.
