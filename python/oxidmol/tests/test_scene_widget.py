"""Lifecycle of the 3-D viewport widget.

Most tests replace the Rust renderer with a fake one, so they need no GPU and run in CI.
The last test starts the real renderer and is marked ``gpu``.
"""

import pytest
from pytestqt.qtbot import QtBot

from oxidmol import lsd
from oxidmol.ui import scene
from oxidmol.ui.scene import WgpuWidget


class FakeRenderer:
    """Records what the widget asks of the renderer."""

    instances: list["FakeRenderer"] = []
    fail_on_create = False
    fail_on_render = False
    fail_on_load = False

    def __init__(self, window_handle: int, *, width: int, height: int) -> None:
        if FakeRenderer.fail_on_create:
            raise RuntimeError("no adapter")
        self.window_handle = window_handle
        self.size = (width, height)
        self.resizes: list[tuple[int, int]] = []
        self.frames = 0
        self.loaded: list[str] = []
        FakeRenderer.instances.append(self)

    def resize(self, width: int, height: int) -> None:
        self.resizes.append((width, height))

    def render(self) -> None:
        if FakeRenderer.fail_on_render:
            raise RuntimeError("device lost")
        self.frames += 1

    def load_molecule(self, path: str) -> None:
        if FakeRenderer.fail_on_load:
            raise RuntimeError("bad file")
        self.loaded.append(path)


@pytest.fixture(autouse=True)
def fake_renderer(monkeypatch: pytest.MonkeyPatch) -> None:
    FakeRenderer.instances = []
    FakeRenderer.fail_on_create = False
    FakeRenderer.fail_on_render = False
    FakeRenderer.fail_on_load = False
    monkeypatch.setattr(scene, "WgpuRenderer", FakeRenderer)


def _shown_widget(qtbot: QtBot) -> WgpuWidget:
    widget = WgpuWidget()
    qtbot.addWidget(widget)
    widget.show()
    return widget


def _errors(widget: WgpuWidget) -> list[str]:
    messages: list[str] = []
    widget.error_occurred.connect(messages.append)
    return messages


@pytest.mark.ui
def test_renderer_is_created_on_first_show_with_the_native_handle(qtbot: QtBot) -> None:
    widget = WgpuWidget()
    qtbot.addWidget(widget)
    assert widget.renderer is None

    widget.show()

    assert len(FakeRenderer.instances) == 1
    assert widget.renderer is FakeRenderer.instances[0]
    assert widget.renderer.window_handle == int(widget.winId())


@pytest.mark.ui
def test_surface_size_is_in_device_pixels(qtbot: QtBot, monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setattr(WgpuWidget, "devicePixelRatioF", lambda _self: 2.0)
    widget = _shown_widget(qtbot)

    assert FakeRenderer.instances[0].size == (widget.width() * 2, widget.height() * 2)

    widget.resize(700, 500)
    assert FakeRenderer.instances[0].resizes[-1] == (widget.width() * 2, widget.height() * 2)


@pytest.mark.ui
def test_frames_are_drawn_only_while_visible(qtbot: QtBot) -> None:
    widget = _shown_widget(qtbot)
    renderer = FakeRenderer.instances[0]

    qtbot.waitUntil(lambda: renderer.frames > 0, timeout=2000)
    widget.hide()
    frames_when_hidden = renderer.frames
    qtbot.wait(100)
    assert renderer.frames == frames_when_hidden

    widget.show()
    qtbot.waitUntil(lambda: renderer.frames > frames_when_hidden, timeout=2000)
    assert len(FakeRenderer.instances) == 1, "showing again must not create a second renderer"


@pytest.mark.ui
def test_release_renderer_stops_drawing_and_is_idempotent(qtbot: QtBot) -> None:
    widget = _shown_widget(qtbot)
    renderer = FakeRenderer.instances[0]
    qtbot.waitUntil(lambda: renderer.frames > 0, timeout=2000)

    widget.release_renderer()
    widget.release_renderer()
    frames = renderer.frames
    qtbot.wait(100)

    assert widget.renderer is None
    assert renderer.frames == frames


@pytest.mark.ui
def test_failed_init_reports_the_error_and_is_not_retried(qtbot: QtBot) -> None:
    FakeRenderer.fail_on_create = True
    widget = WgpuWidget()
    qtbot.addWidget(widget)
    errors = _errors(widget)

    widget.show()
    widget.hide()
    widget.show()

    assert widget.renderer is None
    assert len(errors) == 1
    assert "no adapter" in errors[0]


@pytest.mark.ui
def test_render_error_stops_the_frame_loop_after_one_report(qtbot: QtBot) -> None:
    FakeRenderer.fail_on_render = True
    widget = WgpuWidget()
    qtbot.addWidget(widget)
    errors = _errors(widget)

    widget.show()
    qtbot.waitUntil(lambda: len(errors) > 0, timeout=2000)
    qtbot.wait(100)

    assert len(errors) == 1, "a failing frame must not be reported 60 times a second"
    assert "device lost" in errors[0]


@pytest.mark.ui
def test_load_file_emits_basename_and_reports_failures(qtbot: QtBot) -> None:
    widget = _shown_widget(qtbot)
    errors = _errors(widget)

    with qtbot.waitSignal(widget.molecule_loaded) as signal:
        widget.load_file("/some/dir/protein.pdb")
    assert signal.args == ["protein.pdb"]

    FakeRenderer.fail_on_load = True
    widget.load_file("/some/dir/broken.pdb")
    assert len(errors) == 1
    assert "broken.pdb" in errors[0]


@pytest.mark.ui
def test_load_file_before_the_renderer_exists_is_ignored(qtbot: QtBot) -> None:
    widget = WgpuWidget()
    qtbot.addWidget(widget)
    loaded: list[str] = []
    widget.molecule_loaded.connect(loaded.append)

    widget.load_file("/some/dir/protein.pdb")

    assert loaded == []


@pytest.mark.gpu
@pytest.mark.ui
def test_real_renderer_starts_draws_and_resizes(qtbot: QtBot, monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setattr(scene, "WgpuRenderer", lsd.WgpuRenderer)  # the real one, undo the autouse fake
    widget = WgpuWidget()
    qtbot.addWidget(widget)
    errors = _errors(widget)

    widget.show()
    qtbot.waitUntil(lambda: widget.renderer is not None or bool(errors), timeout=5000)
    assert errors == []
    assert widget.renderer is not None
    assert (widget.renderer.width, widget.renderer.height) == widget._pixel_size()

    widget.renderer.render()
    widget.resize(800, 600)
    assert (widget.renderer.width, widget.renderer.height) == widget._pixel_size()
    widget.renderer.render()
    assert errors == []
