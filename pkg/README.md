# Parser de uma calculadora

## Descrição

Para que você crie programas complexos como por exemplo
os do tipo em que você analisa um conteúdo, e depois produz uma
resposta, sem que a solução fique fixa no seu código, isto entrega
a base de um projeto que pode expandir de acordo com a necessidade.

Então aqui, nós temos o seguinte:

```bash
cargo run -- "5 + 5"
```

Esta é a entrada que preparamos para o nosso programa lide
com ela e nos devolva um resultado, como este:

```bash
5 + 5 = 10
```

Simples, mas você pode parar e pensar, realmente é?
Talvez se você adiciona-se operação após operação as coisas
seriam alcançadas com facilidade, mas é aquilo que nós percebemos,
se nós formos incrementar estes testes, entradas mais complexas podem
vir a mente:

```bash
cargo run -- "(5 + ((10 * 2) / 5))"
```

Esta com certeza pode quebrar toda a implementação que desejarmos fazer,
e levar a nossa mente a loucura tentando entender como cobrir tantos
casos de uma só vez. Mas como nosso programa já esta preparado para receber
esta entrada de conteúdo, ele lida de uma forma bastante simples:

```bash
(5 + ((10 * 2) / 5)) = 9
```

Olha só que interessante, ele entendeu a precedencia e calculou exatamente
o que intencionamos na implementação, que esta o mais próximo possível dos
métodos utilizados pelas ferramentas de nosso dia a dia.

Bom, eu, através desta ferramenta consigo ver os próximos passos, até porque
eu gosto de refletir sobre as implementações que faço e entender de forma
mais objetetiva o que eu fiz através de um desafio onde tenho janela de tempo
para poder realizar algum experimento, porque com isso o estudo fica divertido
e ao mesmo tempo proveitoso, lembrando que você pode virar a chavinha do fator
`lucro` na equação e perfilar os seus trabalhos, para entregar próximo do tempo
de um trabalho real.

Eu venho por meio desta ferramenta compartilhar esta prática para que vocês tenham
também um bom estudo.

## Instalação

Basta clonar o projeto e ter as ferramentas para linguagem `Rust` e nada menos
que um:

```bash
cargo run -- [expressão]
```

Para tirar você do extremo zero.

## Créditos

Eu utilizei o pacote `nom` principalmente na implementação porque
ele ajuda com utilidades que estão presentes neste tipo de tarefa.