import logging
import os
from typing import override

from PySide6.QtCore import Qt, QTimer, Signal
from PySide6.QtGui import QDragEnterEvent, QDropEvent, QResizeEvent, QShowEvent
from PySide6.QtWidgets import QWidget

from oxidmol.lsd import Molecule, WgpuRenderer

logger = logging.getLogger(__name__)

_ACCEPTED_EXTS = (".pdb", ".cif")


class WgpuWidget(QWidget):
    """WGPU-backed 3-D viewport, embeds native window handle."""

    molecule_loaded = Signal(str)  # emits basename of loaded file

    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self.setAttribute(Qt.WidgetAttribute.WA_NativeWindow, True)
        self.setAttribute(Qt.WidgetAttribute.WA_PaintOnScreen, True)
        self.setFocusPolicy(Qt.FocusPolicy.StrongFocus)
        self.setMinimumSize(640, 480)
        self.setAcceptDrops(True)

        self.renderer: WgpuRenderer | None = None
        self._timer = QTimer()
        self._timer.timeout.connect(self._render_frame)

        self.setAttribute(Qt.WidgetAttribute.WA_PaintOnScreen, True)
        self.setAttribute(Qt.WidgetAttribute.WA_OpaquePaintEvent, True)
        self.setAttribute(Qt.WidgetAttribute.WA_NoSystemBackground, True)

    # ── lifecycle ────────────────────────────────────────────────────────────

    @override
    def paintEngine(self) -> None: # type: ignore
        return None

    @override
    def paintEvent(self, event) -> None: # noqa: ARG002
        return None

    def showEvent(self, a0: QShowEvent | None) -> None:  # noqa: ARG002
        if self.renderer is None:
            self._init_renderer()
        self._timer.start(16)

    def resizeEvent(self, a0: QResizeEvent) -> None:
        new_size = a0.size()
        if self.renderer is None:
            return
        self.renderer.resize(new_size.width(), new_size.height())
        # TODO: recreate depth texture on resize


    def _init_renderer(self) -> None:
        try:
            hwnd = int(self.winId())
            self.renderer = WgpuRenderer(hwnd, width=self.width(), height=self.height())
            logger.info("WGPU renderer initialised")
        except BaseException:
            logger.exception("Failed to initialise WGPU renderer")
            return

        # Temp: auto-load test molecule until file-open dialog is used
        test_pdb = os.path.normpath(os.path.join(os.path.dirname(__file__), "../../../tests/data/1crn.pdb"))
        if os.path.exists(test_pdb):
            self.load_file(test_pdb)

    # ── rendering ────────────────────────────────────────────────────────────

    def _render_frame(self) -> None:
        if self.renderer:
            try:
                self.renderer.render()
            except BaseException:
                logger.exception("Render error")

    # ── molecule loading ─────────────────────────────────────────────────────

    def load_file(self, path: str) -> None:
        """Load a PDB or mmCIF file and upload atoms to the GPU."""
        if self.renderer is None:
            logger.warning("Renderer not ready, cannot load %s", path)
            return
        try:
            self.renderer.load_molecule(path)
            name = os.path.basename(path)
            self.molecule_loaded.emit(name)
            logger.info("Loaded molecule: %s", path)
        except Exception:
            logger.exception("Failed to load molecule: %s", path)

    def load_molecule_obj(self, mol: Molecule) -> None:
        """Upload an already-parsed Molecule to the GPU."""
        if self.renderer is None:
            logger.warning("Renderer not ready, cannot upload molecule")
            return
        try:
            self.renderer.load_molecule_obj(mol)
            name = mol.name() or "molecule"
            self.molecule_loaded.emit(name)
        except BaseException:
            logger.exception("Failed to upload molecule")

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
