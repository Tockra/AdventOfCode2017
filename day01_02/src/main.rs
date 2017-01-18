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

#[derive(Debug)]
struct Point {
	x:i16,
	y:i16
}



impl PartialEq for Point {
	fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}


fn main() {	
	// Koordinaten
	let mut x: i16 = 0;
	let mut y: i16 = 0;
	let mut direction = Direction::North;
	
	// Liste der ersten n besuchten Punkte. Dabei ist der n-te Punkt der erste, der doppelt besucht wurde. Es gilt immer size <= n
	let mut points: Vec<Point> = Vec::new();
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines()  {
		// In dieser Aufgabe existiert nur eine Zeile, somit ist diese Schleife nicht zwingend notwendig. Sie wird aber für die folgenden
		// Aufgaben beibehalten.
		
		for elem in line.replace(' ', "").split(",") {
			// Die Richtiung in die gedreht werden soll
			let turn:char = elem.chars().nth(0).unwrap();
			// Die Distanz die gegangen werden soll. Dafür reicht ein i16 !
			let distance:i16 = (&elem[1..]).parse().unwrap();
			
			// Neue Richtung wird bestimmt:
			direction = direction.compute_direction(turn);
			
			// Neue Position wird bestimmt:
			match compute_new_pos(&mut x,&mut y,distance,&direction,&mut points) {
				Some(x) => {
					println!("Erster Doppelter Punkt: {:?} , Distanz: {}",x, x.x.abs()+x.y.abs());
					
					//Weitere Programmausführung ist nicht nötig, Ergebnis wurde gefunden
					return;
				},
				None => {} // Diese Instruktion hat keinen Punkt geliefert, an dem man bereits war.
			}
			
		}
		
		//Finale Ausgabe
		println!("X: {}, Y: {} Distanz: {}",x,y,x.abs()+y.abs());
	}

}

// Diese Funktion erhält 2 veränderbare Referenzen auf die X und Y Koordinate, die Distanz die gelaufen werden soll und eine Referenz auf 
// die aktuelle Richtung. Anschließend besitzen die x- und y-Veriabeln die richtigen Werte! Außerdem wird eine Referenz auf einen Vektor mit bereits besuchten Punkten
// übergeben.
fn compute_new_pos(x: &mut i16,y: &mut i16,distance: i16, d: &Direction, points: &mut Vec<Point>) -> Option<Point> {
	for add in 0..distance {
		match *d {
			Direction::North => {
				*y += 1;
			},
			Direction::East => {
				*x += 1
			},
			Direction::South => {
				*y -= 1
			},
			Direction::West => {
				*x -= 1
			},
		}
		let point = Point{x:*x,y:*y};
		if points.contains(&point) {
			return Some(point);
		}
		else {
			points.push(point);
		}
	}
	None
}

