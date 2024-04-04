#!/bin/bash

for j in {3..15}; do
    echo "Processando com subgrafos de tamanho $j"
    for i in {1..10}; do
        random=$(shuf -i 1-1000 -n 1)
        echo -n "$i $random " >> resultados_$1.txt
        (time cargo run --quiet -- $1 $j $random $2) &>> resultados_$1.txt
    done
    clear
done
