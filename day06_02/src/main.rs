use std::str::FromStr;
fn main() {
	
	let mut past: Vec<Vec<i32>> = vec![];
	let mut past_time: Vec<i32> = vec![];
	let mut count = 0;
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen.

	let mut mem: Vec<i32> = include_str!("../input.data").split_whitespace().map(|x| i32::from_str(x.trim()).unwrap()).collect();
	
	while !past.contains(&mem) {
		past.push(mem.clone());
		past_time.push(count);
		let max = get_max_index(&mem);
		let mut to_distr = mem[max];
		let mut curr = 1;
		mem[max] = 0;
		while to_distr > 0 {
			let len = mem.len();
			mem[(max+curr)%len] += 1;
			to_distr -= 1;
			curr += 1;
		}

		count += 1
	}
	
	// Ausgabe
	println!("Es wurden {} Schritte gemacht, bis eine bereits bekannte Konfiguration eintraf.",count-past_time[get_index_of_double(&past, &mem)]);
}

// Berechnet den Index des maximalen Elements
fn get_max_index(v: &Vec<i32>) -> usize {
	let mut max = 0;
	for i in 0..v.len() {
		if v[i] > v[max] {
			max = i;
		}
	}
	max
}

fn get_index_of_double(v: &Vec<Vec<i32>>, val: &Vec<i32>) -> usize {
	for i in 0..v.len() {
		if v[i] == *val {
			return i
		}
	}
	0
}

