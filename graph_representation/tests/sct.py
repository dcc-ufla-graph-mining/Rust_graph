import sys

def processar_arquivo(nome_arquivo):
    # Lista para armazenar o conteúdo do arquivo modificado
    conteudo_modificado = []

    with open(nome_arquivo, 'r') as arquivo:
        # Lendo o arquivo linha por linha
        linhas = arquivo.readlines()

        for i, linha in enumerate(linhas):
            # Adiciona a linha atual ao conteúdo modificado
            if (i + 1) % 5 == 1:
                conteudo_modificado.append(linha.strip())
                #print(linha)

            # Verifica se estamos na terceira linha de cada sequência
            if (i + 1) % 5 == 3:
                # Adiciona a terceira linha ao final da primeira linha
                conteudo_modificado[-1] += linha.strip().strip("real  ").strip("\n");


    return '\n'.join(conteudo_modificado)


nome_arquivo = sys.argv[1]
conteudo_processado = processar_arquivo(nome_arquivo)

# Escreve o conteúdo processado em um novo arquivo
with open(nome_arquivo + ".result", 'w') as arquivo_processado:
    arquivo_processado.write(conteudo_processado)

print("Arquivo processado com sucesso!")

