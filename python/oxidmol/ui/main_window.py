from lark import Lark, UnexpectedInput
from PySide6.QtCore import Qt
from PySide6.QtGui import QCloseEvent
from PySide6.QtWidgets import (
    QFileDialog,
    QMainWindow,
    QSplitter,
    QVBoxLayout,
    QWidget,
)

from oxidmol.dsl.commands import LoadMoleculeResult
from oxidmol.dsl.grammar import GRAMMAR
from oxidmol.dsl.transformer import CordycepsTransformer
from oxidmol.ui.command_bar import CommandBar
from oxidmol.ui.menu import ActionToolBar, Menu
from oxidmol.ui.object_panel import ObjectPanel
from oxidmol.ui.scene import WgpuWidget
from oxidmol.ui.theme import STYLESHEET


class MainWindow(QMainWindow):
    def __init__(self) -> None:
        super().__init__()
        self.setWindowTitle("OxidMol")
        self.setGeometry(100, 100, 1280, 800)
        self.setMinimumSize(960, 600)
        self.setStyleSheet(STYLESHEET)

        self._parser = Lark(GRAMMAR, parser="earley", start="program")
        self._transformer = CordycepsTransformer()

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
        splitter.setHandleWidth(1)
        splitter.setChildrenCollapsible(False)

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
        self.wgpu_widget.error_occurred.connect(self._on_viewport_error)
        self.command_bar.command_entered.connect(self._on_command)
        self._menu.open_action.triggered.connect(self.open_file)
        self._menu.quit_action.triggered.connect(self.close)

    # ── slots ────────────────────────────────────────────────────────────────

    def _on_molecule_loaded(self, name: str) -> None:
        self.object_panel.add_object(name)
        self.command_bar.print(f" Loaded: {name}")

    def _on_viewport_error(self, message: str) -> None:
        self.command_bar.print(f" Error: {message}")

    def closeEvent(self, event: QCloseEvent) -> None:
        # the GPU surface must go before the native window it draws into
        self.wgpu_widget.release_renderer()
        super().closeEvent(event)

    def _on_command(self, cmd: str) -> None:
        try:
            tree = self._parser.parse(cmd + "\n")
            result = self._transformer.transform(tree)
        except UnexpectedInput as e:
            self.command_bar.print(f" Parse error: {e}")
            return
        except Exception as e:  # noqa: BLE001
            self.command_bar.print(f" Error: {e}")
            return

        if isinstance(result, LoadMoleculeResult):
            self.wgpu_widget.load_molecule_obj(result.molecule)

    def open_file(self) -> None:
        path, _ = QFileDialog.getOpenFileName(
            self,
            "Open Molecule",
            "",
            "Molecule files (*.pdb *.cif);;All files (*)",
        )
        if path:
            self.wgpu_widget.load_file(path)
