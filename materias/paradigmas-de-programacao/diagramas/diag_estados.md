# Diagrama de estados — classe Exemplar

> Versão em texto (Mermaid) do `diag_estados.png`, transcrita a partir da imagem. Usada pelo AVA 2 em `../ava2-modelagem-uml-biblioteca.md`.

```mermaid
stateDiagram-v2
    [*] --> Disponivel : Cadastro no acervo
    Disponivel --> Emprestado : emprestar / usuário regular
    Disponivel --> Reservado : reservar
    Disponivel --> BaixadoDoAcervo : baixa / mais de 5 anos sem uso ou dano
    Reservado --> Emprestado : retirar / reserva confirmada
    Reservado --> Disponivel : cancelar reserva / prazo expirado
    Emprestado --> Disponivel : devolver / sem reserva pendente
    Emprestado --> Bloqueado : devolver / com reserva pendente
    Emprestado --> BaixadoDoAcervo : perda de acervo / mais de 90 dias de atraso
    Bloqueado --> Emprestado : retirar / reserva confirmada
    Bloqueado --> Disponivel : cancelar reserva / prazo expirado
    Bloqueado --> BaixadoDoAcervo : baixa / dano comprovado
    BaixadoDoAcervo --> [*]
```

## Leitura textual

Um exemplar entra no acervo como `Disponivel`. Pode ser emprestado, reservado ou baixado. `BaixadoDoAcervo` é o estado final.
