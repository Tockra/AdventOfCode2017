#[derive(Debug,Clone)]
enum InputElems {
	Elem,
	Modifier(i32,i32,i32)
}

fn main() {
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
		
		// Die Eingabe wird von hinten nach vorne betrachtet, dabei wird im Falle eines einfachen Chars 1 in die Lösung geschrieben und sonst dynamisch der entsprechende Lösungswert berechnet
		input.reverse();
		
		//Lösungsvektor A: Wird dynamisch befüllt
		let mut A: Vec<u64> = Vec::new();
		for _ in input.clone() {
			A.push(0);
		}
		
		for i in 0..input.len() {
			match input[i] {
				InputElems::Elem => A[i] = 1,
				InputElems::Modifier(a,b,_) => {
					let mut size = 0;
					let mut counter = 1;
					while size < a {
						A[i] += A[i-counter];
						A[i-counter] = 0;
						
						// Das nächste betrachtete Zeichen wird angeschaut.
						match input[i-counter] {
							InputElems::Elem => size +=1,
							InputElems::Modifier(_,_,n_size) => size += n_size
						}
						counter += 1;
					
					}
					A[i] *= b as u64;
				}
			}
		}
		
		// Anzahl der Zeichen im dekomprimierten String, entspricht der Summe aller verbleibenen Elemente in A
		let mut count = 0;
		
		for val in A {
			count += val;
		}
		// Ausgabe
		println!("Es wurden {} Zeichen ermittelt",count);
	}
	
	

}
