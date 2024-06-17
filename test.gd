extends Node

# Called when the node enters the scene tree for the first time.
func _ready():
	var csv = TranslationCsv.from_path("res://in.csv")
	csv.replace_value("GREET", "en", "HOWDY FROM GODOT!")	
	csv.write_csv("res://out.csv")
