// Damit der Code nicht unnötig ausartet, habe ich das "and " in der Inputdatei am Ende jeder Zeile entfernt
// In meinem Fall, können die Abkürzungen der Chips und Generatoren als eindeutige Identifikation verwendet werden,
// dies ist in dieser Aufgabe allerdings nicht die Regel!
use std::fmt;

struct Chip {
	name: String,
	typ: String,
}

impl fmt::Display for Chip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.name.chars().nth(0).unwrap(), self.typ.chars().nth(0).unwrap())
    }
}

// Sinnvolle Ausgabe der Chips. Dabei bedeutet ein m an zweiter Stelle = Microship und ein 
impl Chip {
	fn new(name: &str, typ: &str) -> Chip {
		Chip{name:name.to_string(),typ:typ.to_string()}
	}
}

fn main() {
	
	// Array mit einem Vector für jede Etage (von 0-3)
	let mut floors: [Vec<Chip>;4] = [vec!(),vec!(),vec!(),vec!()];
	for i in 0..4 {
		floors[i] = vec![];
	}
	
	// Aktuelle Aufzugsposition
	let mut curr_elevator_pos: u8 = 0;
	
	// Anzahl der bisherigen Schritte
	let mut count = 0;
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines().enumerate() {
		// Nur die ersten 3 Etagen haben Inhalte
		if line.0 < 3 {
			// Iterator über die Zeile (getrennt durch ,)
			let mut iterator = line.1.split(",");
			
			// Spezialbehandlung des ersten Elements der Zeile, da dort trashiger Text steht
			let mut first = iterator.nth(0).unwrap().split(" ").skip(5);
			let name = first.nth(0).unwrap();
			let typ = first.nth(0).unwrap();
			floors[line.0].push(Chip::new(name,typ));
			
			for object in iterator {
				let mut iter = object.split(" ");
				let name = iter.nth(2).unwrap();
				let typ = iter.nth(0).unwrap();
				floors[line.0].push(Chip::new(name,typ));
			}
		}
	}
	
	// An dieser Stelle ist das Array floors initialisiert
	//while floors[3].len() != 10 {
		//count += action(&curr_elevator_pos,&floors);
	//}
	// Optische Ausgabe des Feldes
	for i in (0..4).rev() {
		print!("E{}",i);
		for chip in floors[i].iter() {
			print!(",{}",chip);
		}
		println!("");
	}
	
	
	// Ausgabe
	
}

//Meine aktuelle Vermutung ist, dass  folgende Strategie optimal ist:
// Wenn auf der aktuellen Ebene sowohl Generator, als auch Mikrochip ist, dann nehme beide und bringe sie so hoch wie möglich
// Wenn man auf der höchsten Ebene ist und sich nicht alle Elemente auf dieser ebene befinden, dann prüfe ob sich ein generator oder ein Chip auf der am nächsten liegenden Ebene befindet, die ein Element besitzt.
// ist es ein Chip, hole diesen
// ist es ein generator, dann nehme einen passenden Chip und hole den Generator
// Funktion führt den nächsten effizienten Schritte aus. Danach wird die Anzahl der gemachten Bewegeungen zurückgegeben
fn action(curr_eleator_pos: &u8,floors: &[Vec<Chip>]) -> u32 {
	let mut count = 0;
	// finde zwei Chips mit dem gleichen Typ
	//floors[curr_eleator_pos]
	count
}


