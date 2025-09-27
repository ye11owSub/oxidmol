from cordyceps.ui.menu import ActionToolBar, Menu
from cordyceps.ui.scene import WgpuWidget
from PyQt6.QtCore import Qt
from PyQt6.QtWidgets import QHBoxLayout, QLabel, QMainWindow, QVBoxLayout, QWidget


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Cordyceps")
        self.setGeometry(100, 100, 800, 600)

        menu = Menu(self)
        self.setMenuBar(menu)

        self.action_toolbar = ActionToolBar(self)
        self.addToolBar(Qt.ToolBarArea.TopToolBarArea, self.action_toolbar)

        main_layout = QVBoxLayout()

        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        central_widget.setLayout(main_layout)

        top_layout = QHBoxLayout()
        bottom_layout = QHBoxLayout()
        main_layout.addLayout(top_layout)
        main_layout.addLayout(bottom_layout)

        placeholder1 = QLabel("Scene area 1")
        placeholder1.setAlignment(Qt.AlignmentFlag.AlignCenter)
        placeholder1.setStyleSheet("background:#111; color:#bbb;")
        top_layout.addWidget(placeholder1 )

        self.wgpu_widget = WgpuWidget()
        top_layout.addWidget(self.wgpu_widget)

        placeholder2 = QLabel("Scene area 2")
        placeholder2.setAlignment(Qt.AlignmentFlag.AlignCenter)
        placeholder2.setStyleSheet("background:#111; color:#bbb;")
        top_layout.addWidget(placeholder2)

        placeholder3 = QLabel("Scene area 3")
        placeholder3.setAlignment(Qt.AlignmentFlag.AlignCenter)
        placeholder3.setStyleSheet("background:#111; color:#bbb;")
        bottom_layout.addWidget(placeholder3)

