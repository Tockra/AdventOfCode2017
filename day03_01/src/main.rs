// Datenstruktur zur Beschreibung der 4 Schritte die gemacht werden dürfen
#[derive(Clone, Copy, Debug)]
enum Step {
    UP(i32),
	DOWN(i32),
	RIGHT(i32),
	LEFT(i32),
}

fn get_next_direction(d: &Step, val: i32) -> Step {
	match *d {
		Step::UP(_) => Step::LEFT(val),
		Step::DOWN(_) => Step::RIGHT(val),
		Step::RIGHT(_) => Step::UP(val),
		Step::LEFT(_) => Step::DOWN(val)
	}
}

fn set_curr_direction(d: &Step, val: i32) -> Step {
	match *d {
		Step::UP(_) => Step::UP(val),
		Step::DOWN(_) => Step::DOWN(val),
		Step::RIGHT(_) => Step::RIGHT(val),
		Step::LEFT(_) => Step::LEFT(val)
	}
}

fn get_spiral_to(max_val: i32) -> Vec<Step> {
	let mut spiral: Vec<Step> = vec![];

	let mut curr_dis = 1;
	let mut curr_steps = 0;
	let mut curr_direction = Step::RIGHT(1);
	for i in 0..max_val {

		curr_steps +=1;	
		// Spirale wird abgelaufen. Falls genug Schritte in eine Richtung gegangen wurden (2xcurr_dis)
		if 2*curr_dis == curr_steps {
			spiral.push(curr_direction.clone());

			// Richtungswechsel
			curr_dis += 1;
			curr_direction = get_next_direction(&curr_direction,curr_dis);
			curr_steps = 0;
		}
		else if curr_dis == curr_steps {
			spiral.push(curr_direction.clone());
			curr_direction = get_next_direction(&curr_direction,curr_dis);
		}
		else if i == max_val-1 {
			spiral.push(set_curr_direction(&curr_direction,curr_steps-1));
		}
		
		
	}
	spiral
}

fn main() {

	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		// Wie bereits in der ersten Aufgabe existiert lediglich eine Zeile zum Einlesen. Die Schleife wird auf Grund der Ähnlichkeit zu den anderne Aufgaben denoch nicht angepasst!
		let spiral = get_spiral_to(line.parse::<i32>().unwrap());
		
		let sum_right: i32 = spiral.iter().map(|x| match *x {Step::RIGHT(x) => x, _=>0,}).sum();
		let sum_left: i32 = spiral.iter().map(|x| match *x {Step::LEFT(x) => x, _=>0,}).sum();
		let sum_up: i32 = spiral.iter().map(|x| match *x {Step::UP(x) => x, _=>0,}).sum(); 
		let sum_down: i32 = spiral.iter().map(|x| match *x {Step::DOWN(x) => x, _=>0,}).sum();
		
		// Manhattendistanz
		let distance = (sum_left-sum_right).abs() + (sum_up-sum_down).abs();
		println!("Die gesuchte Distanz entspricht {}.",distance);
		
	}
	


}


