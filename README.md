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
