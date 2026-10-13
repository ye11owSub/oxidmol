import logging
import os
from typing import override

from PySide6.QtCore import Qt, QTimer, Signal
from PySide6.QtGui import QCloseEvent, QDragEnterEvent, QDropEvent, QHideEvent, QPaintEvent, QResizeEvent, QShowEvent
from PySide6.QtWidgets import QWidget

from oxidmol.lsd import Molecule, WgpuRenderer

logger = logging.getLogger(__name__)

_ACCEPTED_EXTS = (".pdb", ".cif")
_FRAME_INTERVAL_MS = 16  # ~60 fps


class WgpuWidget(QWidget):
    """WGPU-backed 3-D viewport that draws straight into this widget's native window.

    The renderer lives exactly while the widget is shown: it is created on the first ``show``,
    the frame timer runs only while the widget is visible, and ``release_renderer`` drops the GPU
    surface (it must happen before the native window is destroyed).
    """

    molecule_loaded = Signal(str)  # basename of the loaded file
    error_occurred = Signal(str)  # human-readable message for the UI

    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        # draw into our own native window, Qt must not paint over it
        self.setAttribute(Qt.WidgetAttribute.WA_NativeWindow, True)
        self.setAttribute(Qt.WidgetAttribute.WA_PaintOnScreen, True)
        self.setAttribute(Qt.WidgetAttribute.WA_OpaquePaintEvent, True)
        self.setAttribute(Qt.WidgetAttribute.WA_NoSystemBackground, True)
        self.setFocusPolicy(Qt.FocusPolicy.StrongFocus)
        self.setMinimumSize(640, 480)
        self.setAcceptDrops(True)

        self.renderer: WgpuRenderer | None = None
        self._init_failed = False  # do not retry (and re-log) on every show
        self._timer = QTimer(self)
        self._timer.setInterval(_FRAME_INTERVAL_MS)
        self._timer.timeout.connect(self._render_frame)

    # ── window lifecycle ─────────────────────────────────────────────────────

    @override
    def paintEngine(self) -> None:  # type: ignore[override]
        return None

    @override
    def paintEvent(self, event: QPaintEvent | None) -> None:
        return None

    def showEvent(self, event: QShowEvent) -> None:
        super().showEvent(event)
        if self.renderer is None and not self._init_failed:
            self._init_renderer()
        if self.renderer is not None:
            self._timer.start()

    def hideEvent(self, event: QHideEvent) -> None:
        super().hideEvent(event)
        self._timer.stop()

    def closeEvent(self, event: QCloseEvent) -> None:
        self.release_renderer()
        super().closeEvent(event)

    def resizeEvent(self, event: QResizeEvent) -> None:
        super().resizeEvent(event)
        if self.renderer is None:
            return
        width, height = self._pixel_size()
        if width > 0 and height > 0:
            self.renderer.resize(width, height)

    def release_renderer(self) -> None:
        """Stop drawing and drop the GPU surface. Safe to call more than once."""
        self._timer.stop()
        self.renderer = None

    def _pixel_size(self) -> tuple[int, int]:
        """Size in device pixels: the GPU surface must match it or HiDPI screens look blurry."""
        ratio = self.devicePixelRatioF()
        return round(self.width() * ratio), round(self.height() * ratio)

    def _init_renderer(self) -> None:
        width, height = self._pixel_size()
        try:
            self.renderer = WgpuRenderer(int(self.winId()), width=width, height=height)
        except Exception as e:
            self._init_failed = True
            logger.exception("Failed to initialise WGPU renderer")
            self.error_occurred.emit(f"Cannot start the 3D viewport: {e}")
            return
        logger.info("WGPU renderer initialised (%dx%d)", width, height)

        # Temp: auto-load test molecule until file-open dialog is used
        test_pdb = os.path.normpath(os.path.join(os.path.dirname(__file__), "../../../tests/data/1crn.pdb"))
        if os.path.exists(test_pdb):
            self.load_file(test_pdb)

    # ── rendering ────────────────────────────────────────────────────────────

    def _render_frame(self) -> None:
        if self.renderer is None:
            return
        try:
            self.renderer.render()
        except Exception as e:
            # a failing frame would fail again 60 times a second, so stop instead of flooding the log
            self._timer.stop()
            logger.exception("Render error, stopping the frame loop")
            self.error_occurred.emit(f"Rendering stopped: {e}")

    # ── molecule loading ─────────────────────────────────────────────────────

    def load_file(self, path: str) -> None:
        """Load a PDB or mmCIF file and upload atoms to the GPU."""
        if self.renderer is None:
            logger.warning("Renderer not ready, cannot load %s", path)
            return
        try:
            self.renderer.load_molecule(path)
        except Exception as e:
            logger.exception("Failed to load molecule: %s", path)
            self.error_occurred.emit(f"Cannot load {os.path.basename(path)}: {e}")
            return
        logger.info("Loaded molecule: %s", path)
        self.molecule_loaded.emit(os.path.basename(path))

    def load_molecule_obj(self, mol: Molecule) -> None:
        """Upload an already-parsed Molecule to the GPU."""
        if self.renderer is None:
            logger.warning("Renderer not ready, cannot upload molecule")
            return
        try:
            self.renderer.load_molecule_obj(mol)
        except Exception as e:
            logger.exception("Failed to upload molecule")
            self.error_occurred.emit(f"Cannot upload molecule: {e}")
            return
        self.molecule_loaded.emit(mol.name() or "molecule")

    # ── drag and drop ────────────────────────────────────────────────────────

    def dragEnterEvent(self, a0: QDragEnterEvent | None) -> None:
        if a0 is None:
            return
        mime = a0.mimeData()
        if (
            mime is not None
            and mime.hasUrls()
            and any(u.toLocalFile().lower().endswith(_ACCEPTED_EXTS) for u in mime.urls())
        ):
            a0.acceptProposedAction()
        else:
            a0.ignore()

    def dropEvent(self, a0: QDropEvent | None) -> None:
        if a0 is None:
            return
        mime = a0.mimeData()
        if mime is None:
            a0.ignore()
            return
        for url in mime.urls():
            path = url.toLocalFile()
            if path.lower().endswith(_ACCEPTED_EXTS):
                self.load_file(path)
                a0.acceptProposedAction()
                return
        a0.ignore()
