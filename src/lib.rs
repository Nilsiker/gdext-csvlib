pub mod csv;

use godot::prelude::*;

struct CsvLibExtension;

#[gdextension]
unsafe impl ExtensionLibrary for CsvLibExtension {}
