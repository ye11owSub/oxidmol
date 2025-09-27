from PyQt6.QtCore import QSize, Qt
from PyQt6.QtGui import QIcon
from PyQt6.QtWidgets import (
    QSizePolicy,
    QStyle,
    QToolBar,
    QToolButton,
    QWidget,
    QWidgetAction,
)


class ActionToolBar(QToolBar):
    def __init__(self, parent: QWidget | None = None):
        super().__init__("ActionToolbar", parent)
        self.setMovable(False)
        self.setFloatable(False)
        self.setIconSize(QSize(18, 18))
        self.setContentsMargins(0, 0, 0, 0)
        self.setContextMenuPolicy(Qt.ContextMenuPolicy.PreventContextMenu)

        style = self.style()

        self.create_menu_button("Residues")

        undo_icon = style.standardIcon(QStyle.StandardPixmap.SP_ArrowBack)
        redo_icon = style.standardIcon(QStyle.StandardPixmap.SP_ArrowForward)

        btn_undo = self.create_menu_button("", icon=undo_icon)
        btn_redo = self.create_menu_button("", icon=redo_icon)

        btn_undo.setToolTip("Undo")
        btn_redo.setToolTip("Redo")

        self.create_menu_button("Zoom")

        self.create_menu_button("Orient")

        self.create_menu_button("Rock")

        self.create_menu_button("Presets…")

        spacer = QWidget()
        spacer.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Preferred)

        self.add_widget(spacer)

        style = self.style()

        self.create_menu_button("Builder…")

        self.create_menu_button("Scenes")

        camera_icon = style.standardIcon(QStyle.StandardPixmap.SP_DialogYesButton)
        self.create_menu_button("Draw/Ray", icon=camera_icon)

        btn_more = self.create_menu_button("…")
        btn_more.setToolButtonStyle(Qt.ToolButtonStyle.ToolButtonTextOnly)

        self.setStyleSheet("""
        QToolBar {
            background: #2b2f36;
            border: none;
            padding: 0;
            spacing: 0;
        }
        QToolButton#ToolbarButton {
            color: #e5e7eb;
            background: #3a3f46;
            border: 1px solid #474c54;
            padding: 6px 10px;
        }
        QToolButton#ToolbarButton:hover {
            background: #4a5058;
        }
        QToolButton#ToolbarButton:pressed,
        QToolButton#ToolbarButton:checked {
            background: #5b616a;
        }
        QToolButton#ToolbarButton:disabled {
            color: #9aa0a6;
            background: #3a3f46;
            border-color: #3a3f46;
        }
        QFrame#ToolbarVLine {
            background: transparent;
            border-left: 1px solid #474c54;
            margin: 0 6px;
            min-width: 1px;
            max-width: 1px;
        }
        """)

    def create_menu_button(self, text: str, icon: None | QIcon = None) -> QToolButton:
        btn = QToolButton()

        if icon is not None:
            btn.setIcon(icon)

        btn.setText(text)
        btn.setObjectName("ToolbarButton")

        self.add_widget(btn)

        return btn

    def add_widget(self, widget: QWidget) -> None:
        widget_action = QWidgetAction(self)
        widget_action.setDefaultWidget(widget)
        self.addAction(widget_action)
