
fn main() {	
	let mut list: Vec<u32> = vec![];

	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines()  {
		// In dieser Aufgabe existiert nur eine Zeile, somit ist diese Schleife nicht zwingend notwendig. Sie wird aber für die folgenden
		// Aufgaben beibehalten.
		
		// Überführt die Line bestehend aus einer zusammenhängenden Zahl in einen Vektor, bei dem jeder Eintrag einem Zeichen/Digit der Zahl entspricht.
		list = line.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();

	}


	// Summe die als Ergebnis dient
	let mut sum = 0;

	// Durchläuft jede zahl und prüft, ob diese ihrem Vorgänger entspricht. Falls dies der Fall ist, wird die Summe erhöht
	for i in 0..list.len() {
		// Element mit dem len()/2 Elementen weiter forne stehenden Element vergleichen, wobei WrapAround durch Modulo erreicht wird
			if list[i] == list[(i+list.len()/2)% list.len()] {
				sum += list[i];
			}
	}

	println!("Das Ergebnis dieser Aufgabe entspricht: {}",sum);


}

