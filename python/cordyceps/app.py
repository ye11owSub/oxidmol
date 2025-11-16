from cordyceps.ui.main_window import MainWindow
from PyQt6.QtWidgets import QApplication

if __name__ == "__main__":
    app = QApplication([])
    window = MainWindow()
    #with open("style.qss", "r") as f:
    #    _style = f.read()
    #    app.setStyleSheet(_style)
    window.show()
    app.exec()
