# Relatório técnico

## Conteúdo
 - Implementação da busca de profundidade, busca em largura e algoritmo de dijkstra
 - Explicação da forma de funcionamento
 - Pseudocódigo
 - Trecho de código implementado

## Busca em profundidade

### Explicação da forma de funcionamento

Dado um vértice inicial, o algoritmo busca os vértices adjacentes e cada vizinho, e caso ainda não tenha sido visitado, é colocado em uma pilha e marcado como visitado. Após isso, a mesma coisa acontece para o primeiro item da pilha. O processo se repete até que todos os vértices sejam visitados, consequentemente deixando a pilha vázia. 

### Pseudocódigo
```
escolha um nó inicial
adicione o nó inicial no histórico de itens visitados
adicione o nó inicial na pilha

enquanto houver item na pilha:
    adicione esse item no histórico de itens visitados
    busque todos os nós adjacentes desse item:
        se esse nó adjacente não foi visitado:
            marque o nó como visitado
            adicione esse nó na pilha

retorne o histórico de itens visitados
```

### Trecho do código implementado
![Snippet](/img/depth_first_search.png)

## Busca em largura

### Explicação da forma de funcionamento
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque pretium dui turpis, ut dapibus augue pellentesque quis. Aenean sit amet nisi rutrum, varius diam pretium, pharetra magna. Nam non condimentum massa. Ut posuere, metus vitae ultricies luctus, augue lacus ultrices ipsum, sed mollis mauris mauris non sapien.

### Pseudocódigo
```
paste your pseudocodigo here
```

### Trecho do código implementado
![Snippet](/img/breadth_first_search.png)

## Algoritmo de dijkstra

### Explicação da forma de funcionamento
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque pretium dui turpis, ut dapibus augue pellentesque quis. Aenean sit amet nisi rutrum, varius diam pretium, pharetra magna. Nam non condimentum massa. Ut posuere, metus vitae ultricies luctus, augue lacus ultrices ipsum, sed mollis mauris mauris non sapien.

### Pseudocódigo
```rs
paste your pseudocodigo here
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