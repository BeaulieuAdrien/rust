use dashmap::DashMap;
use rayon::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::Arc;
use std::time::Instant;

fn main() -> io::Result<()> {

    // Enregistre l'heure de début
    let start_time = Instant::now();

    // Ouverture du fichier
    let path = "../hello/ssl_access_log";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Création des HashMaps pour les codes HTTP et les IPs
    let http_codes = Arc::new(DashMap::new());
    let ips = Arc::new(DashMap::new());

    // Regex pour les codes HTTP
    let http_code_regex = Regex::new(r"\s([0-9]{3})\s").unwrap();

    // boucle sur le fichier
    reader
        .lines()
        // Utilisation de Rayon pour itérer en parallèle
        .par_bridge()
        .for_each(|line| {
            // On vérifie si la ligne est correcte
            if let Ok(line) = line { 
                // Si on trouve un code HTTP
                if let Some(code_match) = http_code_regex.find(&line) {
                    let code = code_match.as_str().trim().to_string();
                    http_codes.entry(code).and_modify(|e| *e += 1).or_insert(1);
                }
                // Si on trouve une IP
                if let Some(ip) = line.split_whitespace().nth(0) {
                    ips.entry(ip.to_string()).and_modify(|e| *e += 1).or_insert(1);
                }
            }
        });

        // TRI ET AFFICHAGE DES RÉSULTATS
    println!(
        "TOTAL HTTP REQUEST : {}",
        ips.iter().map(|entry| *entry.value()).sum::<usize>()
    );

    let mut sorted_http_codes: Vec<_> = http_codes.iter().collect();
    sorted_http_codes.sort_by_key(|entry| entry.key().clone()); // Tri par la clé

    for entry in &sorted_http_codes {
        println!("HTTP Status {}: {}", entry.key(), entry.value());
    }

    println!("Unique IP : {}", ips.len());

    if let Some(biggest_ip) = ips.iter().max_by_key(|entry| *entry.value()) {
        println!("Most recurrent IP {}: {}", biggest_ip.key(), biggest_ip.value());
    }

    let end_time = Instant::now();
    let execution_time = end_time - start_time;
    println!("Execution time: {:?}", execution_time);

    Ok(())
}
