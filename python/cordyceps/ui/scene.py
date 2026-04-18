from cordyceps.lsd import PyWgpuRenderer
from PyQt6.QtCore import Qt, QTimer
from PyQt6.QtGui import QShowEvent
from PyQt6.QtWidgets import QWidget


class WgpuWidget(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setAttribute(Qt.WidgetAttribute.WA_NativeWindow, True)
        self.setAttribute(Qt.WidgetAttribute.WA_PaintOnScreen, True)
        self.setFocusPolicy(Qt.FocusPolicy.StrongFocus)
        self.setMinimumSize(640, 480)

        self.renderer = None
        self.timer = QTimer()
        self.timer.timeout.connect(self.render)

    def showEvent(self, a0: QShowEvent | None):
        if self.renderer is None:
            self.init_wgpu()
        self.timer.start(16)

    def init_wgpu(self):
        try:
            hwnd = int(self.winId())
            width, height = self.width(), self.height()

            self.renderer = PyWgpuRenderer(hwnd, width=width, height=height)
            print("WGSU renderer initialized successfully")

        except Exception as e:
            print(f"Failed to initialize WGSU: {e}")

    def render(self):
        if self.renderer:
            try:
                self.renderer.render()
            except Exception as e:
                print(f"Render error: {e}")

    def resizeEvent(self, event):
        super().resizeEvent(event)
        if self.renderer:
            try:
                self.renderer.resize(self.width(), self.height())
            except Exception as e:
                print(f"Resize error: {e}")
