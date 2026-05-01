import pytest
from pytestqt.qtbot import QtBot

from cordyceps.ui.main_window import MainWindow
from cordyceps.ui.menu import Menu
from cordyceps.ui.scene import WgpuWidget


@pytest.mark.ui
def test_main_window_creation(qtbot: QtBot) -> None:
    window = MainWindow()
    qtbot.addWidget(window)
    assert window.windowTitle() == "Cordyceps"
    assert not window.isVisible()


@pytest.mark.ui
def test_main_window_has_central_widget(qtbot: QtBot) -> None:
    window = MainWindow()
    qtbot.addWidget(window)
    assert window.centralWidget() is not None


@pytest.mark.ui
def test_main_window_has_menu_bar(qtbot: QtBot) -> None:
    window = MainWindow()
    qtbot.addWidget(window)
    assert isinstance(window.menuBar(), Menu)


@pytest.mark.ui
def test_wgpu_widget_initial_state(qtbot: QtBot) -> None:
    widget = WgpuWidget()
    qtbot.addWidget(widget)
    assert widget.renderer is None
    assert widget.minimumWidth() == 640
    assert widget.minimumHeight() == 480


@pytest.mark.gpu
@pytest.mark.ui
def test_main_window_show(qtbot: QtBot) -> None:
    window = MainWindow()
    qtbot.addWidget(window)
    window.show()
    assert window.isVisible()
