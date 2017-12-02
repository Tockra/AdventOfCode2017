fn main() {

	// Ergebnissumme
	let mut sum = 0;

	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		// Jede Zeile wird durchlaufen und min und max werden berechnet
		let mut min: i32 = line.split_whitespace().nth(0).unwrap().parse().unwrap();
		let mut max: i32 = line.split_whitespace().nth(0).unwrap().parse().unwrap();
		
		for elem in line.split_whitespace() {

			let curr_val = elem.parse().unwrap();
			if curr_val > max {
				max = curr_val;
			}
			if curr_val < min {
				min = curr_val;
			}
		}
	
		sum += (max-min).abs();
	
	}

	println!("Die gesuchte Summe beträgt: {}",sum);

}


