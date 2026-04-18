from PyQt6.QtWidgets import QApplication

from cordyceps.ui.main_window import MainWindow


def main() -> None:
    app = QApplication([])
    window = MainWindow()
    # with open("style.qss", "r") as f:
    #    _style = f.read()
    #    app.setStyleSheet(_style)
    window.show()
    app.exec()


if __name__ == "__main__":
    main()
