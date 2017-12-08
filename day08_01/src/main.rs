
use std::collections::HashMap;

fn main() {
	let mut registers = HashMap::new();
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		// Datei wird geparset. Das ist hier etwas hässlig, aber es funktioniert ;) 
		
		let reg = line.split_whitespace().nth(0).unwrap();

		// Register initialisieren
		init(reg,&mut registers);
		
		// Condition vorhanden?
		if line.contains("if") {
			let operation = line.split_whitespace().nth(5).unwrap();
			let left = line.split_whitespace().nth(4).unwrap();
			let right = line.split_whitespace().nth(6).unwrap();
			init(left,&mut registers);
			init(right,&mut registers);
			
			let cond: bool = if operation == "==" {*registers.get(left).unwrap() == right.parse::<i32>().unwrap()}
				else if operation == "!="{*registers.get(left).unwrap() != right.parse::<i32>().unwrap()}
				else if operation == "<=" {*registers.get(left).unwrap() <= right.parse::<i32>().unwrap()} 
				else if operation == "<" {*registers.get(left).unwrap() < right.parse::<i32>().unwrap()}
				else if operation == ">=" {*registers.get(left).unwrap() >= right.parse::<i32>().unwrap()}
				else {*registers.get(left).unwrap() > right.parse::<i32>().unwrap()};
			
			if cond {
				let val = line.split_whitespace().nth(2).unwrap().parse::<i32>().unwrap();
				let mut n_v = 0;
				if line.contains("inc") {
					n_v = registers.get(&reg.to_string()).unwrap() + val;
				}
				else {
					n_v = registers.get(&reg.to_string()).unwrap() - val;
				}
				registers.remove(&reg.to_string());
				registers.insert(reg.to_string(),n_v);
			}

		}
		
	}
	
	// Variable zum zählen der leuchtenden Pixel. Das Zählen kann mit der Ausgabe verbunden werden
	let mut max = *registers.iter().nth(0).unwrap().1;
	
	// Ausgabe
	for (_,e) in registers {
		if e > max {
			max = e;
		}
	}

	println!("Maximaler Registerwert {}",max);
	
}

fn init(register: &str, registers: &mut HashMap<String,i32> ) {
	if registers.get(register).is_none()  {
			registers.insert(register.to_string(),0);
	}
}
