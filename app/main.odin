package app

import "core:fmt"
import "core:os"
import rl "vendor:raylib"

BG :: rl.Color{0xf5, 0xf1, 0xf9, 0xff}

main :: proc() {
	// before opening, read config

	config: Config

	if config_exists() {
		config, err := config_read()
		if err != nil {
			fmt.eprintln("failed to read config file: %v", err)
			return
		}
	} else {
		config, err := create_default_config()
		if err != nil {
			fmt.eprintfln("failed to create config file: %v", err)
			return
		}
	}

	flags :: rl.ConfigFlags{}
	rl.SetConfigFlags(flags)

	title: cstring = "pingky"

	rl.InitWindow(700, 500, title)
	rl.SetTargetFPS(30)

	running := true

	for running {
		running ~= rl.WindowShouldClose()

		rl.BeginDrawing()
		rl.ClearBackground(BG)

		rl.EndDrawing()
	}

	rl.CloseWindow()
}

