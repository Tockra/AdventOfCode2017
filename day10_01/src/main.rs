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
		self.higher != 0
	}
	
	// Prüft ob der Bot einen lower Value besitzt
	fn has_lower(&self) -> bool {
		self.lower != 0
	}
	
	// Funktion um einem Bot einen Chip gibt. Dabei wird darauf geachtet, dass der Wert im lower immer kleiner als der 
	// im higher ist
	fn give_value(&mut self, value: u32) {
		if (self.lower == 61 && value == 17) || (self.lower == 17 && value == 61) {
			println!("Huhu");
		}
		if self.lower > value {
			self.higher = self.lower;
			self.lower = value;
		}
		else {
			self.lower = value;
		}
	}
	
	// Diese Funktion prüft, ob sowohl higher und lower voll sind. 
	// Ist dies der Fall, werden die Chips weitergegeben
	fn interact(&self, mut bots: &mut [Bot], mut outputs: &mut [u32]) {
		if self.has_higher() && self.has_lower() {
			println!("lower: {}, higher: {}",self.lower,self.higher);
			if self.bot_lower != 0 {
				let mut bot = bots[self.bot_lower];
				bot.give_value(self.lower);
				bot.interact(bots, outputs);
			}
			else {
				outputs[self.output_lower] = self.lower;
			}
			
			if self.bot_higher != 0 {
				let mut bot = bots[self.bot_higher];
				bot.give_value(self.higher);
				bot.interact(bots, outputs);
			}
			else {
				outputs[self.output_higher] = self.higher;
			}
		}
	}
}

// In dieser Aufgabe wird nach der Anzahl der Wörter geparsed. Da die Sätze immer identisch sind, 
// wird bei "value 11 goes to bot 124" beispielsweise immer das 2. und 6. Wort genommen, um die Bot-NR
// und den Wert zu ermitteln!
fn main() {
	
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
			// Für ein besseres verständnis werden einige Hilfsvariabeln definiert
			let mut iterator = line.split(" ");
			let bot_id = iterator.nth(1).unwrap().parse::<usize>().unwrap();
			let low_typ = iterator.nth(3).unwrap();
			let low_val = iterator.nth(0).unwrap().parse::<usize>().unwrap();
			let high_typ = iterator.nth(3).unwrap();
			let high_val = iterator.nth(0).unwrap().parse::<usize>().unwrap();
			
			let val_lower = bots[bot_id].lower as usize;
			
			// Dem Bot bot_id wird gesagt, an wen der lower-Chip geht
			if low_typ.contains("bot") {
				bots[low_val].bot_lower = val_lower;
			}
			else { // Output
				bots[low_val].output_lower = val_lower;
			}
			
			let val_higher = bots[bot_id].higher as usize;
			// Dem Bot bot_id wird gesagt, an wen der higher-Chip geht
			if high_typ.contains("bot") {
				bots[high_val].bot_higher = val_higher;
			}
			else { // Output
				bots[high_val].output_higher = val_higher;
			}
		}
	}
	
	// Durchlaufe alle Bots und lasse alle Bots mit 2 Werten ihre Instruktionen durchführen
	for i in 0..bots.len() {
	    let bot = bots[i].clone();
	    bot.interact(&mut bots,&mut outputs);
	}
	
	
	// Ausgabe
	//println!("Es wurden {} Zeichen ermittelt",count);
	
}
