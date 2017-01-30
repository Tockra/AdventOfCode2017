#[derive(Clone,Copy)]
struct Bot {
	id: usize,
	higher: u32,
	lower: u32,
	bot_higher: Option<usize>,
	output_higher: usize,
	
	bot_lower: Option<usize>,
	output_lower: usize
}

impl Bot {
	// Erzeugt Bot, der weder higher noch lower besitzt 
	fn new() -> Bot {
		Bot {higher: 0,lower: 0,bot_higher: None, output_higher: 0, bot_lower: None, output_lower: 0, id: 0}
	}
	
	// Prüft ob der Bot einen higher Value besitzt
	fn has_higher(&self) -> bool {
		self.higher != 0
	}
	
	fn give_id(&mut self,id: usize) {
		self.id = id;
	}
	
	fn reset(&mut self) {
		self.higher = 0;
		self.lower = 0;
	}
	
	// Prüft ob der Bot einen lower Value besitzt
	fn has_lower(&self) -> bool {
		self.lower != 0
	}
	
	// Funktion um einem Bot einen Chip gibt. Dabei wird darauf geachtet, dass der Wert im lower immer kleiner als der 
	// im higher ist
	fn give_value(&mut self, value: u32) {
		if (self.lower == 61 && value == 17) || (self.lower == 17 && value == 61) {
			println!("Bot {} vergleicht die Chips",self.id);
		}
		
		if !self.has_lower() {
			self.lower = value;
		}
		else if self.lower > value {
			self.higher = self.lower;
			self.lower = value;
		}
		else {
			self.higher = value;
		}
	}
	
	// Diese Funktion prüft, ob sowohl higher und lower voll sind. 
	// Ist dies der Fall, werden die Chips weitergegeben
	fn interact(&mut self, mut bots: &mut [Bot], mut outputs: &mut [u32]) {
		if self.has_higher() && self.has_lower() {
			let value_lower = self.lower;
			let value_higher = self.higher;
			bots[self.id].reset();
			
			match self.bot_lower {
				Some(id) => {
					bots[id].give_value(value_lower);
					let mut bot = bots[id];
					bot.interact(bots, outputs);
				}
				_ => {
					outputs[self.output_lower] = value_lower;
				}
			}
			
			match self.bot_higher {
				Some(id) => {
					bots[id].give_value(value_higher);
					let mut bot = bots[id];
					bot.interact(bots, outputs);
				}
				_ => {
					outputs[self.output_higher] = value_higher;
				}
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
	
	// Jedem Bot wird seine ID zugewiesen
	for i in 0..bots.len() {
		bots[i].give_id(i);
	}
	
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
			
			
			// Dem Bot bot_id wird gesagt, an wen der lower-Chip geht
			if low_typ.contains("bot") {
				
				bots[bot_id].bot_lower = Some(low_val);
			}
			else { // Output
				bots[bot_id].output_lower = low_val;
			}
			
			// Dem Bot bot_id wird gesagt, an wen der higher-Chip geht
			if high_typ.contains("bot") {
				bots[bot_id].bot_higher = Some(high_val);
			}
			else { // Output
				bots[bot_id].output_higher = high_val;
			}
		}
	}
	
	// Durchlaufe alle Bots und lasse alle Bots mit 2 Werten ihre Instruktionen durchführen
	for i in 0..bots.len() {
	    let mut bot = bots[i].clone();
	    bot.interact(&mut bots,&mut outputs);
	}
	
	println!("________Bots_______:");
	for bot in bots.iter() {
		if bot.has_higher() || bot.has_lower() {
			println!("bot {} lower: {}, higher: {}",bot.id,bot.lower,bot.higher);
		}
	}
	
	println!("________Outputs_______:");
	for i in 0..outputs.len() {
		if outputs[i] != 0 {
			println!("Output {}: {}",i,outputs[i]);
		}
	}
	
	// Ausgabe für 10_02
	println!("Die gesuchte Zahl entspricht: {}",outputs[0]*outputs[1]*outputs[2]);
	
}
