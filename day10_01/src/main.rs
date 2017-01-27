#[derive(Clone,Copy)]
struct Bot {
	higher: u32,
	lower: u32,
	bot_higher: usize,
	output_higher: usize,
	
	bot_lower: usize,
	output_lower: usize
}

impl Bot {
	// Erzeugt Bot, der weder higher noch lower besitzt 
	fn new() -> Bot {
		Bot {higher: 0,lower: 0,bot_higher: 0, output_higher: 0, bot_lower: 0, output_lower: 0}
	}
	
	// Prüft ob der Bot einen higher Value besitzt
	fn has_higher(&self) -> bool {
		self.higher == 0
	}
	
	// Prüft ob der Bot einen lower Value besitzt
	fn has_lower(&self) -> bool {
		self.lower == 0
	}
	
	// Funktion um einem Bot einen Wert zu geben
	fn give_value(&mut self, value: u32) {
		if self.has_lower() {
			self.higher = value;
		}
		else {
			self.lower = value;
		}
	}
}

// In dieser Aufgabe wird nach der Anzahl der Wörter geparsed. Da die Sätze immer identisch sind, 
// wird bei "value 11 goes to bot 124" beispielsweise immer das 2. und 6. Wort genommen, um die Bot-NR
// und den Wert zu ermitteln!
fn main() {
	
	// Erzeugt einen Vektor, der alle Zeilen beinhaltet, die beim ersten durchlauf geskippt werden und erst beim zweiten 
	// durchlauf betrachtet werden
	let mut skipped = Vec::new();
	
	// Array in dem jeder Index der entsprechenden Botnummer entspricht
	let mut bots = [Bot::new();500];
	
	// Array für die Outputs, indem jeder Index dem entsprechenden Output entspricht
	let mut outputs = [0;500];
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
	
		
	
		

		
		if line.contains("value") {
			let mut tmp_it = line.split(" ");
			let value = tmp_it.nth(1).unwrap().parse::<u32>().unwrap();
			
			bots[tmp_it.nth(3).unwrap().parse::<usize>().unwrap()].give_value(value);
		}
		else {
			skipped.push(line);
		}
	}
	
	for line in skipped.into_iter() {
		// Für ein besseres verständnis werden einige Hilfsvariabeln definiert
		let mut tmp_it = line.split(" ");
		let bot_id = tmp_it.nth(1).unwrap().parse::<usize>().unwrap();
		let low_typ = tmp_it.nth(3).unwrap();
		let low_val = tmp_it.nth(0).unwrap().parse::<usize>().unwrap();
		let high_typ = tmp_it.nth(3).unwrap();
		let high_val = tmp_it.nth(0).unwrap().parse::<usize>().unwrap();
		
		let val_lower = bots[bot_id].lower as usize;
		if low_typ.contains("bot") {
			bots[low_val].bot_lower = val_lower;
		}
		else { // Output
			bots[low_val].output_lower = val_lower;
		}
		
		let val_higher = bots[bot_id].higher as usize;
		if high_typ.contains("bot") {
			bots[high_val].bot_higher = val_higher;
		}
		else { // Output
			bots[high_val].output_higher = val_higher;
		}
		
	}
	
	
	
	
	// Ausgabe
	//println!("Es wurden {} Zeichen ermittelt",count);
	
}
