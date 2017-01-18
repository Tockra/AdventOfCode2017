fn main() {
	
	// Feld, dass das Keypad darstellt, -1 Felder sind nicht betretbare Felder
	let mut keypad = [[-1;11];11];
	
	// Aktuelle Position im Keypad
	let mut x: usize = 3;
	let mut y: usize = 5;
	
	// Das Array wird entsprechend der Aufgabenstellung angepasst. Der restliche Quellcode beleibt im Vergleich zu day02_01 unverändert
									     keypad[5][3] = 1;
					  keypad[4][4] = 2;  keypad[5][4] = 3;  keypad[6][4] = 4;
	keypad[3][5] = 5; keypad[4][5] = 6;  keypad[5][5] = 7;  keypad[6][5] = 8; keypad[7][5] = 9;
					  keypad[4][6] = 10; keypad[5][6] = 11; keypad[6][6] = 12;
									     keypad[5][7] = 13;
	

	
	
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
		// Achtung die Hexadezimalzahlen A, B, C und D werden entsprechend ihrer dezimaldarstellung ausgegeben 10, 11, 12, 13
		println!("Aktuelle Zahl: {}",keypad[x][y]);
		
	}

}


