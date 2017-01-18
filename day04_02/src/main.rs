use std::char;
// Datenstruktur für einen Raum
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

}

fn main() {
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		let mut room = Room::new();
		for elem in line.replace(']',"").split("[") {
			room.push(elem);
		} 
		
		// Rotiere alle Chars des Namens um die SektorID
		let encrypted_name = room.name.chars()
				.map(|x| 
					// Chars werden nur rotiert, wenn das char kein '-' ist
					if x != '-' {
						// Formel zum rotieren + unwrap, da nicht zwangsläufig ein Char von from_u32() berechnet werden muss
						// Die Formel für die Berechnung lässt allerdings ungültigen Wert, der einen Fehler verursachen könnte, zu!
						char::from_u32(('a' as u32 + ((x as u32 - 'a' as u32 + (room.sector_id % 26) as u32) % 26))).unwrap()
					} 
					else {x}
				).collect::<String>();

		// Es wird geprüft, ob der aktuell betrachtete Raum der North Pole raum ist. 
		if encrypted_name.contains("north") {
			println!("Raum gefunden: {} ID: {}",encrypted_name,room.sector_id);
		}
	
	}
	
}


