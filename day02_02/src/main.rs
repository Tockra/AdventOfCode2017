fn main() {

	// Ergebnissumme
	let mut sum = 0;
	let mut is = 1;
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		let arr: Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

		// Betrachte für jedes Element arr[i] alle folgenden Elemente und prüfe ob diese in irgendeiner Form ganzzahlig teilbar sind.
		for i in 0..arr.len() {
			for j in i+1..arr.len() {
				if arr[i] % arr[j] == 0 {
					sum += arr[i]/arr[j];
				}
				else if arr[j] % arr[i] == 0 {
					sum += arr[j] / arr[i];
				}
			}
		}
		 is += 1;
	
	}

	println!("Die gesuchte Summe beträgt: {}",sum);

}


