import logging

from PyQt6.QtCore import Qt, QTimer
from PyQt6.QtGui import QResizeEvent, QShowEvent
from PyQt6.QtWidgets import QWidget

from cordyceps.lsd import PyWgpuRenderer

logger = logging.getLogger(__name__)


class WgpuWidget(QWidget):
    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self.setAttribute(Qt.WidgetAttribute.WA_NativeWindow, True)
        self.setAttribute(Qt.WidgetAttribute.WA_PaintOnScreen, True)
        self.setFocusPolicy(Qt.FocusPolicy.StrongFocus)
        self.setMinimumSize(640, 480)

        self.renderer: PyWgpuRenderer | None = None
        self.timer = QTimer()
        self.timer.timeout.connect(self._render_frame)

    def showEvent(self, a0: QShowEvent | None) -> None:  # noqa: ARG002
        if self.renderer is None:
            self.init_wgpu()
        self.timer.start(16)

    def init_wgpu(self) -> None:
        try:
            hwnd = int(self.winId())
            width, height = self.width(), self.height()
            self.renderer = PyWgpuRenderer(hwnd, width=width, height=height)
            logger.info("WGPU renderer initialized successfully")
        except BaseException:
            logger.exception("Failed to initialize WGPU")

    def _render_frame(self) -> None:
        if self.renderer:
            try:
                self.renderer.render()
            except BaseException:
                logger.exception("Render error")

    def resizeEvent(self, a0: QResizeEvent | None) -> None:
        super().resizeEvent(a0)
