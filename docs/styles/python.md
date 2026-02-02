# Python Style Guide

This document is a Python coding style guide.

For naming conventions, follow [PEP 8](https://peps.python.org/pep-0008/#naming-conventions).

---

## Part 1: Essential

### 1. Project Setup and Module Structure

#### Principles
- Use `uv` and `pyproject.toml` for project management
- Follow a clear package hierarchy
- Separate concerns into distinct modules
- Re-export public APIs explicitly

#### Rules
- Use `pyproject.toml` as the single source of project configuration
- Use `uv` for dependency management and virtual environments
- Choose `hatchling` or `uv_build` as the build backend
- Organize packages by responsibility: `core/`, `lib/`, `utils/`, `config/`
- Define public APIs in `__init__.py` with explicit imports
- Create `__main__.py` for CLI entry points
- Keep modules focused on a single responsibility
- Use relative imports within the same package, absolute imports for external packages

#### Build Backend Selection
| Backend | Use Case |
|---------|----------|
| `uv_build` | Pure Python projects, fast builds, tight uv integration |
| `hatchling` | Flexible configuration, plugins, mature ecosystem |
| `setuptools` | Legacy projects, C extensions |

#### Code Review Checklist
- [ ] Is `pyproject.toml` used for project configuration?
- [ ] Are dependencies properly separated (runtime vs dev vs optional)?
- [ ] Does each module have a single responsibility?
- [ ] Are public APIs explicitly exported in `__init__.py`?

---

### 2. Error Handling

#### Principles
- Define specific exception types
- Catch exceptions at appropriate levels
- Provide informative error messages

#### Rules
- Define custom exceptions inheriting from `Exception`
- Include descriptive error messages with context
- Use specific exception types, not bare `except:`
- Re-raise exceptions with context using `raise ... from e`

#### Good Example
```python
class ConfigError(Exception):
    """Configuration related errors."""

def load_config(path: str) -> dict:
    try:
        with open(path) as f:
            return yaml.safe_load(f)
    except FileNotFoundError as e:
        raise ConfigError(f"config not found: {path}") from e
```

#### Code Review Checklist
- [ ] Are custom exceptions defined for domain-specific errors?
- [ ] Do error messages include sufficient context?
- [ ] Is `raise ... from e` used to preserve the exception chain?

---

### 3. Functions and Methods

#### Principles
- Functions should do one thing well
- Use descriptive parameter names
- Prefer explicit over implicit

#### Rules
- Limit function length to maintain readability
- Use keyword-only arguments for clarity: `def func(*, required_kwarg)`
- Provide default values for optional parameters
- Return early to avoid deep nesting
- Use `@functools.wraps` in decorators to preserve metadata

#### Good Example
```python
def process_file(path: str, *, encoding: str = "utf-8") -> list[str]:
    if not Path(path).exists():
        raise FileNotFoundError(f"not found: {path}")
    return Path(path).read_text(encoding=encoding).splitlines()
```

#### Decorator Pattern
```python
from functools import wraps

def retry(max_attempts: int = 3):
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            for attempt in range(max_attempts):
                try:
                    return func(*args, **kwargs)
                except Exception:
                    if attempt == max_attempts - 1:
                        raise
        return wrapper
    return decorator
```

#### Code Review Checklist
- [ ] Does each function have a single responsibility?
- [ ] Are keyword-only arguments used where appropriate?
- [ ] Is `@functools.wraps` used in decorators?
- [ ] Is early return used to reduce nesting?

---

### 4. Concurrency

- I/O-bound: `async`/`await`
- CPU-bound:
  - Pure Python workloads: `ProcessPoolExecutor` or `multiprocessing` (CPython GIL prevents true CPU parallelism with threads)
  - Workloads using C extensions that release the GIL: `ThreadPoolExecutor` can be effective
- Don't trust AI when developing concurrent architectures. Add experienced team members as reviewers.

---

## Part 2: Reference

### 5. Type Hints

#### Principles
- Use modern Python 3.10+ type hint syntax
- Use type hints for all public APIs
- Leverage static type checking for early error detection

#### Rules
- Use built-in generics: `list[str]`, `dict[str, Any]`, `tuple[int, ...]`
- Use `X | None` for nullable values (not `Optional[X]`)
- Use `X | Y` for union types (not `Union[X, Y]`)
- Add type hints to all function parameters and return values
- Use `Any` sparingly and only when truly necessary

#### Type Hint Syntax
| Case | Recommended Syntax | Legacy |
|-----------|---------------|---------------|
| Nullable value | `str \| None` | `Optional[str]` |
| Union types | `str \| int` | `Union[str, int]` |
| Dictionary | `dict[str, Any]` | `Dict[str, Any]` |
| List | `list[int]` | `List[int]` |

#### Type Aliases
| Python Version | Syntax |
|----------------|--------|
| 3.12+ | `type Vector = list[float]` |
| 3.10-3.11 | `Vector: TypeAlias = list[float]` |

#### Good Example
```python
from collections.abc import Callable, Iterator
from typing import TypeAlias

def find_files(root: str, pattern: str) -> Iterator[str]:
    ...

# Type alias (Python 3.10-3.11)
Vector: TypeAlias = list[float]

# Type alias (Python 3.12+)
type Handler = Callable[[str], bool]
```

#### Code Review Checklist
- [ ] Is modern type syntax used (`list`, `dict`, `|`)?
- [ ] Do all public functions have type hints?
- [ ] Is `Any` used only where truly necessary?

---

### 6. Python Idioms

#### Principles
- Write idiomatic Python code
- Prefer standard library solutions
- Leverage Python's dynamic features appropriately

#### Rules
- Use `pathlib.Path` instead of string paths
- Use f-strings for string formatting (t-strings in Python 3.14+)
- Use comprehensions for simple transformations
- Prefer `Protocol` over inheritance for structural subtyping
- Leverage duck typing when appropriate

#### pathlib.Path
```python
from pathlib import Path

config_path = Path("config") / "settings.yaml"
if config_path.exists():
    content = config_path.read_text()
```

#### Comprehensions
```python
# List comprehension
names = [user.name for user in users if user.active]

# Dict comprehension
scores = {name: calc_score(name) for name in names}
```

#### Protocol (Structural Subtyping)
```python
from typing import Protocol

class Readable(Protocol):
    def read(self) -> str: ...

def process(source: Readable) -> str:
    return source.read()  # Works with any object that has read()
```

#### t-string (Python 3.14+)
```python
# Template string for safe interpolation
query = t"SELECT * FROM users WHERE name = {name}"
```

#### Code Review Checklist
- [ ] Is `pathlib.Path` used instead of string paths?
- [ ] Are f-strings used for formatting?
- [ ] Are comprehensions used for simple transformations?
- [ ] Is `Protocol` preferred over abstract base classes?

---

### 7. Import Management

#### Principles
- Avoid circular imports
- Import only what is needed
- Make dependencies explicit

#### Rules
- Use lazy imports for expensive modules
- Define `__all__` to control public API
- Break circular imports by moving imports inside functions
- Order imports: stdlib → third-party → local

#### Lazy Import
```python
def process_data(data: dict) -> "DataFrame":
    import pandas as pd  # Lazy import
    return pd.DataFrame(data)
```

#### `__all__` Definition
```python
# mymodule/__init__.py
from .core import Config, load_config
from .errors import ConfigError

__all__ = ["Config", "load_config", "ConfigError"]
```

#### Breaking Circular Imports
```python
# Avoid: top-level circular import
# from .other import OtherClass

def get_other() -> "OtherClass":
    from .other import OtherClass  # Import inside function
    return OtherClass()
```

#### Code Review Checklist
- [ ] Is import order correct (stdlib → third-party → local)?
- [ ] Are expensive imports lazy-loaded when appropriate?
- [ ] Is `__all__` defined for public modules?
- [ ] Are circular imports avoided?

---

### 8. Classes and Dataclasses

#### Principles
- Use `dataclass` for data containers
- Use `attrs` when advanced features are needed
- Keep classes focused on single responsibility

#### Rules
- Use `@dataclass` for simple data containers
- Use `@attrs.define` when you need validators, converters, or slots by default
- Use `frozen=True` for immutable data
- Prefer composition over inheritance
- Use `field(default_factory=...)` for mutable defaults

#### dataclass Example
```python
from dataclasses import dataclass, field

@dataclass
class Job:
    job_id: str
    sql_filepath: str
    upstreams: list[str] = field(default_factory=list)
    timeout: int = 300

@dataclass(frozen=True)
class Config:
    name: str
    version: str = "1.0.0"
```

#### attrs Example (Advanced)
```python
from attrs import define, field, validators

@define
class Job:
    job_id: str = field(validator=validators.instance_of(str))
    timeout: int = field(default=300, converter=int)
```

#### Code Review Checklist
- [ ] Is `@dataclass` used for data container classes?
- [ ] Are mutable defaults using `field(default_factory=...)`?
- [ ] Is `frozen` used for immutable data classes?

---

### 9. Testing

#### Principles
- Write tests that verify behavior, not implementation
- Test names should describe what is being verified
- Keep tests isolated and independent

#### Rules
- Place unit tests in `tests/` directory mirroring source structure
- Use `pytest` for testing
- Test edge cases: empty input, boundary values, error conditions
- Mock external dependencies

#### Good Example
```python
def test_guard_raises_error_on_missing_job_id():
    """Job.guard should raise ValueError when job_id is missing."""
    data = {"sql_filepath": "test.sql"}
    with pytest.raises(ValueError, match="job_id"):
        Job.guard(data)
```

#### Code Review Checklist
- [ ] Does the test name describe what is being verified?
- [ ] Are edge cases tested?
- [ ] Are external dependencies mocked?
- [ ] Is the test testing behavior, not implementation?

---

### 10. Documentation

#### Principles
- Comments explain "why", code explains "what"
- Document public APIs with docstrings
- Keep documentation synchronized with code

#### Rules
- Use Google-style docstrings consistently
- Write docstrings for all public modules, classes, and functions
- Use inline comments for non-obvious logic
- Mark incomplete work with `# TODO:` comments

#### Docstring Style
```python
def load_config(path: str, *, validate: bool = True) -> Config:
    """Load configuration from a YAML file.

    Args:
        path: Path to the configuration file.
        validate: Whether to validate the configuration.

    Returns:
        Loaded configuration object.

    Raises:
        ConfigError: If the file cannot be read or parsed.
    """
```

#### Code Review Checklist
- [ ] Do public APIs have docstrings?
- [ ] Do docstrings include Args, Returns, and Raises sections?
- [ ] Do comments explain "why" not "what"?
