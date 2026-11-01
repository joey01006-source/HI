"""Double-click on Windows with Python installed to open the square window."""

import tkinter as tk


def main():
    window = tk.Tk()
    window.title("Square")
    window.geometry("500x500")
    window.minsize(220, 220)

    canvas = tk.Canvas(window, background="white", highlightthickness=0)
    canvas.pack(fill=tk.BOTH, expand=True)
    square = canvas.create_rectangle(0, 0, 200, 200, fill="#2878dc", outline="")

    def center_square(event):
        x = (event.width - 200) / 2
        y = (event.height - 200) / 2
        canvas.coords(square, x, y, x + 200, y + 200)

    canvas.bind("<Configure>", center_square)
    window.mainloop()


if __name__ == "__main__":
    main()
