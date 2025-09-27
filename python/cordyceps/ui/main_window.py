from cordyceps.ui.menu import ActionToolBar, Menu
from cordyceps.ui.scene import WgpuWidget
from PyQt6.QtCore import Qt
from PyQt6.QtWidgets import QLabel, QMainWindow, QVBoxLayout, QWidget


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Cordyceps")
        self.setGeometry(100, 100, 800, 600)

        menu = Menu(self)
        self.setMenuBar(menu)

        self.action_toolbar = ActionToolBar(self)
        self.addToolBar(Qt.ToolBarArea.TopToolBarArea, self.action_toolbar)

        placeholder = QLabel("Scene area")
        placeholder.setAlignment(Qt.AlignmentFlag.AlignCenter)
        placeholder.setStyleSheet("background:#111; color:#bbb;")
        self.setCentralWidget(placeholder)

        central_widget = QWidget()
        layout = QVBoxLayout()

        self.wgpu_widget = WgpuWidget()
        layout.addWidget(self.wgpu_widget)

        central_widget.setLayout(layout)
        self.setCentralWidget(central_widget)
