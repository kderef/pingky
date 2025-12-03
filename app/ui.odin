package app

import "core:strings"
import rl "vendor:raylib"

Vec2 :: rl.Vector2
Rect :: rl.Rectangle

gfont: rl.Font

FONT_PATH :: "C:\\Windows\\Fonts\\arial.ttf" when ODIN_OS == .Windows else {""}

PADDING :: 20

SIZE_UI :: SIZE_NORMAL
SIZE_NORMAL :: 26.0
SIZE_LARGE :: 50.0


gfont_measure :: proc(text: cstring, size: f32) -> Vec2 {
	return rl.MeasureTextEx(gfont, text, size, 1.0)
}

ui_init :: proc() {
	// load font
	gfont = rl.LoadFontEx(FONT_PATH, 100, nil, 0)
	rl.SetTextureFilter(gfont.texture, .BILINEAR)

	// raygui

	style_props := [][3]int {
		{0, 0, int(0x878787ff)}, // DEFAULT_BORDER_COLOR_NORMAL 
		{0, 1, int(0x2c2c2cff)}, // DEFAULT_BASE_COLOR_NORMAL 
		{0, 2, int(0xc3c3c3ff)}, // DEFAULT_TEXT_COLOR_NORMAL 
		{0, 3, int(0xe1e1e1ff)}, // DEFAULT_BORDER_COLOR_FOCUSED 
		{0, 4, int(0x848484ff)}, // DEFAULT_BASE_COLOR_FOCUSED 
		{0, 5, int(0x181818ff)}, // DEFAULT_TEXT_COLOR_FOCUSED 
		{0, 6, int(0x000000ff)}, // DEFAULT_BORDER_COLOR_PRESSED 
		{0, 7, int(0xefefefff)}, // DEFAULT_BASE_COLOR_PRESSED 
		{0, 8, int(0x202020ff)}, // DEFAULT_TEXT_COLOR_PRESSED 
		{0, 9, int(0x6a6a6aff)}, // DEFAULT_BORDER_COLOR_DISABLED 
		{0, 10, int(0x818181ff)}, // DEFAULT_BASE_COLOR_DISABLED 
		{0, 11, int(0x606060ff)}, // DEFAULT_TEXT_COLOR_DISABLED 
		{0, 16, SIZE_UI}, // DEFAULT_TEXT_SIZE 
		{0, 17, int(0x00000000)}, // DEFAULT_TEXT_SPACING 
		{0, 18, int(0x9d9d9dff)}, // DEFAULT_LINE_COLOR 
		{0, 19, int(0x3c3c3cff)}, // DEFAULT_BACKGROUND_COLOR 
		{0, 20, int(0x00000000)}, // DEFAULT_TEXT_LINE_SPACING 
		{1, 5, int(0xf7f7f7ff)}, // LABEL_TEXT_COLOR_FOCUSED 
		{1, 8, int(0x898989ff)}, // LABEL_TEXT_COLOR_PRESSED 
		{4, 5, int(0xb0b0b0ff)}, // SLIDER_TEXT_COLOR_FOCUSED 
		{5, 5, int(0x848484ff)}, // PROGRESSBAR_TEXT_COLOR_FOCUSED 
		{9, 5, int(0xf5f5f5ff)}, // TEXTBOX_TEXT_COLOR_FOCUSED 
		{10, 5, int(0xf6f6f6ff)}, // VALUEBOX_TEXT_COLOR_FOCUSED 
	}

	rl.GuiSetFont(gfont)

	for prop in style_props {
		rl.GuiSetStyle(auto_cast prop[0], auto_cast prop[1], auto_cast prop[2])
	}
}

ui_close :: proc() {
	rl.UnloadFont(gfont)
}


show_settings :: proc() {

	rl.GuiWindowBox({20, 20, 400, 400}, "settings")

	text: cstring = "hello"

	rl.GuiTextBox({20, 60, 200, 20}, text, 64, true)
}

