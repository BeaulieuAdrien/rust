use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {

    // Enregistre l'heure de début
    let start_time = Instant::now(); 

    // Ouverture du fichier
    let path = "ssl_access_log";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Création des HashMaps pour les codes HTTP et les IPs
    let mut http_codes = HashMap::new();
    let mut ips = HashMap::new();

    // Regex pour les codes HTTP
    let http_code_regex = Regex::new(r"\s([0-9]{3})\s").unwrap();

    // boucle sur le fichier
    for line in reader.lines() {
        let line = line?;
        // Si on trouve un code HTTP
        if let Some(code_match) = http_code_regex.find(&line) {
            let code = code_match.as_str().trim();
            // On incrémente le compteur
            *http_codes.entry(String::from(code)).or_insert(0) += 1;
        }
        // Si on trouve une IP
        if let Some(ip) = line.split_whitespace().nth(0) {
            // On incrémente le compteur
            *ips.entry(String::from(ip)).or_insert(0) += 1;
        }
    }

    // Affichage des résultats
    println!("TOTAL HTTP REQUEST : {}", ips.values().sum::<usize>());

    let mut sorted_http_codes: Vec<_> = http_codes.iter().collect();
    sorted_http_codes.sort_by_key(|&(code, _)| code);

    for (code, count) in sorted_http_codes {
        println!("HTTP Status {}: {}", code, count);
    }

    println!("Unique IP : {}", ips.len());

    let biggest_ip = ips.iter().max_by_key(|&(_, count)| count).unwrap();
    println!("Most recurrent IP {}: {}", biggest_ip.0, biggest_ip.1);


    // Affiche le temps d'exécution
    let end_time = Instant::now();
    let execution_time = end_time - start_time;

    println!("Execution time: {:?}", execution_time);

    Ok(())
}