# Diagrama de sequência — empréstimo de exemplar

> Versão em texto (Mermaid) do `diag_sequencia.png`, transcrita a partir da imagem. Usada pelo AVA 2 em `../ava2-modelagem-uml-biblioteca.md`.

```mermaid
sequenceDiagram
    autonumber
    actor U as u Usuario
    actor B as b Bibliotecaria
    participant S as SistemaBiblioteca
    participant UO as Usuario objeto
    participant EX as Exemplar
    participant PU as Publicacao
    participant EM as Emprestimo

    U->>B: solicitaEmprestimo(exemplares[1..3])
    activate B
    B->>S: iniciarEmprestimo(matriculaUsuario, idExemplares)
    activate S
    S->>UO: verificarSituacao(matriculaUsuario)
    UO-->>S: statusRegular

    loop para cada exemplar (1 a 3)
        S->>EX: verificarStatus(idExemplar)
        EX-->>S: status = DISPONIVEL
        S->>PU: obterValorEmprestimo(idPublicacao)
        PU-->>S: valorAtualEmprestimo
    end

    S->>EM: registrar(usuario, bibliotecaria, exemplares, datas, valorTotal)
    EM-->>S: emprestimoCriado

    loop para cada exemplar
        S->>EX: atualizarStatus(EMPRESTADO)
        EX-->>S: ok
    end

    S->>B: solicitarPagamento(valorTotal)
    B->>U: informaValor(valorTotal)
    U->>B: efetuaPagamento()
    B->>S: confirmarPagamento()
    S-->>B: emitirComprovante()
    B->>U: entregaExemplaresEComprovante()
    deactivate S
    deactivate B
```

## Leitura textual

O usuário pede até três exemplares. O sistema verifica a situação do usuário e a disponibilidade de cada exemplar, registra o empréstimo, marca os exemplares como emprestados, solicita o pagamento e entrega os exemplares com o comprovante.
