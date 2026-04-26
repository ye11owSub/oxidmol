from PyQt6.QtCore import Qt
from PyQt6.QtWidgets import (
    QFileDialog,
    QMainWindow,
    QSplitter,
    QVBoxLayout,
    QWidget,
)

from cordyceps.ui.command_bar import CommandBar
from cordyceps.ui.menu import ActionToolBar, Menu
from cordyceps.ui.object_panel import ObjectPanel
from cordyceps.ui.scene import WgpuWidget

_WINDOW_STYLE = """
QMainWindow {
    background: #1c1c1c;
}
QMenuBar {
    background: #1e1e1e;
    color: #c8c8c8;
    border-bottom: 1px solid #333333;
}
QMenuBar::item {
    background: transparent;
    padding: 4px 10px;
}
QMenuBar::item:selected {
    background: #2e2e2e;
}
QMenu {
    background: #252525;
    color: #c8c8c8;
    border: 1px solid #3a3a3a;
}
QMenu::item:selected {
    background: #3a5a8a;
}
QScrollBar:vertical {
    background: #1a1a1a;
    width: 8px;
    border: none;
}
QScrollBar::handle:vertical {
    background: #404040;
    border-radius: 4px;
    min-height: 20px;
}
QScrollBar::add-line:vertical, QScrollBar::sub-line:vertical {
    height: 0;
}
"""


class MainWindow(QMainWindow):
    def __init__(self) -> None:
        super().__init__()
        self.setWindowTitle("Cordyceps")
        self.setGeometry(100, 100, 1280, 800)
        self.setStyleSheet(_WINDOW_STYLE)

        self._menu = Menu(self)
        self.setMenuBar(self._menu)
        self._toolbar = ActionToolBar(self)
        self.addToolBar(Qt.ToolBarArea.TopToolBarArea, self._toolbar)

        # ── central widget ──────────────────────────────────────────────────
        central = QWidget()
        self.setCentralWidget(central)
        root = QVBoxLayout(central)
        root.setContentsMargins(0, 0, 0, 0)
        root.setSpacing(0)

        # ── horizontal splitter: viewport | object panel ────────────────────
        splitter = QSplitter(Qt.Orientation.Horizontal)
        splitter.setHandleWidth(2)
        splitter.setStyleSheet("QSplitter::handle { background: #333333; }")

        self.wgpu_widget = WgpuWidget()
        splitter.addWidget(self.wgpu_widget)

        self.object_panel = ObjectPanel()
        splitter.addWidget(self.object_panel)

        splitter.setStretchFactor(0, 1)
        splitter.setStretchFactor(1, 0)
        splitter.setSizes([1060, 220])

        root.addWidget(splitter, stretch=1)

        # ── command bar ─────────────────────────────────────────────────────
        self.command_bar = CommandBar()
        root.addWidget(self.command_bar)

        # ── wiring ──────────────────────────────────────────────────────────
        self.wgpu_widget.molecule_loaded.connect(self._on_molecule_loaded)
        self.command_bar.command_entered.connect(self._on_command)
        self._menu.open_action.triggered.connect(self.open_file)
        self._menu.quit_action.triggered.connect(self.close)

    # ── slots ────────────────────────────────────────────────────────────────

    def _on_molecule_loaded(self, name: str) -> None:
        self.object_panel.add_object(name)
        self.command_bar.print(f" Loaded: {name}")

    def _on_command(self, cmd: str) -> None:
        pass


    def open_file(self) -> None:
        path, _ = QFileDialog.getOpenFileName(
            self,
            "Open Molecule",
            "",
            "Molecule files (*.pdb *.cif);;All files (*)",
        )
        if path:
            self.wgpu_widget.load_file(path)
