enum Input_Elems {
	Elem(char),
	Modifier(i32,i32)
}

fn main() {
	
	// Anzahl der Zeichen im dekomprimierten String
	let mut count = 0;
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	// In dieser Inputdatei existiert allerdings nur eine Zeile, somit wird die Schleife nur einmal durchlaufen!
	for line in include_str!("../input.data").lines() {
		// Eingabe wird geparsed und in einen Vector mit Input_Elems mgewandelt
		let mut input: Vec<Input_Elems> = Vec::new();
		let tmp = line.split(|c| ['(', ')'].contains(&c));
		for elem in tmp.enumerate() {
			
		}
		// Es wird durch den String iteriert. Wenn in einem Block (axb) eingetreten wird, dann werden die nächsten a Zeichen b mal ins Ergebniss aufgenommen. 
		//Dann werden a Zeichen übersprungen und weiter gezählt
		for c in line.replace('(',"").replace(')',"").replace('x',",").chars() {
			if c != '(' {
				count += 1 ;
			}
			else {
				
			}
		}
	}
	
	
	// Ausgabe
	println!("Es wurden {} Zeichen ermittelt",count);
}
