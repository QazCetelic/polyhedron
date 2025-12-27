pub mod identify; // Get launcher info from header using the first line, standalone
pub mod index; // Index the log header and return the positions of each specific found part for more detailed info
pub mod extract; // Functions to extract specific info from the header using the index
pub mod info; // All extractable info from header