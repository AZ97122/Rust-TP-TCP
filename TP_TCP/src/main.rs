mod fonctions;

use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use fonctions::{Client, handle_client};

fn main() -> std::io::Result<()> {
    let clients = Arc::new(Mutex::new(Vec::<Client>::new()));

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Serveur TCP démarré sur 127.0.0.1:7878");

    for stream_res in listener.incoming() {
        let stream = stream_res?;
        let peer_addr = stream.peer_addr().unwrap_or_else(|_| "inconnue".parse().unwrap());

        // Affichage du message de connexion
        println!("Un client s'est connecté depuis {}", peer_addr);

        // Création Client
        let client = Client::new(stream);

        let clients_ref = Arc::clone(&clients);

        {
            let mut locked_clients = clients_ref.lock().unwrap();
            locked_clients.push(Client::new(client.stream.try_clone().unwrap()));
        }

        thread::spawn(move || {
            handle_client(client);

            let mut locked_clients = clients_ref.lock().unwrap();
            locked_clients.pop();
        });
    }

    Ok(())
}

