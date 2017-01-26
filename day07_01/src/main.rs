fn main() {
	//Anzahl der Abbanyme
	let mut count = 0;
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		let mut is_ip = false;
		
		// Lösung ginge auch noch funktionaler, ist dann als Außenstehender kaum noch nachvollziehbar.
		for part in line.split(|c| ['[', ']'].contains(&c)).map(contains_abba).enumerate() {
			// Prüfen ob wir uns im Hypernet-Feld befinden
			if part.0%2 == 1 && part.1 {
				is_ip = false;
				break;
			}
			if part.0%2 == 0 && part.1 {
				is_ip = true;
			}
		}
		if is_ip {
			count += 1;
		}
	}
	
	// Ausgabe
	println!("Es wurden {} gültige Addressen gefunden!",count);
}

// Prüft ob addr_part ein Abbanym beinhaltet. Es wird davon ausgegangen, dass 1 char = 1 byte
fn contains_abba(addr_part: &str) -> bool {
	for limit_left in 0..addr_part.len()-3 {
		// Es wird davon ausgegenangen, dass nur normale nicht Unicode-Buchstaben in hyper_net stehen
		let slice = &addr_part[limit_left..limit_left+4];
		if is_abba(slice) {
			return true;
		}
	}
	false
}

// Funktion prüft, ob ein übergegebener 4 elementiger String ein Abbanym ist
// Es wird sichergestellt das kein unwrap jemals realistisch panict
fn is_abba(candidate: &str) -> bool {
	candidate.chars().nth(0).unwrap() ==candidate.chars().nth(3).unwrap()  && candidate.chars().nth(1).unwrap()  == candidate.chars().nth(2).unwrap()  && candidate.chars().nth(0).unwrap()  != candidate.chars().nth(1).unwrap() 
}

