
struct Passphrase<'a> {
	name: Vec<&'a str>,
}
impl<'a> Passphrase<'a> {
	
	fn new(name: Vec<&str>) -> Passphrase {
		Passphrase {name: name}
	}
	
	// Diese Funktion prüft die Passphrase gültig ist
	// Aktuelle Version ist völlig unsauber programmiert und sollte so nicht verwendet werden...
	fn is_passphrase(&self) -> bool {
		// Vergleiche alle mit allen
		for i in 0..self.name.len() {
			for j in i+1..self.name.len() {
				if are_anagrams(self.name[i],self.name[j]) {
					return false
				}
			}
		}
		true
	}
}

// Vergleicht zwei Strings, ob diese ein Anagram sind
fn are_anagrams(w1: &str, w2: &str) -> bool {
	if w1.len() == w2.len() {
		for c in w1.chars() {
			if !w2.contains(c) {
				return false
			}
		}
		return true
	}
	false
}

fn main() {
	// Summe der bisherigen gültigen Passphrases
	let mut sum = 0;

	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		let pp = Passphrase::new(line.split_whitespace().collect()); // Jede Zeile wird in die Datenstruktur überführt

		if pp.is_passphrase() { // Einfacher gültigkeitscheck
			sum +=1;
		}
		
	}
	
	println!("Es gibt {} ungültige Passphrases", sum);
	

}


