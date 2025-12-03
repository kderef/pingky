package app

// TODO: handle config err (messagebox maybe?)


import "core:fmt"
import "core:os"
import rl "vendor:raylib"

BG :: rl.Color{0xf5, 0xf1, 0xf9, 0xff}

main :: proc() {
	// before opening, read config

	config: Config
	conf_err: ConfigError

	if config_exists() {
		config, conf_err = config_read()
		if conf_err != nil {
			fmt.eprintln("failed to read config file: %v", conf_err)
			return
		}
	} else {
		config, conf_err = create_default_config()
		if conf_err != nil {
			fmt.eprintfln("failed to create config file: %v", conf_err)
			return
		}
	}

	fmt.printfln("config: %#v", config)


	flags :: rl.ConfigFlags{}
	rl.SetConfigFlags(flags)

	rl.InitWindow(700, 500, config.naam)

	window_w := rl.GetScreenWidth()
	window_h := rl.GetScreenHeight()

	ui_init()

	rl.SetExitKey(.KEY_NULL)
	rl.SetTargetFPS(15)

	title_size := gfont_measure(config.naam, SIZE_LARGE)
	title_pos := Vec2{f32(window_w) / 2.0 - title_size.x / 2.0, PADDING}

	running := true

	for running {
		running ~= rl.WindowShouldClose()

		rl.BeginDrawing()
		rl.ClearBackground(BG)

		rl.DrawTextEx(gfont, config.naam, title_pos, SIZE_LARGE, 1.0, rl.BLACK)

		show_settings()

		rl.EndDrawing()
	}

	ui_close()
	rl.CloseWindow()
}

