from PySide6.QtCore import Signal
from PySide6.QtWidgets import (
    QFrame,
    QHBoxLayout,
    QLabel,
    QLineEdit,
    QPlainTextEdit,
    QVBoxLayout,
    QWidget,
)

_LOG_STYLE = """
QPlainTextEdit {
    background: #0e0e0e;
    color: #b8b8b8;
    border: none;
    font-family: monospace;
    font-size: 11px;
    padding: 4px 6px;
    selection-background-color: #2a4a7a;
}
"""

_PROMPT_STYLE = """
QLabel {
    background: #080808;
    color: #00bb3a;
    font-family: monospace;
    font-size: 12px;
    padding: 3px 4px 3px 8px;
    border-top: 1px solid #2e2e2e;
    border-right: 1px solid #2e2e2e;
}
"""

_INPUT_STYLE = """
QLineEdit {
    background: #080808;
    color: #00dd44;
    border: none;
    border-top: 1px solid #2e2e2e;
    font-family: monospace;
    font-size: 12px;
    padding: 3px 6px;
}
QLineEdit:focus {
    border-top: 1px solid #4a90d9;
}
"""


class CommandBar(QWidget):
    command_entered = Signal(str)

    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self.setFixedHeight(120)

        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(0)

        sep = QFrame()
        sep.setFrameShape(QFrame.Shape.HLine)
        sep.setStyleSheet("background: #333333; max-height: 1px; border: none;")
        layout.addWidget(sep)

        self._log = QPlainTextEdit()
        self._log.setReadOnly(True)
        self._log.setMaximumBlockCount(1000)
        self._log.setStyleSheet(_LOG_STYLE)
        layout.addWidget(self._log, stretch=1)

        input_row = QWidget()
        row_layout = QHBoxLayout(input_row)
        row_layout.setContentsMargins(0, 0, 0, 0)
        row_layout.setSpacing(0)

        prompt = QLabel(">")
        prompt.setStyleSheet(_PROMPT_STYLE)
        row_layout.addWidget(prompt)

        self._input = QLineEdit()
        self._input.setStyleSheet(_INPUT_STYLE)
        self._input.setPlaceholderText("enter command…")
        self._input.returnPressed.connect(self._on_enter)
        row_layout.addWidget(self._input)

        layout.addWidget(input_row)

        self.print(" OxidMol — open-source molecular viewer")
        self.print(" Drag-and-drop a .pdb or .cif file, or use File > Open")

    def print(self, text: str) -> None:
        """Append a line to the log area."""
        self._log.appendPlainText(text)

    def _on_enter(self) -> None:
        cmd = self._input.text().strip()
        if not cmd:
            return
        self.print(f"c> {cmd}")
        self._input.clear()
        self.command_entered.emit(cmd)
