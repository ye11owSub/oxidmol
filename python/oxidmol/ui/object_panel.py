from PySide6.QtCore import Qt
from PySide6.QtWidgets import (
    QHBoxLayout,
    QLabel,
    QPushButton,
    QScrollArea,
    QSizePolicy,
    QVBoxLayout,
    QWidget,
)

from oxidmol.ui.theme import DEFAULT_OBJECT_COLOR, MUTED_OBJECT_COLOR

# (letter, tooltip): the PyMOL-style action menus of a row
_ROW_BUTTONS: list[tuple[str, str]] = [
    ("A", "Actions"),
    ("S", "Show"),
    ("H", "Hide"),
    ("L", "Label"),
    ("C", "Color"),
]

_FOOTER_BUTTONS: list[tuple[str, str]] = [
    ("S", "Show all"),
    ("H", "Hide all"),
    ("L", "Label all"),
    ("C", "Color all"),
]


def _flat_button(text: str, tooltip: str, object_name: str) -> QPushButton:
    button = QPushButton(text)
    button.setObjectName(object_name)
    button.setToolTip(tooltip)
    button.setCursor(Qt.CursorShape.PointingHandCursor)
    return button


class ObjectRow(QWidget):
    def __init__(
        self,
        name: str,
        color: str = DEFAULT_OBJECT_COLOR,
        parent: QWidget | None = None,
    ) -> None:
        super().__init__(parent)
        self.setObjectName("ObjectRow")
        self.setAttribute(Qt.WidgetAttribute.WA_StyledBackground, True)
        self.setFixedHeight(26)

        layout = QHBoxLayout(self)
        layout.setContentsMargins(8, 2, 6, 2)
        layout.setSpacing(4)

        dot = QLabel()
        dot.setFixedSize(8, 8)
        dot.setStyleSheet(f"background: {color}; border-radius: 4px;")  # the only per-object color
        layout.addWidget(dot)

        label = QLabel(name)
        label.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Preferred)
        layout.addWidget(label)

        for text, tooltip in _ROW_BUTTONS:
            layout.addWidget(_flat_button(text, tooltip, "RowButton"))


class ObjectPanel(QWidget):
    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self.setObjectName("ObjectPanel")
        self.setAttribute(Qt.WidgetAttribute.WA_StyledBackground, True)
        self.setMinimumWidth(185)
        self.setMaximumWidth(300)

        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(0)

        header = QLabel("Molecules / Selections")
        header.setObjectName("ObjectPanelHeader")
        header.setAlignment(Qt.AlignmentFlag.AlignCenter)
        header.setFixedHeight(26)
        layout.addWidget(header)

        scroll = QScrollArea()
        scroll.setObjectName("ObjectScroll")
        scroll.setWidgetResizable(True)
        scroll.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)

        content = QWidget()
        content.setObjectName("ObjectList")
        self._list = QVBoxLayout(content)
        self._list.setContentsMargins(4, 4, 4, 4)
        self._list.setSpacing(2)
        self._list.addStretch()

        scroll.setWidget(content)
        layout.addWidget(scroll, stretch=1)

        footer = QWidget()
        footer.setObjectName("ObjectFooter")
        footer.setAttribute(Qt.WidgetAttribute.WA_StyledBackground, True)
        footer.setFixedHeight(28)
        footer_layout = QHBoxLayout(footer)
        footer_layout.setContentsMargins(6, 3, 6, 3)
        footer_layout.setSpacing(4)
        for text, tooltip in _FOOTER_BUTTONS:
            footer_layout.addWidget(_flat_button(text, tooltip, "FooterButton"))
        footer_layout.addStretch()
        layout.addWidget(footer)

        self.add_object("(all)", MUTED_OBJECT_COLOR)

    def add_object(self, name: str, color: str = DEFAULT_OBJECT_COLOR) -> None:
        """Add a molecule/selection row. Call after loading a file."""
        self._list.insertWidget(self._list.count() - 1, ObjectRow(name, color))
