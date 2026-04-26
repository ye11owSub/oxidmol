from PyQt6.QtGui import QAction, QKeySequence
from PyQt6.QtWidgets import QMainWindow, QMenu, QMenuBar


def _stub(parent: QMenuBar, label: str) -> QAction:
    a = QAction(label, parent)
    a.setEnabled(False)
    return a


class Menu(QMenuBar):
    def __init__(self, parent: QMainWindow) -> None:
        super().__init__(parent)
        self.open_action: QAction
        self.quit_action: QAction
        self.about_action: QAction

        self._build_file()
        self._build_view()
        self._build_structure()
        self._build_appearance()
        self._build_animation()
        self._build_analysis()
        self._build_bookmarks()
        self._build_plugins()
        self._build_help()

    # ── File ─────────────────────────────────────────────────────────────────

    def _build_file(self) -> None:
        m = QMenu("File", self)

        self.open_action = QAction("Open…", self)
        self.open_action.setShortcut(QKeySequence.StandardKey.Open)
        m.addAction(self.open_action)

        recent = QMenu("Open Recent", m)
        recent.setEnabled(False)
        m.addMenu(recent)

        m.addSeparator()

        save = QAction("Save Session As…", self)
        save.setShortcut(QKeySequence.StandardKey.Save)
        save.setEnabled(False)
        m.addAction(save)

        m.addSeparator()

        for label in ("Export Image…", "Export Animation…", "Export Molecule…"):
            m.addAction(_stub(self, label))

        m.addSeparator()

        run = QAction("Run Script…", self)
        run.setEnabled(False)
        m.addAction(run)

        m.addSeparator()

        self.quit_action = QAction("Quit", self)
        self.quit_action.setShortcut(QKeySequence.StandardKey.Quit)
        m.addAction(self.quit_action)

        self.addMenu(m)

    # ── View ─────────────────────────────────────────────────────────────────

    def _build_view(self) -> None:
        m = QMenu("View", self)

        seq = QAction("Sequence Panel", self)
        seq.setCheckable(True)
        seq.setEnabled(False)
        m.addAction(seq)

        m.addSeparator()

        zoom_in = QAction("Zoom In", self)
        zoom_in.setShortcut(QKeySequence.StandardKey.ZoomIn)
        zoom_in.setEnabled(False)
        m.addAction(zoom_in)

        zoom_out = QAction("Zoom Out", self)
        zoom_out.setShortcut(QKeySequence.StandardKey.ZoomOut)
        zoom_out.setEnabled(False)
        m.addAction(zoom_out)

        reset = QAction("Reset View", self)
        reset.setShortcut(QKeySequence("R"))
        reset.setEnabled(False)
        m.addAction(reset)

        m.addSeparator()

        fullscreen = QAction("Full Screen", self)
        fullscreen.setShortcut(QKeySequence.StandardKey.FullScreen)
        fullscreen.setCheckable(True)
        fullscreen.setEnabled(False)
        m.addAction(fullscreen)

        self.addMenu(m)

    # ── Structure ────────────────────────────────────────────────────────────

    def _build_structure(self) -> None:
        m = QMenu("Structure", self)

        frag = QMenu("Add Fragment", m)
        for label in ("Acetyl", "Amine", "Benzene", "Cyclohexane"):
            frag.addAction(_stub(self, label))
        m.addMenu(frag)

        res = QMenu("Add Residue", m)
        for label in ("ALA", "GLY", "PHE", "TRP", "SER", "THR"):
            res.addAction(_stub(self, label))
        m.addMenu(res)

        m.addSeparator()
        m.addAction(_stub(self, "Energy Minimise"))

        self.addMenu(m)

    # ── Appearance ───────────────────────────────────────────────────────────

    def _build_appearance(self) -> None:
        m = QMenu("Appearance", self)

        bg_menu = QMenu("Background", m)
        for color in ("Dark", "Light", "Black", "White"):
            bg_menu.addAction(_stub(self, color))
        m.addMenu(bg_menu)

        m.addSeparator()

        repr_menu = QMenu("Representation", m)
        for r in ("Lines", "Sticks", "Spheres", "Surface", "Cartoon", "Ribbon"):
            repr_menu.addAction(_stub(self, r))
        m.addMenu(repr_menu)

        m.addSeparator()

        quality_menu = QMenu("Quality", m)
        for q in ("Draft", "Normal", "High", "Ultra"):
            a = QAction(q, self)
            a.setCheckable(True)
            a.setEnabled(False)
            quality_menu.addAction(a)
        m.addMenu(quality_menu)

        transp = QMenu("Transparency", m)
        for t in ("None", "25%", "50%", "75%"):
            transp.addAction(_stub(self, t))
        m.addMenu(transp)

        m.addSeparator()

        fps = QAction("Show FPS", self)
        fps.setCheckable(True)
        fps.setEnabled(False)
        m.addAction(fps)

        self.addMenu(m)

    # ── Animation ────────────────────────────────────────────────────────────

    def _build_animation(self) -> None:
        m = QMenu("Animation", self)

        for label in ("Play", "Stop", "Rewind", "Reset", "Clear"):
            m.addAction(_stub(self, label))

        m.addSeparator()

        grid_menu = QMenu("Grid Layout", m)
        for g in ("Off", "2×2", "3×3", "4×4"):  # noqa: RUF001
            a = QAction(g, self)
            a.setCheckable(True)
            a.setEnabled(False)
            grid_menu.addAction(a)
        m.addMenu(grid_menu)

        self.addMenu(m)

    # ── Analysis ─────────────────────────────────────────────────────────────

    def _build_analysis(self) -> None:
        m = QMenu("Analysis", self)
        for label in ("Measure Distance", "Measure Angle", "Superpose", "Mutagenesis", "Label Atoms"):
            m.addAction(_stub(self, label))
        self.addMenu(m)

    # ── Bookmarks ────────────────────────────────────────────────────────────

    def _build_bookmarks(self) -> None:
        m = QMenu("Bookmarks", self)

        for label in ("Save View", "Insert Before", "Insert After", "Delete"):
            m.addAction(_stub(self, label))

        m.addSeparator()

        for i in range(1, 7):
            a = QAction(f"View {i}", self)
            a.setShortcut(QKeySequence(f"F{i}"))
            a.setEnabled(False)
            m.addAction(a)

        self.addMenu(m)

    # ── Plugins ──────────────────────────────────────────────────────────────

    def _build_plugins(self) -> None:
        m = QMenu("Plugins", self)
        m.addAction(_stub(self, "Plugin Manager…"))
        m.addSeparator()
        self.addMenu(m)

    # ── Help ─────────────────────────────────────────────────────────────────

    def _build_help(self) -> None:
        m = QMenu("Help", self)

        contents = QAction("Documentation", self)
        contents.setShortcut(QKeySequence.StandardKey.HelpContents)
        contents.setEnabled(False)
        m.addAction(contents)

        m.addSeparator()

        self.about_action = QAction("About Cordyceps…", self)
        m.addAction(self.about_action)

        report = QAction("Report Issue…", self)
        report.setEnabled(False)
        m.addAction(report)

        self.addMenu(m)
