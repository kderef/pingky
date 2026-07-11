LIBS = -luser32 -lkernel32 -lgdi32 -lshell32

pingky: main.c
	gcc main.c -o pingky -nostdlib -s $(LIBS) -e WinMainCRTStartup -mwindows
