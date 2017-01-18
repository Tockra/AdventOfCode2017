// Datenstruktur zur simplen Darstellung eines Dreiecks
#[derive(Copy,Clone)]
struct Triangle {
	x: u16,
	y: u16,
	z: u16
}

impl Triangle {
	fn new() -> Triangle {
        Triangle { x: 0, y: 0, z: 0, }
    }
    
    // Funktion, mit deren Hilfe ein Dreieck Seite für Seite definiert werden kann
	fn fill(&mut self, val: u16) {
		if self.y != 0  {
			self.z = val;
		}
		else if self.x != 0 {
			self.y = val;
		}
		else {
			self.x = val;
		}
	}
	
	fn is_complete(&self) -> bool {
		self.x != 0 && self.y != 0 && self.z != 0
	}
	
	// Nach der gegebenen Bedingung wird geprüft, ob das aktuelle Dreieck ein echtes Dreieck sein kann
	fn is_triangle(&self) -> bool {
		self.x + self.y > self.z && self.x + self.z > self.y && self.y + self.z > self.x
	}
}


fn main() {
	// Anzahl der gefundenen möglichen Dreiecke
	let mut count = 0;
	
	
	// Es werden immer 3 Dreiecke aufeinmal eingelesen
	let mut dreiecke = [Triangle::new();3];
	
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		// Zähler für das 1., 2. und 3. Dreieck
		let mut i = 0;
		
		// Jede Zeile beinhaltet ein Dreieck. Die einzelnen Koordinaten sind mit Leerzeichen getrennt
		for elem in line.split_whitespace() {
			//Koordinate wird zu u16 geparsed und in das aktuelle Dreieck gepusht
			dreiecke[i].fill(elem.parse().unwrap());
			i += 1;
		}

		
		// Wenn das 0. Dreieck komplett ist, dann sind an dieser Stelle auch das 2. und 3. Dreieck komplett, da immer 3 Koordinaten pro Zeile eingelesen werden.
		if dreiecke[0].is_complete() {
			
			// Prüfe ob das 1., 2. und 3. Dreieck echte Dreiecke sind
			for dr in dreiecke.iter() {
				if dr.is_triangle() {
					count += 1;
				}
			}
			
			// Die drei aktuell betrachteten Dreiecke werden zurückgesetzt
			dreiecke = [Triangle::new();3];
		}
		
	}
	
	// Ausgabe der Anzahl der möglichen Dreiecke
	println!("Mögliche Dreiecke: {}",count);

}


