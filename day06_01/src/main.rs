fn main() {
	
	let past: Vec<Vec<i32>> = vec![];
	let mut count = 0;
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen.

	let mut mem = include_str!("../input.data").split_whitespace();
	
	while !past.contains(mem) {
		past.push(mem);

		let max = get_max_index(&mem);
		let mut to_distr = mem[max];
		let mut curr = 1;
		mem[max] = 0;
		while to_distr > 0 {
			mem[(max+curr)%mem.len()] += 1;
			to_distr -= 1;
			curr += 1;
		}

		count += 1
	}
		
	// Ausgabe
	println!("Es wurden {} Schritte gemacht, bis eine bereits bekannte Konfiguration eintraf.",count);
}

// Berechnet den Index des maximalen Elements
fn get_max_index(v: Vec<i32>) -> usize {
	let mut max = 0;
	for i in 0..v.len() {
		if v[i] > v[max] {
			max = i;
		}
	}
	max
}


