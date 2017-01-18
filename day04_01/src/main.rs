use std::cmp::Ordering;

struct Room {
	name: String,
	checksum: String,
	sector_id: i32
}
impl Room {
	
	fn new() -> Room {
		Room {name: "".to_string(), checksum: "".to_string(),sector_id: -1}
	}
	
	// Diese Funktion wird zum Erzeugen eines Room Elements verwendet
	fn push(&mut self,wort:&str ) {
		if self.name == "" {
			// Aus dem Namen wird gleich die Sektor-ID genommen und in eine eigene Variabel geschrieben
			self.name = wort[0..wort.to_string().rfind("-").unwrap()].to_string();
			self.sector_id = wort[wort.to_string().rfind("-").unwrap()+1..].parse().unwrap();
		}
		else {
			self.checksum = wort.to_string();
		}
	}
	// Diese Funktion prüft die Raumeigenschaft des aktuellen Raumes
	fn is_room(&self) -> bool {
		// Baue einen Vektor auf, der alle Buchstaben die im Name vorkommen in sortierter Reihenfolge beinhaltet
		let mut sorted_chars:Vec<(char,usize)> = Vec::new();
		for c in self.name.replace('-',"").chars() {
			// Falls der aktuelle Buchstabe c noch nicht im Vektor drin ist ..
			if sorted_chars.iter().filter(|x| x.0 == c).count() == 0 {
				// .. wird er hinzugefügt. Dabei gilt (c,<häufigkeit>)
				sorted_chars.push((c,self.name.chars().filter(|x| x == &c).count()));
			}
		}
		// sorted_chars ist bisher unsortiert, wird aber nun nach der Häufigkeit (zweite Tupelkomponente) sortiert
		sorted_chars.sort_by(compare);
		
		// Es wird überprüft, ob der n-te Char in der Checksumme auch der n-te char in sorted_chars ist
		for i in 0..self.checksum.len() {
			if sorted_chars[i].0 != self.checksum.chars().nth(i).unwrap() {
				return false;
			}
		}
		true
	}
}

// Funktion zum Vergleichen zweier Tupel. Falls die Häufigkeit gleich ist, wird nach den Buchstaben sortiert,
// sonst nach der Häufigkeit
fn compare(x:&(char,usize),y:&(char,usize)) -> Ordering {
	if x.1 == y.1 {
		return x.0.cmp(&(y.0));
	}
	y.1.cmp(&(x.1))
}

fn main() {
	// Summe der bisherigen Sektor-IDs der gültigen Räume
	let mut sum = 0;
		
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		let mut room = Room::new();
		for elem in line.replace(']',"").split("[") {
			room.push(elem);
		} 
		if room.is_room() {
			sum += room.sector_id;
		}
	}
	
	println!("Die Summe der Sektor-IDs beträgt: {}", sum)
	

}


