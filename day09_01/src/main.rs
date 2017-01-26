#[derive(Debug)]
enum InputElems {
	Elem,
	Modifier(i32,i32,i32)
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
		let mut input: Vec<InputElems> = Vec::new();
		let tmp = line.split(|c| ['(', ')'].contains(&c));
		for elem in tmp.enumerate() {
			if elem.0 % 2 == 1 {
				let mut splitted_modifier = elem.1.split("x");
				let a = splitted_modifier.nth(0).unwrap().parse::<i32>().unwrap();
				let b = splitted_modifier.nth(0).unwrap().parse::<i32>().unwrap();			
				input.push(InputElems::Modifier(a,b,elem.1.len() as i32 +2));
			}
			else {
				for _ in 0..elem.1.len() {
					input.push(InputElems::Elem);
				}
			}
		}
	
		// Nun kann über die Eingabe iteriert werden und zählen.
		let mut i = 0;
		while i<input.len() {
			match input[i] {
				InputElems::Elem => count += 1,
				InputElems::Modifier(a,b,_) => {
					count += a*b;
					let mut size = a;
					
					// Hier wird i entsprechend der size des aktuellen Bereichs angepasst
					while size >0 {
						i += 1;
						match input[i] {
							InputElems::Elem => size -= 1,
							InputElems::Modifier(_,_,n_size) => size -= n_size
						}
					}
				}
			};
			i += 1;
		}
		
	}
	
	
	// Ausgabe
	println!("Es wurden {} Zeichen ermittelt",count);
}
