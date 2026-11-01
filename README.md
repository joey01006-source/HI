# Square

A C++ program that opens a window and draws a blue square. Close the window to exit.

## Windows

In a Visual Studio Developer Command Prompt, build and run:

```bat
cl /EHsc /std:c++17 square.cpp user32.lib gdi32.lib /Fe:square.exe
square.exe
```

Once built, you can also double-click `square.exe`.

## Linux

Install a C++ compiler and X11 development headers (on Debian/Ubuntu:
`sudo apt install g++ libx11-dev`), then build and run:

```sh
g++ -std=c++17 -Wall -Wextra square.cpp -lX11 -o square
./square
```

Run in a graphical desktop session. A terminal-only cloud session cannot show
the window on your computer. macOS is currently unsupported.
