import tomllib
from pathlib import Path
from typing import Self

import tomli_w
from pydantic import BaseModel, Field, field_validator

SETTINGS_PATH = Path.home() / ".oxidmol" / "settings.toml"

Color = tuple[float, float, float]


class RenderSettings(BaseModel):
    background_color: Color = (0.05, 0.05, 0.08)
    ambient: float = Field(0.2, ge=0.0, le=1.0)
    diffuse: float = Field(0.8, ge=0.0, le=1.0)
    specular: float = Field(0.3, ge=0.0, le=1.0)
    shininess: float = Field(32.0, gt=0.0)
    depth_cue: bool = True
    antialias: int = Field(2, ge=1, le=4)

    # Можно добавить валидатор для конкретных значений
    @field_validator("antialias")
    @classmethod
    def antialias_must_be_power_of_two(cls, v: int) -> int:
        if v not in (1, 2, 4):
            raise ValueError("antialias must be 1, 2 or 4")
        return v


class RepresentationSettings(BaseModel):
    sphere_scale: float = Field(1.0, gt=0.0)
    stick_radius: float = Field(0.15, gt=0.0)
    ribbon_width: float = Field(2.0, gt=0.0)
    ribbon_sampling: int = Field(10, ge=1, le=100)
    surface_quality: int = Field(1, ge=0, le=2)


class FetchSettings(BaseModel):
    default_format: str = Field("bcif", pattern="^(bcif|cif|pdb)$")
    timeout: int = Field(30, ge=1)
    fallback_to_pdbe: bool = True


class SessionSettings(BaseModel):
    auto_zoom: bool = True
    auto_show: str = Field("cartoon", pattern="^(cartoon|sticks|spheres|lines|surface)$")


class Settings(BaseModel):
    render: RenderSettings = Field(default_factory=RenderSettings)
    representation: RepresentationSettings = Field(default_factory=RepresentationSettings)
    fetch: FetchSettings = Field(default_factory=FetchSettings)
    session: SessionSettings = Field(default_factory=SessionSettings)

    model_config = {"validate_assignment": True}  # валидация при set

    @classmethod
    def load(cls) -> Self:
        """Load from file, fall back to defaults if not found."""
        if not SETTINGS_PATH.exists():
            return cls()

        with open(SETTINGS_PATH, "rb") as f:
            data = tomllib.load(f)

        # model_validate заменяет весь наш _load_toml + _coerce
        return cls.model_validate(data)

    def get(self, key: str) -> object:
        """
        Get setting value by dotted key.

        get("render.ambient") → 0.2
        """
        group_name, attr = self._split(key)
        return getattr(getattr(self, group_name), attr)

    def set(self, key: str, value: object) -> None:
        """
        Set setting value with validation, then save to file.

        Raises ValidationError with clear message if value is invalid.
        """
        group_name, attr = self._split(key)
        group = getattr(self, group_name)

        setattr(group, attr, value)
        self.save()

    def save(self) -> None:
        """Write current settings to TOML file."""
        SETTINGS_PATH.parent.mkdir(parents=True, exist_ok=True)
        with open(SETTINGS_PATH, "wb") as f:
            tomli_w.dump(self.model_dump(), f)

    def reset(self, key: str | None = None) -> None:
        """
        Reset to defaults.

        reset()                  — everything
        reset("render")          — one group
        reset("render.ambient")  — one value
        """
        if key is None:
            new = Settings()
            for group_name in ("render", "representation", "fetch", "session"):
                setattr(self, group_name, getattr(new, group_name))

        elif "." not in key:
            group_cls = type(getattr(self, key))
            setattr(self, key, group_cls())

        else:
            group_name, attr = self._split(key)
            group = getattr(self, group_name)
            default = getattr(type(group)(), attr)
            setattr(group, attr, default)

        self.save()

    def _split(self, key: str) -> tuple[str, str]:
        parts = key.split(".", 1)
        if len(parts) != 2:
            raise KeyError(f"Key must be 'group.setting', got: '{key}'")
        group_name, attr = parts
        if not hasattr(self, group_name):
            raise KeyError(f"Unknown settings group: '{group_name}'")
        if not hasattr(getattr(self, group_name), attr):
            raise KeyError(f"Unknown setting: '{key}'")
        return group_name, attr


settings = Settings.load()
