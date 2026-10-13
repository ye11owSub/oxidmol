from PySide6.QtCore import Qt, Signal
from PySide6.QtWidgets import (
    QHBoxLayout,
    QLabel,
    QLineEdit,
    QPlainTextEdit,
    QVBoxLayout,
    QWidget,
)

from oxidmol.ui.theme import mono_font


class CommandBar(QWidget):
    command_entered = Signal(str)

    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self.setObjectName("CommandBar")
        self.setAttribute(Qt.WidgetAttribute.WA_StyledBackground, True)
        self.setFixedHeight(120)

        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(0)

        self._log = QPlainTextEdit()
        self._log.setReadOnly(True)
        self._log.setMaximumBlockCount(1000)
        self._log.setObjectName("CommandLog")
        self._log.setFont(mono_font(10))
        layout.addWidget(self._log, stretch=1)

        input_row = QWidget()
        row_layout = QHBoxLayout(input_row)
        row_layout.setContentsMargins(0, 0, 0, 0)
        row_layout.setSpacing(0)

        prompt = QLabel(">")
        prompt.setObjectName("CommandPrompt")
        prompt.setFont(mono_font(11))
        row_layout.addWidget(prompt)

        self._input = QLineEdit()
        self._input.setObjectName("CommandInput")
        self._input.setFont(mono_font(11))
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
