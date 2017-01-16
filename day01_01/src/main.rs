use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

enum Direction {
	North,
	East,
	South,
	West,
}

impl Direction {
	// Diese Funktion bestimmt ausgehend von der alten Ausrichtung und der aktuellen Bewegung, die neue Ausrichtung
	fn compute_direction(self,turn:char) -> Direction {
		if turn == 'R' {
			match self {
				Direction::North => Direction::East,
				Direction::East => Direction::South,
				Direction::South => Direction::West,
				Direction::West => Direction::North,
			}
		}
		else {
			match self {
				Direction::North =>  Direction::West,
				Direction::East =>  Direction::North,
				Direction::South =>  Direction::East,
				Direction::West =>  Direction::South,
			}
		}
	}
}


fn main() {
	//Eine Fehlerbehandlung ist in diesem Projekt aufgrund der Größe nicht unbedingt nötig!
	//Input File + Reader:
	let reader = BufReader::new(File::open("input.data").unwrap());
	
	// Koordinaten
	let mut x: i16 = 0;
	let mut y: i16 = 0;
	let mut direction = Direction::North;
	
	
	//read line by line
	for line in reader.lines() {
		// In dieser Aufgabe existiert nur eine Zeile, somit ist diese Schleife nicht zwingend notwendig. Sie wird aber für die folgenden
		// Aufgaben beibehalten.
		
		for elem in line.unwrap().replace(' ', "").split(",") {
			// Die Richtiung in die gedreht werden soll
			let turn:char = elem.chars().nth(0).unwrap();
			// Die Distanz die gegangen werden soll. Dafür reicht ein i16 !
			let distance:i16 = (&elem[1..]).parse().unwrap();
			
			// Neue Richtung wird bestimmt:
			direction = direction.compute_direction(turn);
			
			// Neue Position wird bestimmt:
			compute_new_pos(&mut x,&mut y,distance,&direction);
			
		}
		
		//Finale Ausgabe
		println!("X: {}, Y: {} Distanz: {}",x,y,x.abs()+y.abs());
	}

}

// Diese Funktion erhält 2 veränderbare Referenzen auf die X und Y Koordinate, die Distanz die gelaufen werden soll und eine Referenz auf 
// die aktuelle Richtung. Anschließend besitzen die x- und y-Veriabeln die richtigen Werte!
fn compute_new_pos(x: &mut i16,y: &mut i16,distance: i16, d: &Direction) {
	match *d {
		Direction::North => (*y += distance),
		Direction::East => (*x += distance),
		Direction::South => (*y -= distance),
		Direction::West => (*x -= distance),
	};
}

