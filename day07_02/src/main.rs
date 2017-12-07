use std::collections::HashMap;

fn main() {
	//Name des Programms das auf einer linken Seite, aber auf keiner rechten Seite vorkommt.
	let mut root = "";

	// Kostenfunktion costs(p) für Programme:
	let mut costs = HashMap::new();
	
	let mut children = HashMap::new();

	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		let prog = line.split_whitespace().nth(0).unwrap();
		let weight = line.split_whitespace().nth(1).unwrap().replace("(","").replace(")","").parse::<i32>().unwrap();
		
		// Befüllt kosten
		costs.insert(prog,weight);

		// Befüllt Kinder
		if line.contains("->") {
			let c = line.split("-> ").nth(1).unwrap().split(", ").collect::<Vec<&str>>();
			children.insert(prog, c);
		}
		else {
			children.insert(prog,vec![]);
		}


		if count_program(&include_str!("../input.data").lines().collect(), prog) < 2 {
			root = prog;
		}
	}
	
	check_balance(root,&costs, &children);
	
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

fn check_balance(root: &str, costs: &HashMap<&str,i32>, adj: &HashMap<&str,Vec<&str>>) -> i32 {
	let mut child_weight: Vec<i32> = vec![];

	for c in adj.get(root).unwrap() {
		let result = check_balance(c,&costs, &adj);
		if result == -1 {
			return -1;
		}
		child_weight.push(check_balance(c,&costs, &adj));
	}

    // Bei dem Ergebnis muss man etwas raten, welches Gewicht das ist, dass am tiefsten im Baum ist. 
	// In der Regel muss das aber das kleinste sein.
	if !child_weight.iter().all(|&x| x == child_weight[0]) {
		for i in 0..child_weight.len() {

			// Element das anders ist als alle anderen
			if child_weight.iter().filter(|&&x| x == child_weight[i]).count() != child_weight.len()-1 {
				// Berechnet den Wert, um den das Gewicht verändert werden muss
				let diff = child_weight[i] - child_weight[(i+1)%child_weight.len()];

				// Gebe neues Soll-Gewicht für Programm aus
				println!("Das Gewicht von {} muss auf {} gesetzt werden. ", adj.get(root).unwrap()[i], costs.get(adj.get(root).unwrap()[i]).unwrap()-diff)
			}
		}
		// Fehlerfall (Hier hätte ich auch nen Optional nutzen können)
		return -1;
	}
	let mut sum = *costs.get(root).unwrap();

	sum += child_weight.iter().sum();

	sum
}