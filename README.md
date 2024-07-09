# Reverse Shell em Rust

## Visão Geral

Este projeto apresenta a implementação de um reverse shell utilizando a linguagem de programação Rust. Um reverse shell é uma técnica em que o alvo (servidor) se conecta de volta ao atacante (cliente), permitindo ao atacante executar comandos no sistema do alvo.

## Estrutura do Projeto

O projeto é dividido em dois componentes principais:

1. **Servidor**
2. **Cliente**

### Servidor

O servidor é responsável por escutar conexões de entrada em uma porta específica e executar comandos recebidos do cliente. Ele utiliza sockets TCP para estabelecer conexões e executar comandos no sistema operacional subjacente, retornando a saída desses comandos de volta para o cliente.

**Principais Funcionalidades do Servidor:**
- Escuta em uma porta definida (neste exemplo, a porta 7878).
- Aceita conexões de clientes.
- Lê comandos enviados pelo cliente.
- Executa os comandos no sistema operacional.
- Envia a saída dos comandos de volta para o cliente.

### Cliente

O cliente é responsável por conectar-se ao servidor na porta especificada, enviar comandos e receber a saída desses comandos executados pelo servidor. Ele também utiliza sockets TCP para comunicação.

**Principais Funcionalidades do Cliente:**
- Conecta-se ao servidor na porta 7878.
- Envia comandos ao servidor.
- Recebe a saída dos comandos executados pelo servidor.

## Fluxo de Trabalho

1. **Início do Servidor:**
   - O servidor inicia e começa a escutar conexões na porta 7878.
   - Quando uma conexão é estabelecida, o servidor aceita a conexão e aguarda comandos.

2. **Conexão do Cliente:**
   - O cliente se conecta ao servidor na porta 7878.
   - Uma vez conectado, o cliente pode enviar comandos para o servidor.

3. **Execução de Comandos:**
   - O servidor recebe os comandos do cliente, executa-os localmente e envia a saída de volta para o cliente.
   - O cliente exibe a saída dos comandos recebidos do servidor.
