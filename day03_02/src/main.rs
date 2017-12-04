// Datenstruktur zur Beschreibung der 4 Schritte die gemacht werden dürfen
#[derive(Clone, Copy, Debug)]
enum Step {
    UP,
	DOWN,
	RIGHT,
	LEFT,
}

fn get_next_direction(d: &Step) -> Step {
	match *d {
		Step::UP => Step::LEFT,
		Step::DOWN => Step::RIGHT,
		Step::RIGHT => Step::UP,
		Step::LEFT => Step::DOWN
	}
}

fn get_next_coords(curr_dir: &Step, x: usize, y: usize) -> (usize,usize) {
	match *curr_dir {
		Step::UP => (x,y-1),
		Step::DOWN => (x,y+1),
		Step::RIGHT => (x+1,y),
		Step::LEFT => (x-1,y),
	}
}

// Ränder des Arrays werden ignoriert. Gibt es keine gültige Lösung, crasht das Programm :p
fn generate_spiral(max_val: i32) -> i32 {
	// Hardcoded 20x20 Spirale
	let mut spiral = [[0; 20]; 20];

	let mut curr_x: usize = 10;
	let mut curr_y: usize = 10;

	let mut curr_dis = 1;
	let mut curr_direction = Step::RIGHT;
	let mut twice = false;
	spiral[10][10] = 1;
	for i in 1..max_val {
		let (x,y) = get_next_coords(&curr_direction,curr_x,curr_y);
		curr_x = x;
		curr_y = y;
		
		// Das Neue Feld entspricht der Summe aller Nachbarn. Da alle noch nicht initialisierten Nachbarn = 0 sind, muss hier keine Fallunterscheidung gemacht werden.
		spiral[curr_x][curr_y] = spiral[curr_x-1][curr_y] + spiral[curr_x][curr_y-1] + spiral[curr_x][curr_y+1] + spiral[curr_x+1][curr_y] + spiral[curr_x+1][curr_y+1] + spiral[curr_x-1][curr_y-1] + spiral[curr_x-1][curr_y+1]+ spiral[curr_x+1][curr_y-1];      
		
		if spiral[curr_x][curr_y] > max_val {
			return spiral[curr_x][curr_y]
		}
		// Zweimal bereits curr_dis erreicht. Somit muss die Richtung gewechselt werden
		if i%curr_dis == 0 && twice {

			// Richtungswechsel
			curr_dis += 1;
			curr_direction = get_next_direction(&curr_direction);
			twice = false;
		}
		else if i%curr_dis == 0 {
			curr_direction = get_next_direction(&curr_direction);
			twice = true;
		}
		
		
	}
	0
}

fn main() {

	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		// Wie bereits in der ersten Aufgabe existiert lediglich eine Zeile zum Einlesen. Die Schleife wird auf Grund der Ähnlichkeit zu den anderne Aufgaben denoch nicht angepasst!
		let spiral = generate_spiral(line.parse::<i32>().unwrap());
		
		println!("Die erste zahl größer als {} ist {}.",line,spiral);
		
	}
	


}


