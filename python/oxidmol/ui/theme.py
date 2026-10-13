"""Colors, fonts and the application stylesheet in one place.

Widgets only set an ``objectName``; everything visual lives in ``STYLESHEET``.
"""

from string import Template

from PySide6.QtGui import QFont, QFontDatabase

# surfaces, darkest to lightest
VIEWPORT = "#0d0d1a"  # same as the renderer clear color (0.05, 0.05, 0.10)
WINDOW = "#14141f"
PANEL = "#1a1a28"
RAISED = "#252538"
BORDER = "#2c2c42"

# text
TEXT = "#d8dae5"
TEXT_MUTED = "#8a8ea6"
TEXT_DISABLED = "#555a70"

# accents
ACCENT = "#5b8def"
ACCENT_SOFT = "#2a3d6b"
SUCCESS = "#4cc38a"

DEFAULT_OBJECT_COLOR = ACCENT
MUTED_OBJECT_COLOR = TEXT_MUTED


def mono_font(point_size: int = 10) -> QFont:
    """The platform's fixed-width font (there is no family called "Monospace" on macOS)."""
    font = QFontDatabase.systemFont(QFontDatabase.SystemFont.FixedFont)
    font.setPointSize(point_size)
    return font


_TEMPLATE = Template("""
QMainWindow { background: $WINDOW; }

QToolTip {
    background: $RAISED;
    color: $TEXT;
    border: 1px solid $BORDER;
    padding: 4px 6px;
}

/* ── menus ─────────────────────────────────────────────────────────────── */
QMenuBar {
    background: $WINDOW;
    color: $TEXT;
    border-bottom: 1px solid $BORDER;
    padding: 2px 4px;
}
QMenuBar::item { background: transparent; padding: 4px 10px; border-radius: 4px; }
QMenuBar::item:selected { background: $RAISED; }

QMenu {
    background: $PANEL;
    color: $TEXT;
    border: 1px solid $BORDER;
    border-radius: 6px;
    padding: 4px;
}
QMenu::item { padding: 6px 28px 6px 12px; border-radius: 4px; }
QMenu::item:selected { background: $ACCENT_SOFT; }
QMenu::item:disabled { color: $TEXT_DISABLED; }
QMenu::separator { height: 1px; background: $BORDER; margin: 4px 8px; }

/* ── toolbar ───────────────────────────────────────────────────────────── */
QToolBar {
    background: $WINDOW;
    border: none;
    border-bottom: 1px solid $BORDER;
    padding: 3px 4px;
    spacing: 2px;
}
QToolButton#TBtn {
    color: $TEXT;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 4px;
    padding: 4px 10px;
}
QToolButton#TBtn:hover { background: $RAISED; border-color: $BORDER; }
QToolButton#TBtn:pressed, QToolButton#TBtn:checked { background: $ACCENT_SOFT; border-color: $ACCENT; }
QToolButton#TBtn:disabled { color: $TEXT_DISABLED; }
QFrame#VSep { background: $BORDER; border: none; min-width: 1px; max-width: 1px; margin: 6px 4px; }

/* ── splitter, scrollbars ──────────────────────────────────────────────── */
QSplitter::handle { background: $BORDER; }

QScrollBar:vertical { background: transparent; width: 8px; margin: 2px; border: none; }
QScrollBar::handle:vertical { background: $BORDER; border-radius: 3px; min-height: 24px; }
QScrollBar::handle:vertical:hover { background: $TEXT_DISABLED; }
QScrollBar::add-line:vertical, QScrollBar::sub-line:vertical { height: 0; }
QScrollBar::add-page:vertical, QScrollBar::sub-page:vertical { background: transparent; }

/* ── object panel ──────────────────────────────────────────────────────── */
QWidget#ObjectPanel { background: $PANEL; }
QLabel#ObjectPanelHeader {
    background: $WINDOW;
    color: $TEXT_MUTED;
    border-bottom: 1px solid $BORDER;
    font-size: 11px;
    font-weight: 600;
}
QScrollArea#ObjectScroll { border: none; background: $PANEL; }
QWidget#ObjectList { background: $PANEL; }

QWidget#ObjectRow { background: transparent; border-radius: 4px; }
QWidget#ObjectRow:hover { background: $RAISED; }
QWidget#ObjectRow QLabel { background: transparent; color: $TEXT; }

QPushButton#RowButton, QPushButton#FooterButton {
    color: $TEXT_MUTED;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 3px;
    font-size: 10px;
    font-weight: 600;
    padding: 0;
}
QPushButton#RowButton { min-width: 18px; max-width: 18px; min-height: 16px; max-height: 16px; }
QPushButton#FooterButton { min-width: 24px; max-width: 24px; min-height: 20px; max-height: 20px; }
QPushButton#RowButton:hover, QPushButton#FooterButton:hover {
    color: $TEXT;
    background: $RAISED;
    border-color: $BORDER;
}
QPushButton#RowButton:pressed, QPushButton#FooterButton:pressed { background: $ACCENT_SOFT; }

QWidget#ObjectFooter { background: $WINDOW; border-top: 1px solid $BORDER; }

/* ── command bar ───────────────────────────────────────────────────────── */
QWidget#CommandBar { background: $VIEWPORT; }
QPlainTextEdit#CommandLog {
    background: $VIEWPORT;
    color: $TEXT_MUTED;
    border: none;
    border-top: 1px solid $BORDER;
    padding: 4px 8px;
    selection-background-color: $ACCENT_SOFT;
}
QLabel#CommandPrompt {
    background: $VIEWPORT;
    color: $SUCCESS;
    border-top: 1px solid $BORDER;
    padding: 4px 4px 4px 10px;
    font-weight: bold;
}
QLineEdit#CommandInput {
    background: $VIEWPORT;
    color: $TEXT;
    border: none;
    border-top: 1px solid $BORDER;
    padding: 4px 6px;
    selection-background-color: $ACCENT_SOFT;
}
QLineEdit#CommandInput:focus { border-top: 1px solid $ACCENT; }
""")

STYLESHEET = _TEMPLATE.substitute(
    VIEWPORT=VIEWPORT,
    WINDOW=WINDOW,
    PANEL=PANEL,
    RAISED=RAISED,
    BORDER=BORDER,
    TEXT=TEXT,
    TEXT_MUTED=TEXT_MUTED,
    TEXT_DISABLED=TEXT_DISABLED,
    ACCENT=ACCENT,
    ACCENT_SOFT=ACCENT_SOFT,
    SUCCESS=SUCCESS,
)
