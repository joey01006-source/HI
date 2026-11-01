@echo off
setlocal
cd /d "%~dp0"
where cl >nul 2>nul
if errorlevel 1 (
    if exist "%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" (
        for /f "usebackq tokens=*" %%i in (`"%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do call "%%i\VC\Auxiliary\Build\vcvars64.bat"
    )
)
where cl >nul 2>nul
if errorlevel 1 (
    echo Install Visual Studio Build Tools with Desktop development with C++, then run play.bat again.
    pause
    exit /b 1
)
if not exist build mkdir build
cl /nologo /EHsc /std:c++17 square.cpp /Fo:build\square.obj /Fe:build\square.exe user32.lib gdi32.lib /link /SUBSYSTEM:WINDOWS
if errorlevel 1 (
    pause
    exit /b 1
)
start "" "build\square.exe"
