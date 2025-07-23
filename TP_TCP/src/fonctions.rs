// Tout ce qui est commenté correspond aux exercices fait précédemment pendant le cours

/* use std::fs::File;
use std::io::{self, BufReader, Read};

struct Fichier {
    nom: String,
}

impl Fichier {
    // Fonction pour créer un fichier (déjà vue)
    fn creer(nom: &str) -> io::Result<Fichier> {
        File::create(nom)?;
        Ok(Fichier {
            nom: nom.to_string(),
        })
    }

    // Fonction pour lire le contenu du fichier
    fn lire(&self) -> io::Result<String> {
        let fichier = File::open(&self.nom)?;              // Ouvre le fichier
        let mut reader = BufReader::new(fichier);          // Crée le buffer reader
        let mut contenu = String::new();
        reader.read_to_string(&mut contenu)?;              // Lis tout le contenu dans la string
        Ok(contenu)
    }
}

fn main() -> io::Result<()> {
    // Supposons que le fichier "ecrire.rs" existe et contient du texte
    let fichier = Fichier::creer("ecrire.rs")?;

    // Ici, on pourrait écrire quelque chose avant, mais juste pour lire :
    let contenu = fichier.lire()?;
    println!("Contenu du fichier :\n{}", contenu);
    
    Ok(())
} */


/* use std::fs::File;
use std::io::{self, BufReader, Read, Write};

struct Fichier {
    nom: String,
}

impl Fichier {
    // Crée le fichier et écrit le contenu dedans
    fn creer_avec_contenu(nom: &str, contenu: &str) -> io::Result<Fichier> {
        let mut fichier = File::create(nom)?;          // Création fichier
        fichier.write_all(contenu.as_bytes())?;        // Écriture contenu
        Ok(Fichier {
            nom: nom.to_string(),
        })
    }

    // Lire le contenu du fichier (déjà vu)
    fn lire(&self) -> io::Result<String> {
        let fichier = File::open(&self.nom)?;
        let mut reader = BufReader::new(fichier);
        let mut contenu = String::new();
        reader.read_to_string(&mut contenu)?;
        Ok(contenu)
    }
}

fn main() -> io::Result<()> {
    // Demander le nom du fichier
    println!("Entrez le nom du fichier à créer : ");
    let mut nom_fichier = String::new();
    io::stdin().read_line(&mut nom_fichier)?;
    let nom_fichier = nom_fichier.trim(); // Formatage de la chaine pour les retour chariot au cas ou

    // Demander le contenu à écrire
    println!("Entrez le contenu à écrire dans le fichier : ");
    let mut contenu = String::new();
    io::stdin().read_line(&mut contenu)?;
    let contenu = contenu.trim(); // Formatage

    // Créer le fichier avec ce contenu
    let fichier = Fichier::creer_avec_contenu(nom_fichier, contenu)?;

    println!("Fichier '{}' créé avec le contenu suivant :", fichier.nom);
    println!("{}", fichier.lire()?);

    Ok(())
}
*/

/* 
// Serveur TCP, pour tester on peut par exemple lancer une connection telnet depuis le client. Le serveur renverra un message pour notifier de la connexion ddu client et de son ID
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// Structure Client pour représenter chaque client connecté
struct Client {
    id: usize,
    stream: TcpStream,
}

// Implémentation du client
impl Client {
    fn new(id: usize, stream: TcpStream) -> Client {
        Client { id, stream }
    }
}

// Fonction de la gestion client
fn handle_client(mut client: Client, clients: Arc<Mutex<Vec<usize>>>) {
    let mut buffer = [0u8; 512];
    loop {
        match client.stream.read(&mut buffer) {
            Ok(0) => {
                // message de notification pour client déconnecté
                println!("Client {} a quitté", client.id);
                // Retirer le client de la liste
                let mut clients_lock = clients.lock().unwrap();
                clients_lock.retain(|&x| x != client.id);
                break;
            }
            Ok(n) => {
                // Lire le message
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Client {}: {}", client.id, message);

                // Répondre au client (echo)
                if let Err(e) = client.stream.write_all(message.as_bytes()) {
                    eprintln!("Erreur en écriture vers le client {}: {}", client.id, e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Erreur lecture client {}: {}", client.id, e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Serveur TCP démarré sur le port 7878");

    // Liste thread-safe (Mutex) de IDs de clients connectés (propriétaire)
    let clients = Arc::new(Mutex::new(Vec::new()));

    let mut next_id = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                next_id += 1;
                let client = Client::new(next_id, stream);
                println!("Nouveau client connecté: id {}", client.id);

                // Ajouter l’ID client à la liste
                let clients_clone = Arc::clone(&clients);
                {
                    let mut clients_lock = clients_clone.lock().unwrap();
                    clients_lock.push(client.id);
                }

                // Spawn un thread pour gérer la connexion du client
                thread::spawn(move || {
                    handle_client(client, clients_clone);
                    println!("Fin thread client {}", next_id);
                });
            }
            Err(e) => eprintln!("Erreur en acceptant une connexion : {}", e),
        }
    }

    Ok(())
}
*/

// TP serveur TCP
use std::net::TcpStream;
use std::io::Read;

// Structure représentant un client connecté
pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Client { stream }
    }
}

// Fonction publique pour gérer la connexion d'un client
pub fn handle_client(mut client: Client) {
    let mut buffer = [0u8; 512];

    loop {
        match client.stream.read(&mut buffer) {
            Ok(0) => break, // client a fermé la connexion
            Ok(_) => {
                // Simple lecture sans traitement, selon consigne
            }
            Err(_) => break,
        }
    }
}
