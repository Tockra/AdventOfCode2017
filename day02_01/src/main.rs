fn main() {
	
	// Feld, dass das Keypad darstellt, -1 Felder sind nicht betretbare Felder
	let mut keypad = [[-1;11];11];
	
	// Aktuelle Position im Keypad
	let mut x: usize = 5;
	let mut y: usize = 5;
	
	//Array für Aufgabe 2_01 definieren, so dass die inneren Symbole den Tasten des Keypads entsprechen
	keypad[4][4] = 1; keypad[5][4] = 2; keypad[6][4] = 3;
	keypad[4][5] = 4; keypad[5][5] = 5; keypad[6][5] = 6;
	keypad[4][6] = 7; keypad[5][6] = 8; keypad[6][6] = 9;
	

	
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		
		// Jede Zeile gibt eine Wegbeschreibung (char für char) zum Zielkey aus. Nach abarbeitung der folgenden Schleife 
		// entsprechen x und y den Koordinaten des Zielkeys.
		for elem in line.chars() {
			match elem {
				'R' if keypad[x+1][y] != -1 => x +=1,
				'L' if keypad[x-1][y] != -1 => x -=1,
				'U' if keypad[x][y-1] != -1 => y -=1,
				'D' if keypad[x][y+1] != -1 => y +=1,
				_ => ()
				
			}

		}
		
		// Zielkey wird ausgegeben
		println!("Aktuelle Zahl: {}",keypad[x][y]);
		
	}

}


