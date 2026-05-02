from PySide6.QtCore import Qt
from PySide6.QtWidgets import (
    QFrame,
    QHBoxLayout,
    QLabel,
    QPushButton,
    QScrollArea,
    QSizePolicy,
    QVBoxLayout,
    QWidget,
)

_PANEL_BG = "#1a1a1a"
_ROW_BG = "#212121"
_ROW_HOVER = "#2a2a2a"

# (label, normal_bg, hover_bg, pressed_bg, tooltip)
_BUTTONS: list[tuple[str, str, str, str, str]] = [
    ("A", "#3d6b41", "#4e8a54", "#2e5230", "Actions"),
    ("S", "#6b6b22", "#8a8a2e", "#525218", "Show"),
    ("H", "#3a3a3a", "#4a4a4a", "#2a2a2a", "Hide"),
    ("L", "#235c7a", "#2e7aa0", "#1a4a62", "Label"),
    ("C", "#7a3020", "#a03e2a", "#5e2418", "Color"),
]

_BTN_TMPL = """
QPushButton {{
    color: #e0e0e0;
    font-size: 10px;
    font-weight: bold;
    border: none;
    padding: 0;
    min-width: 18px;
    max-width: 18px;
    min-height: 16px;
    max-height: 16px;
    border-radius: 2px;
    background: {bg};
}}
QPushButton:hover {{ background: {hover}; }}
QPushButton:pressed {{ background: {pressed}; }}
"""


class ObjectRow(QWidget):
    def __init__(
        self,
        name: str,
        color: str = "#4a90d9",
        parent: QWidget | None = None,
    ) -> None:
        super().__init__(parent)
        self.setAttribute(Qt.WidgetAttribute.WA_StyledBackground, True)
        self.setFixedHeight(22)
        self.setStyleSheet(f"ObjectRow {{ background: {_ROW_BG}; }}")

        layout = QHBoxLayout(self)
        layout.setContentsMargins(6, 2, 4, 2)
        layout.setSpacing(3)

        dot = QLabel()
        dot.setFixedSize(8, 8)
        dot.setAttribute(Qt.WidgetAttribute.WA_StyledBackground, True)
        dot.setStyleSheet(f"background: {color}; border-radius: 4px;")
        layout.addWidget(dot)

        lbl = QLabel(name)
        lbl.setStyleSheet("color: #c8c8c8; font-size: 11px; background: transparent;")
        lbl.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Preferred)
        layout.addWidget(lbl)

        for text, bg, hover, pressed, tip in _BUTTONS:
            btn = QPushButton(text)
            btn.setStyleSheet(_BTN_TMPL.format(bg=bg, hover=hover, pressed=pressed))
            btn.setToolTip(tip)
            layout.addWidget(btn)

    def enterEvent(self, event: object) -> None:
        self.setStyleSheet(f"ObjectRow {{ background: {_ROW_HOVER}; }}")
        super().enterEvent(event)  # type: ignore[arg-type]

    def leaveEvent(self, event: object) -> None:
        self.setStyleSheet(f"ObjectRow {{ background: {_ROW_BG}; }}")
        super().leaveEvent(event)  # type: ignore[arg-type]


class ObjectPanel(QWidget):
    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self.setMinimumWidth(185)
        self.setMaximumWidth(300)
        self.setAttribute(Qt.WidgetAttribute.WA_StyledBackground, True)
        self.setStyleSheet(f"ObjectPanel {{ background: {_PANEL_BG}; }}")

        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(0)

        # Header
        header = QLabel("Molecules / Selections")
        header.setAlignment(Qt.AlignmentFlag.AlignCenter)
        header.setFixedHeight(22)
        header.setStyleSheet(
            "QLabel { background: #111111; color: #888888; font-size: 10px; border-bottom: 1px solid #333333; }"
        )
        layout.addWidget(header)

        # Scrollable list
        scroll = QScrollArea()
        scroll.setWidgetResizable(True)
        scroll.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        scroll.setStyleSheet("QScrollArea { border: none; background: transparent; }")

        self._content = QWidget()
        self._content.setAttribute(Qt.WidgetAttribute.WA_StyledBackground, True)
        self._content.setStyleSheet(f"background: {_PANEL_BG};")
        self._list = QVBoxLayout(self._content)
        self._list.setContentsMargins(0, 2, 0, 2)
        self._list.setSpacing(1)
        self._list.addStretch()

        scroll.setWidget(self._content)
        layout.addWidget(scroll, stretch=1)

        # Separator + button row at bottom
        sep = QFrame()
        sep.setFrameShape(QFrame.Shape.HLine)
        sep.setStyleSheet("background: #333333; max-height: 1px; border: none;")
        layout.addWidget(sep)

        btn_row = QWidget()
        btn_row.setAttribute(Qt.WidgetAttribute.WA_StyledBackground, True)
        btn_row.setStyleSheet("background: #141414;")
        btn_row.setFixedHeight(24)
        btn_layout = QHBoxLayout(btn_row)
        btn_layout.setContentsMargins(4, 2, 4, 2)
        btn_layout.setSpacing(4)

        for label in ("S", "H", "L", "C"):
            b = QPushButton(label)
            b.setFixedSize(22, 18)
            b.setStyleSheet(
                "QPushButton { color: #999; font-size: 10px; background: #2a2a2a;"
                " border: 1px solid #444; border-radius: 2px; }"
                " QPushButton:hover { background: #383838; }"
            )
            b.setToolTip(f"{label} all")
            btn_layout.addWidget(b)

        btn_layout.addStretch()
        layout.addWidget(btn_row)

        # Default "(all)" entry
        self.add_object("(all)", "#888888")

    def add_object(self, name: str, color: str = "#4a90d9") -> None:
        """Add a molecule/selection row. Call after loading a file."""
        row = ObjectRow(name, color)
        self._list.insertWidget(self._list.count() - 1, row)
