#if defined(_WIN32)
#define WIN32_LEAN_AND_MEAN
#include <windows.h>

LRESULT CALLBACK windowProcedure(HWND window, UINT message, WPARAM w, LPARAM l) {
    if (message == WM_PAINT) {
        PAINTSTRUCT paint;
        HDC context = BeginPaint(window, &paint);
        RECT bounds;
        GetClientRect(window, &bounds);
        int x = (bounds.right - 200) / 2, y = (bounds.bottom - 200) / 2;
        RECT square = {x, y, x + 200, y + 200};
        HBRUSH brush = CreateSolidBrush(RGB(40, 120, 220));
        FillRect(context, &square, brush);
        DeleteObject(brush);
        EndPaint(window, &paint);
        return 0;
    }
    if (message == WM_DESTROY) {
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProcW(window, message, w, l);
}

int WINAPI WinMain(HINSTANCE instance, HINSTANCE, LPSTR, int showCommand) {
    WNDCLASSW definition = {};
    definition.lpfnWndProc = windowProcedure;
    definition.hInstance = instance;
    definition.lpszClassName = L"SquareWindow";
    definition.hCursor = LoadCursor(nullptr, IDC_ARROW);
    definition.hbrBackground = reinterpret_cast<HBRUSH>(COLOR_WINDOW + 1);
    definition.style = CS_HREDRAW | CS_VREDRAW;
    if (!RegisterClassW(&definition)) return 1;
    HWND window = CreateWindowExW(0, definition.lpszClassName, L"Square",
        WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, CW_USEDEFAULT, 500, 500,
        nullptr, nullptr, instance, nullptr);
    if (!window) return 1;
    ShowWindow(window, showCommand);
    MSG message;
    int result;
    while ((result = GetMessageW(&message, nullptr, 0, 0)) > 0) {
        TranslateMessage(&message);
        DispatchMessageW(&message);
    }
    return result == -1 ? 1 : 0;
}
#elif defined(__linux__)
#include <X11/Xlib.h>
#include <iostream>

int main() {
    Display* display = XOpenDisplay(nullptr);
    if (!display) {
        std::cerr << "Cannot open a display. Run in a graphical desktop session.\n";
        return 1;
    }
    int screen = DefaultScreen(display);
    Window window = XCreateSimpleWindow(display, RootWindow(display, screen),
        0, 0, 500, 500, 0, BlackPixel(display, screen), WhitePixel(display, screen));
    XStoreName(display, window, "Square");
    XSelectInput(display, window, ExposureMask | StructureNotifyMask);
    Atom closeMessage = XInternAtom(display, "WM_DELETE_WINDOW", False);
    XSetWMProtocols(display, window, &closeMessage, 1);
    GC context = XCreateGC(display, window, 0, nullptr);
    XColor blue;
    if (XParseColor(display, DefaultColormap(display, screen), "#2878dc", &blue) &&
        XAllocColor(display, DefaultColormap(display, screen), &blue)) {
        XSetForeground(display, context, blue.pixel);
    } else {
        XSetForeground(display, context, BlackPixel(display, screen));
    }
    XMapWindow(display, window);
    bool running = true;
    while (running) {
        XEvent event;
        XNextEvent(display, &event);
        if (event.type == Expose || event.type == ConfigureNotify) {
            XWindowAttributes bounds;
            XGetWindowAttributes(display, window, &bounds);
            XClearWindow(display, window);
            XFillRectangle(display, window, context, (bounds.width - 200) / 2,
                           (bounds.height - 200) / 2, 200, 200);
        } else if (event.type == ClientMessage &&
                   static_cast<Atom>(event.xclient.data.l[0]) == closeMessage) {
            running = false;
        }
    }
    XFreeGC(display, context);
    XDestroyWindow(display, window);
    XCloseDisplay(display);
    return 0;
}
#else
#error "This program supports Windows and Linux desktops."
#endif
