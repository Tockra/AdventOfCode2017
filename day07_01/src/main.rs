fn main() {
	//Name des Programms das auf einer linken Seite, aber auf keiner rechten Seite vorkommt.
	let mut name = "";
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		let prog = line.split_whitespace().nth(0).unwrap();

		if count_program(&include_str!("../input.data").lines().collect(), prog) < 2 {
			name = prog;
		}
	}
	
	// Ausgabe
	println!("Es wurden {} gültiges Programm gefunden!",name);
}

fn count_program(rest: &Vec<&str> , prog: &str) -> i32 {
	let mut count = 0;
	for progg in rest {
		if progg.to_string().contains(prog) {
			count += 1;
		}
	}
	count
}