extern crate ndarray;

use ndarray::Array2;
use ndarray::Axis;


fn main() {
	let mut screen = Array2::from_elem((50, 6), '.');
	
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		// Datei wird geparset. Das ist hier etwas hässlig, aber es funktioniert ;) 
		if line.contains("row") {
			let coords: Vec<&str> = line[13..].split(" ").collect();
			rotate_row(&mut screen,coords[0].parse::<usize>().unwrap(),coords[coords.len()-1].parse::<usize>().unwrap())
		}
		else if line.contains("column") {
			let coords: Vec<&str> = line[16..].split(" ").collect();
			rotate_column(&mut screen,coords[0].parse::<usize>().unwrap(),coords[coords.len()-1].parse::<usize>().unwrap());
		}
		else if line.contains("rect") {
			let coords: Vec<&str> = line[5..].split("x").collect();
			rect(&mut screen,coords[0].parse::<usize>().unwrap(),coords[1].parse::<usize>().unwrap());
		}
	}
	
	// Variable zum zählen der leuchtenden Pixel. Das Zählen kann mit der Ausgabe verbunden werden
	let mut count = 0;
	
	// Ausgabe
	for y in 0..screen.len_of(Axis(1)) {
		for x in 0..screen.len_of(Axis(0)) {
			if screen[[x,y]] == '#' {
				count += 1;
			}
			print!("{}",screen[[x,y]]);
		}
		println!("");
	}
	println!("Anzahl der leuchtenden Pixel: {}",count);
}

// Funktion, die ein Rechteck an # erzeugt
fn rect(screen: &mut Array2<char>,a: usize, b: usize) {
	for x in 0..a {
		for y in 0..b {
			screen[[x,y]] = '#';
		}
	}
}
// Funktion, die Spalten verschiebt
fn rotate_column(screen: &mut Array2<char>, x: usize, dis: usize) {	
	for _ in 0..dis {
		let copy = screen.clone();
		
		screen[[x,0]] = copy[[x,screen.len_of(Axis(1))-1]];
		for y in 1..copy.len_of(Axis(1)) {
			screen[[x,y]] = copy[[x,y-1]];
		}
	}
}

// Funktion die Zeilen verschiebt
fn rotate_row(screen: &mut Array2<char>,y: usize, dis: usize) {
	for _ in 0..dis {
		let copy = screen.clone();
		
		screen[[0,y]] = copy[[screen.len_of(Axis(0))-1,y]];
		for x in 1..copy.len_of(Axis(0)) {
			screen[[x,y]] = copy[[x-1,y]]
		}
	}
}