from PyQt6.QtGui import QAction, QKeySequence
from PyQt6.QtWidgets import (
    QMainWindow,
    QMenu,
    QMenuBar,
)


class Menu(QMenuBar):
    ELEMENT_TITELS = (
        "File",
        "View",
        "Build",
        "Movie",
        "Display",
        "Settings",
        "Scenes",
        "Mouse",
        "Wizard",
        "Plugin",
        "Help",
    )

    def __init__(self, parent: QMainWindow):
        super().__init__(parent)
        act_open = QAction("...", self)
        act_open.setShortcut(QKeySequence.StandardKey.Open)

        for element in self.ELEMENT_TITELS:
            self.create_menu_element(element, act_open)

    def create_menu_element(self, title: str, action: QAction) -> None:
        file_menu = QMenu(title, self)
        file_menu.addAction(action)
        self.addMenu(file_menu)
