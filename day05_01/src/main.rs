
use std::str::FromStr;
fn main() {
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	let mut lines: Vec<i32> = include_str!("../input.data").lines().map(|x| i32::from_str(x.trim()).unwrap()).collect();
	
	// Aktuelle Zeile
	let mut curr_line: usize = 0;

	// Anzahl der möglichen Sprünge
	let mut count = 0;

	// Schleife läuft bis Speicherzugriffsfehler...
	loop {
		let old_line = curr_line;
		curr_line = (curr_line as i32 + lines[curr_line]) as usize;
		count += 1;
		lines[old_line] += 1;

		println!("Letzte Position {} Aktuelle Position {} Sprünge: {} ", old_line, curr_line,count);
	}
	
}


