// Datenstruktur zur simplen Darstellung eines Dreiecks
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
	
	// Nach der gegebenen Bedingung wird geprüft, ob das aktuelle Dreieck ein echtes Dreieck sein kann
	fn is_triangle(&self) -> bool {
		self.x + self.y > self.z && self.x + self.z > self.y && self.y + self.z > self.x
	}
}


fn main() {
	
	// Anzahl der gefundenen möglichen Dreiecke
	let mut count = 0;
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		// Neues Dreieck wird für jede Zeile angelegt
		let mut dreieck = Triangle::new();
		
		// Jede Zeile beinhaltet ein Dreieck. Die einzelnen Koordinaten sind mit Leerzeichen getrennt
		for elem in line.split_whitespace() {
			//Koordinate wird zu u16 geparsed und in das aktuelle Dreieck gepusht
			dreieck.fill(elem.parse().unwrap());
		}
		
		// Es wird geprüft, ob das Dreieck der aktuellen Zeile wirklich ein Dreieck sein kann.
		if dreieck.is_triangle()  {
			count += 1;
		}
		
		
	}
	
	// Ausgabe der Anzahl der möglichen Dreiecke
	println!("Mögliche Dreiecke: {}",count);

}


