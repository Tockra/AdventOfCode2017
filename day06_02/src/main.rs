fn main() {
	// Array für die Ausgabe
	let mut input: [Vec<char>;8] = [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		for i in 0..line.len() {
			input[i].push(line.chars().nth(i).unwrap());
		}
	}
	
	let mut output = ['-';8];
	
	// Im Array output wird für jeden Index i, der häufigste Charakter im entsprechenden Vektor aus dem input Array bestimmt
	for i in 0..8 {
		// Im Fold muss eine kleine Anpassung für Aufgabe 02 gemacht werden
		// Es muss dafür gesorgt werden, dass der Char ' ' niemals Lösungskandidat ist.
		output[i] = input[i].iter().fold(' ',
			|acc, &x| 
			if input[i].iter().filter(|y| *y == &x).count() < input[i].iter().filter(|y| *y == &acc).count() || acc == ' ' {
				x
			} else {
				acc
			});
	}
	
	// Ausgabe
	println!("{}",output.iter().cloned().collect::<String>());
}


