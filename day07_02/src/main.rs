fn main() {
	//Anzahl der Abbanyme
	let mut count = 0;
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		
		let (even,odd) : (Vec<(usize,&str)>,Vec<(usize,&str)>) = line.split(|c| ['[', ']'].contains(&c)).enumerate().partition(|&n| n.0 % 2 == 0);
		
		// Prüft, ob es irgendwo im super_net-Teil ein Aba mit zugehörigem Bab aus dem hyper_net Teil gibt
		if even.into_iter().any(|x| contains_aba(x.1,&odd)) {
			count += 1 ;
		}
		
	}
	
	// Ausgabe
	println!("Es wurden {} gültige Addressen gefunden!",count);
}

// Prüft ob addr_part ein Aba mit zugehörigen Bab beinhaltet. Es wird davon ausgegangen, dass 1 char = 1 byte
fn contains_aba(addr_part: &str, hyper_net: &Vec<(usize,&str)>) -> bool {
	for limit_left in 0..addr_part.len()-2 {
		
		// Es wird davon ausgegenangen, dass nur normale nicht Unicode-Buchstaben in addr_teil stehen
		let slice = &addr_part[limit_left..limit_left+3];
		
		// Wenn ein aba gefunden wird, dann wird geschaut, ob in irgendeinem zugehörigen hyper_net teil ein Bab existiert
		if is_aba(slice) && contains_bab(slice,hyper_net) {
			return true;
		}
	}
	false
}

fn contains_bab(aba: &str, hyper_net: &Vec<(usize,&str)>) -> bool {
	// bab Zeichenkette wird erzeugt
	let mut bab = String::new();
	bab.push(aba.chars().nth(1).unwrap());
	bab.push(aba.chars().nth(0).unwrap());
	bab.push(aba.chars().nth(1).unwrap());
	
	// Es wird geprüft, ob irgendein hyper_net Teil die Bab Zeichenkette beinhaltet
	hyper_net.iter().any(|x| x.1.to_string().contains(&bab))
}
// Funktion prüft, ob ein übergegebener 3 elementiger String ein Abbanym ist
// Es wird sichergestellt das kein unwrap jemals realistisch panict
fn is_aba(candidate: &str) -> bool {
	candidate.chars().nth(0).unwrap() ==candidate.chars().nth(2).unwrap()  && candidate.chars().nth(0).unwrap()  != candidate.chars().nth(1).unwrap() 
}

