# Relatório técnico

## Conteúdo
 - Descrição dos campos do Tipo Abstrato de Dado (TAD) proposto;
 - Implementação da busca de profundidade, busca em largura e algoritmo de dijkstra
 - Explicação da forma de funcionamento
 - Pseudocódigo
 - Trecho de código implementado

## Descrição TAD

![TAD](/img/tad.png)

A estrutura `Node` representa um vértice
 - `code`: código do vértice.
 - `name`: nome do vértice.

A estrutura `Edge` representa uma aresta
 - `from`: código do vértice que essa aresta parte.
 - `to`: código do vértice destino.
 - `weight`: peso dessa aresta.

A estrutura `Graph` representa o grafo
 - `is_weighted`: representa se esse grafo é ponderado ou não.
 - `edges`: lista de arestas pertencentes desse grafo.
 - `nodes`: lista de vértices pertencentes desse grafo.


## Busca em profundidade

### Explicação da forma de funcionamento

Dado um vértice inicial, o algoritmo busca os vértices adjacentes e cada vizinho, e caso ainda não tenha sido visitado, é colocado em uma pilha e marcado como visitado. Após isso, a mesma coisa acontece para o último item adicionado na pilha e assim por diante. O processo se repete até que todos os vértices sejam visitados, consequentemente deixando a pilha vazia. 

### Pseudocódigo
```
escolha um nó inicial
adicione o nó inicial no histórico de itens visitados
adicione o nó inicial na pilha

enquanto houver item na pilha:
    pegue o último item adicionado na pilha
    adicione esse item no histórico de itens visitados
    busque todos os nós adjacentes desse item:
        se esse nó adjacente não foi visitado:
            marque o nó como visitado
            adicione esse nó no final da pilha

retorne o histórico de itens visitados
```

### Trecho do código implementado
![Snippet](/img/depth_first_search.png)

## Busca em largura

### Explicação da forma de funcionamento
Dado um vértice inicial, o algoritmo busca os vértices adjacentes e cada vizinho, e caso ainda não tenha sido visitado, é colocado no final de uma fila e marcado como visitado. Após isso, a mesma coisa acontece para o primeiro item da fila e assim por diante. O processo se repete até que todos os vértices sejam visitados, consequentemente deixando a fila vazia. 

### Pseudocódigo
```
escolha um nó inicial
adicione o nó inicial no histórico de itens visitados
adicione o nó inicial na fila

enquanto houver item na fila:
    pegue o primeiro item da fila
    adicione esse item no histórico de itens visitados
    busque todos os nós adjacentes desse item:
        se esse nó adjacente não foi visitado:
            marque o nó como visitado
            adicione esse nó no final da fila

retorne o histórico de itens visitados
```

### Trecho do código implementado
![Snippet](/img/breadth_first_search.png)

## Algoritmo de dijkstra

### Explicação da forma de funcionamento
O algoritmo usa uma estrutura Hash map (chave-valor) para armazenar o vértice (chave) e a distância até ele (valor) em relação a um dado vértice inicial. Inicialmente esse Hash map começa com todas as distâncias no máximo (ou algo que indique que essa distância não foi calculada ainda). O algoritmo, parecido com o de busca em largura, usa uma pilha para controlar os vértices que ainda precisam ser visitado, enquanto essa pilha não estiver vazia, o vértice atual é marcado como visitado, e para cada aresta conectada a esse vértice, é calculada a distância do vértice atual mais o peso da aresta, se ela for menor que a distância armazenada no Hash map, insere ou subistitui o antigo valor com essa nova distância, após isso, o vértice da atual aresta é adicionado na pilha para a próxima iteração, isso se repete até todos os vértices terem sido visitados e a pilha ficar vazia.

### Pseudocódigo
```
vértice inicial V

distancias[V] = -1
pilha = [(V, -1)]
visitados = []

para cada V e distancia D enquanto a pilha não estiver vazia:
    se visitados contém V:
        próxima iteração
    se não:
        adiciona V nos visitados

    para cada aresta A conectada em V:
        nova_distancia  = D + peso de A
        se nova_distancia for menor que distancias[V]:
            distancias[V] = nova_distancia
            adicione o vizinho dessa aresta na pilha

retorne distancias            

```

### Trecho do código implementado
![Snippet](/img/dijkstra.png)

## Comparação dos algoritmos de Dijkstra, Floyd e Bellman-Ford

### Dijkstra
Dado um vértice inicial, o algoritmo de Dijkstra encontra o menor caminho entre **todos** os vértices do grafo, desde que exista um caminho entre eles. Funciona com grafos e dígrafos, sendo eles ponderados ou não. Não funciona com arestas negativas.

### Bellman-Ford
É bem similar ao algoritmo de Dijkstra. Necessita de um vértice inicial e **deve** ser direcionado e pondarado. As arestas podem conter valores negativos.

### Floyd
Já o algoritmo de Floyd funciona de forma diferente dos demais, não precisamos definir um ponto de origem e nos mostra vários caminhos entre os vértices. O algoritmo de Floyd calcula o peso entre **todos** os pares de vértices do grafo. Aceita grafos direcionados ou não e assim como o de Bellman-Ford, aceita valores negativos nas arestas.

### Tabela comparativa

|                   | Dijkstra | Bellman-Ford | Floyd |
|-------------------|:--------:|:------------:|:-----:|
| Arestas negativas |     ✖️    |       ✔️      |   ✔️   |
| Vértice Inicial   |     ✔️    |       ✔️      |   ✖️   |
| Grafos            |     ✔️    |       ✔️      |   ✔️   |
| Dígrafos          |     ✔️    |       ✔️      |   ✔️   |
| Direcionado       |     ✔️    |       ✔️      |   ✔️   |
| Não Direcionado   |     ✔️    |       ✖️      |   ✔️   |
| Ponderado         |     ✔️    |       ✔️      |   ✔️   |
| Não Ponderado     |     ✔️    |       ✖️      |   ✖️   |
