# Square

A C++ program that opens a window and draws a blue square. Close the window to exit.

## Windows

Download the latest repository files to your computer, then double-click
**play.bat**. It builds the program and opens a separate game-style window.
The game executable uses the Windows GUI subsystem, so it does not open a console.
The build step may briefly show a terminal.

You need Visual Studio or Visual Studio Build Tools with the **Desktop development
with C++** workload installed. The launcher finds the compiler automatically.

Alternatively, in a Visual Studio Developer Command Prompt:

```bat
cl /EHsc /std:c++17 square.cpp user32.lib gdi32.lib /Fe:square.exe /link /SUBSYSTEM:WINDOWS
start "" square.exe
```

After running the launcher once, double-click `build\square.exe` to open the
window directly. Re-run `play.bat` after downloading source updates; an older
executable will keep showing the old behavior.

## Linux

Install a C++ compiler and X11 development headers (on Debian/Ubuntu:
`sudo apt install g++ libx11-dev`), then build and run:

```sh
g++ -std=c++17 -Wall -Wextra square.cpp -lX11 -o square
./square
```

Run in a graphical desktop session. A terminal-only cloud session cannot show
the window on your computer. macOS is currently unsupported.
