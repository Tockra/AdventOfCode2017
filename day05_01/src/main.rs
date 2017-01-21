extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::str;

fn main() {
	// Die input Datei wird in die erzeugte Binärdatei eingebunden und als eingabe gelesen. Das dynamische Einlesen von anderen Dateien 
	// ist nicht notwendig.
	// Die folgende for-Schleife durchläuft die Datei zeile für Zeile.
	for line in include_str!("../input.data").lines() {
		
		// MD5 Hasher
		let mut hasher = Md5::new();
		
		// Vektor für die Ausgabe
		let mut output = Vec::new();
		
		// i ist die aktuelle Erweiterung des Wortes line : line + id
		// i wird anschließend solange erhöht, bis ein MD5 Hash mit 5 führenden Nullen gefunden wurde
		for i in 0..std::u64::MAX {
			  hasher.input(line.as_bytes());
			  hasher.input(i.to_string().as_bytes());
			  
			  // MD5 Hashes bestehen aus 16 Bytes
			  let mut result = [0; 16]; 
		      hasher.result(&mut result);
		      
		      // Idee für diese Optimierung von : https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7
		      let first_five = result[0] as i32 + result[1] as i32 + (result[2] >> 4) as i32;
		      
		      if first_five == 0 {
				  output.push(hasher.result_str().chars().nth(5).unwrap());
		      }
		      hasher.reset(); 
		      
		      if output.len() ==8 {
				  break;
		      } 
		}
		
		// Ausgabe
		for str in output {
			print!("{}",str);
		}
		println!("");
	
	}
	
}


