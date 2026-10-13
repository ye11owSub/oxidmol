from PySide6.QtCore import QSize, Qt
from PySide6.QtGui import QIcon
from PySide6.QtWidgets import (
    QFrame,
    QSizePolicy,
    QStyle,
    QToolBar,
    QToolButton,
    QWidget,
    QWidgetAction,
)


class ActionToolBar(QToolBar):
    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__("Actions", parent)
        self.setMovable(False)
        self.setFloatable(False)
        self.setIconSize(QSize(16, 16))
        self.setContentsMargins(2, 0, 2, 0)
        self.setContextMenuPolicy(Qt.ContextMenuPolicy.PreventContextMenu)

        style = self.style()
        assert style is not None

        # ── View group ───────────────────────────────────────────────────────
        zoom = self._btn("Zoom All")
        zoom.setToolTip("Zoom to fit all objects")
        orient = self._btn("Orient")
        orient.setToolTip("Align camera to principal axes")
        reset = self._btn("Reset")
        reset.setToolTip("Reset camera to default")

        self._sep()

        # ── Undo / Redo ──────────────────────────────────────────────────────
        undo_icon = style.standardIcon(QStyle.StandardPixmap.SP_ArrowBack)
        redo_icon = style.standardIcon(QStyle.StandardPixmap.SP_ArrowForward)
        btn_undo = self._btn("", icon=undo_icon)
        btn_undo.setToolTip("Undo (Ctrl+Z)")
        btn_redo = self._btn("", icon=redo_icon)
        btn_redo.setToolTip("Redo (Ctrl+Y)")

        self._sep()

        # ── Rotation ─────────────────────────────────────────────────────────
        spin = self._btn("Spin")
        spin.setCheckable(True)
        spin.setToolTip("Auto-rotate around Y axis")

        self._sep()

        # ── Representations ──────────────────────────────────────────────────
        presets = self._btn("Presets…")
        presets.setToolTip("Apply a representation preset")
        presets.setEnabled(False)

        self._sep()

        # ── Views ────────────────────────────────────────────────────────────
        bookmarks = self._btn("Bookmarks")
        bookmarks.setToolTip("Manage saved views")
        bookmarks.setEnabled(False)

        # ── Spacer ───────────────────────────────────────────────────────────
        spacer = QWidget()
        spacer.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Preferred)
        self._add_widget(spacer)

        # ── Right group ──────────────────────────────────────────────────────
        self._sep()

        builder = self._btn("Builder…")
        builder.setToolTip("Open structure builder")
        builder.setEnabled(False)

        self._sep()

        render_icon = style.standardIcon(QStyle.StandardPixmap.SP_ComputerIcon)
        render = self._btn("Render", icon=render_icon)
        render.setToolTip("Render current frame")
        render.setEnabled(False)

        self._sep()

        more = self._btn("…")
        more.setToolTip("More options")
        more.setEnabled(False)

    # ── helpers ──────────────────────────────────────────────────────────────

    def _btn(self, text: str, *, icon: QIcon | None = None) -> QToolButton:
        btn = QToolButton()
        btn.setText(text)
        btn.setObjectName("TBtn")
        btn.setCursor(Qt.CursorShape.PointingHandCursor)
        if icon is not None:
            btn.setIcon(icon)
            btn.setToolButtonStyle(
                Qt.ToolButtonStyle.ToolButtonTextBesideIcon if text else Qt.ToolButtonStyle.ToolButtonIconOnly
            )
        self._add_widget(btn)
        return btn

    def _sep(self) -> None:
        line = QFrame()
        line.setObjectName("VSep")
        line.setFrameShape(QFrame.Shape.VLine)
        self._add_widget(line)

    def _add_widget(self, widget: QWidget) -> None:
        action = QWidgetAction(self)
        action.setDefaultWidget(widget)
        self.addAction(action)
