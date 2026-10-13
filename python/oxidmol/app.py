from PySide6.QtWidgets import QApplication

from oxidmol.ui.main_window import MainWindow


def main() -> None:
    app = QApplication([])
    app.setApplicationName("OxidMol")
    app.setStyle("Fusion")  # the stylesheet looks the same on every platform
    window = MainWindow()
    window.show()
    app.exec()


if __name__ == "__main__":
    main()
