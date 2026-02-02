# Python Style Guide

이 문서는 Python 코딩 스타일 가이드입니다.

네이밍 컨벤션은 [PEP 8](https://peps.python.org/pep-0008/#naming-conventions)을 따릅니다.

---

## Part 1: 꼭 지켜야 하는 것

### 1. 프로젝트 설정과 모듈 구조

#### 원칙
- `uv`와 `pyproject.toml`로 프로젝트를 관리한다
- 명확한 패키지 계층 구조를 따른다
- 관심사를 분리하여 모듈을 구성한다
- 공개 API는 명시적으로 re-export한다

#### 규칙
- `pyproject.toml`을 프로젝트 설정의 단일 소스로 사용한다
- 의존성 관리와 가상 환경에 `uv`를 사용한다
- 빌드 백엔드로 `hatchling` 또는 `uv_build`를 선택한다
- 책임에 따라 패키지를 구성한다: `core/`, `lib/`, `utils/`, `config/`
- `__init__.py`에서 명시적 import로 공개 API를 정의한다
- CLI 진입점은 `__main__.py`에 작성한다
- 모듈은 단일 책임에 집중한다
- 같은 패키지 내에서는 상대 import, 외부 패키지는 절대 import를 사용한다

#### 빌드 백엔드 선택 기준
| 백엔드 | 사용 사례 |
|--------|----------|
| `uv_build` | 순수 Python 프로젝트, 빠른 빌드, uv와의 긴밀한 통합 |
| `hatchling` | 유연한 설정, 플러그인, 성숙한 생태계 |
| `setuptools` | 레거시 프로젝트, C 확장 |

#### 코드 리뷰 체크리스트
- [ ] 프로젝트 설정에 `pyproject.toml`이 사용되었는가?
- [ ] 의존성이 적절히 분리되었는가 (런타임 vs 개발 vs 선택적)?
- [ ] 각 모듈이 단일 책임을 가지는가?
- [ ] `__init__.py`에서 공개 API가 명시적으로 export되어 있는가?

---

### 2. 에러 처리

#### 원칙
- 구체적인 예외 타입을 정의한다
- 적절한 수준에서 예외를 처리한다
- 맥락을 포함한 설명적인 에러 메시지를 제공한다

#### 규칙
- `Exception`을 상속받아 커스텀 예외를 정의한다
- 맥락을 포함한 설명적인 에러 메시지를 작성한다
- bare `except:` 대신 구체적인 예외 타입을 사용한다
- `raise ... from e`를 사용하여 맥락과 함께 예외를 재발생시킨다

#### 좋은 예시
```python
class ConfigError(Exception):
    """설정 관련 에러."""

def load_config(path: str) -> dict:
    try:
        with open(path) as f:
            return yaml.safe_load(f)
    except FileNotFoundError as e:
        raise ConfigError(f"config not found: {path}") from e
```

#### 코드 리뷰 체크리스트
- [ ] 도메인별 에러에 커스텀 예외가 정의되었는가?
- [ ] 에러 메시지에 충분한 맥락이 포함되었는가?
- [ ] 예외 체인 보존을 위해 `raise ... from e`가 사용되었는가?

---

### 3. 함수와 메서드

#### 원칙
- 함수는 한 가지 일을 잘 해야 한다
- 설명적인 매개변수 이름을 사용한다
- 암묵적인 것보다 명시적인 것을 선호한다

#### 규칙
- 가독성 유지를 위해 함수 길이를 제한한다
- 명확성을 위해 키워드 전용 인수를 사용한다: `def func(*, required_kwarg)`
- 선택적 매개변수에 기본값을 제공한다
- 깊은 중첩을 피하기 위해 조기 반환한다
- 데코레이터에서 `@functools.wraps`를 사용하여 메타데이터를 보존한다

#### 좋은 예시
```python
def process_file(path: str, *, encoding: str = "utf-8") -> list[str]:
    if not Path(path).exists():
        raise FileNotFoundError(f"not found: {path}")
    return Path(path).read_text(encoding=encoding).splitlines()
```

#### 데코레이터 패턴
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

#### 코드 리뷰 체크리스트
- [ ] 각 함수가 단일 책임을 가지는가?
- [ ] 키워드 전용 인수가 적절히 사용되었는가?
- [ ] 데코레이터에서 `@functools.wraps`가 사용되었는가?
- [ ] 중첩을 줄이기 위해 조기 반환이 사용되었는가?

---

### 4. 동시성

- I/O 바운드: `async`/`await`
- CPU 바운드:
  - pure Python 연산: `ProcessPoolExecutor` 또는 `multiprocessing` (CPython GIL로 인해 스레드 병렬화 불가)
  - GIL을 해제하는 C 확장 연산: `ThreadPoolExecutor`도 가능
- 동시성 아키텍처를 개발할 때 AI도 믿지 마라. 경험 많은 팀원을 리뷰어로 추가하라.

---

## Part 2: 알아두면 좋은 것

### 5. 타입 힌트

#### 원칙
- 최신 Python 3.10+ 타입 힌트 문법을 사용한다
- 모든 공개 API에 타입 힌트를 사용한다
- 정적 타입 검사를 활용하여 오류를 조기에 발견한다

#### 규칙
- 빌트인 제네릭을 사용한다: `list[str]`, `dict[str, Any]`, `tuple[int, ...]`
- nullable 값에는 `X | None`을 사용한다 (`Optional[X]` 대신)
- 유니온 타입에는 `X | Y`를 사용한다 (`Union[X, Y]` 대신)
- 모든 함수의 매개변수와 반환값에 타입 힌트를 추가한다
- `Any`는 정말 필요한 경우에만 최소한으로 사용한다

#### 타입 힌트 문법
| 상황 | 권장 문법 | 레거시 |
|------|-----------|-------------|
| nullable 값 | `str \| None` | `Optional[str]` |
| 유니온 타입 | `str \| int` | `Union[str, int]` |
| 딕셔너리 | `dict[str, Any]` | `Dict[str, Any]` |
| 리스트 | `list[int]` | `List[int]` |

#### 타입 별칭
| 버전 | 문법 |
|----------------|--------|
| 3.12+ | `type Vector = list[float]` |
| 3.10-3.11 | `Vector: TypeAlias = list[float]` |

#### 좋은 예시
```python
from collections.abc import Callable, Iterator
from typing import TypeAlias

def find_files(root: str, pattern: str) -> Iterator[str]:
    ...

# 타입 별칭 (Python 3.10-3.11)
Vector: TypeAlias = list[float]

# 타입 별칭 (Python 3.12+)
type Handler = Callable[[str], bool]
```

#### 코드 리뷰 체크리스트
- [ ] 최신 타입 문법이 사용되었는가 (`list`, `dict`, `|`)?
- [ ] 모든 공개 함수에 타입 힌트가 있는가?
- [ ] `Any`가 정말 필요한 곳에만 사용되었는가?

---

### 6. Python Idioms

#### 원칙
- 파이썬다운 코드를 작성한다
- 표준 라이브러리 솔루션을 선호한다
- Python의 동적 특성을 적절히 활용한다

#### 규칙
- 문자열 경로 대신 `pathlib.Path`를 사용한다
- 문자열 포맷팅에 f-string을 사용한다 (Python 3.14+에서는 t-string)
- 간단한 변환에는 comprehension을 사용한다
- 상속보다 구조적 서브타이핑을 위해 `Protocol`을 선호한다
- 덕 타이핑을 적절히 활용한다

#### pathlib.Path
```python
from pathlib import Path

config_path = Path("config") / "settings.yaml"
if config_path.exists():
    content = config_path.read_text()
```

#### Comprehensions
```python
# 리스트 컴프리헨션
names = [user.name for user in users if user.active]

# 딕셔너리 컴프리헨션
scores = {name: calc_score(name) for name in names}
```

#### Protocol (구조적 서브타이핑)
```python
from typing import Protocol

class Readable(Protocol):
    def read(self) -> str: ...

def process(source: Readable) -> str:
    return source.read()  # read()가 있는 모든 객체에서 동작
```

#### t-string (Python 3.14+)
```python
# 안전한 보간을 위한 템플릿 문자열
query = t"SELECT * FROM users WHERE name = {name}"
```

#### 코드 리뷰 체크리스트
- [ ] 문자열 경로 대신 `pathlib.Path`가 사용되었는가?
- [ ] 포맷팅에 f-string이 사용되었는가?
- [ ] 간단한 변환에 comprehension이 사용되었는가?
- [ ] 추상 기반 클래스보다 `Protocol`이 선호되었는가?

---

### 7. Import 관리

#### 원칙
- 순환 import를 방지한다
- 필요한 것만 import한다
- 의존성을 명시적으로 만든다

#### 규칙
- 비용이 큰 모듈은 지연 import를 사용한다
- `__all__`을 정의하여 공개 API를 제어한다
- 함수 내부로 import를 이동하여 순환 import를 해결한다
- import 순서: stdlib → third-party → local

#### 지연 Import
```python
def process_data(data: dict) -> "DataFrame":
    import pandas as pd  # 지연 import
    return pd.DataFrame(data)
```

#### `__all__` 정의
```python
# mymodule/__init__.py
from .core import Config, load_config
from .errors import ConfigError

__all__ = ["Config", "load_config", "ConfigError"]
```

#### 순환 Import 해결
```python
# 피하기: 최상위 순환 import
# from .other import OtherClass

def get_other() -> "OtherClass":
    from .other import OtherClass  # 함수 내부에서 import
    return OtherClass()
```

#### 코드 리뷰 체크리스트
- [ ] import 순서가 올바른가 (stdlib → third-party → local)?
- [ ] 비용이 큰 import가 적절히 지연 로드되었는가?
- [ ] 공개 모듈에 `__all__`이 정의되었는가?
- [ ] 순환 import가 방지되었는가?

---

### 8. 클래스와 데이터클래스

#### 원칙
- 데이터 컨테이너에는 `dataclass`를 사용한다
- 고급 기능이 필요하면 `attrs`를 사용한다
- 클래스는 단일 책임에 집중한다

#### 규칙
- 간단한 데이터 컨테이너에는 `@dataclass`를 사용한다
- validators, converters, 기본 slots가 필요하면 `@attrs.define`을 사용한다
- 불변 데이터에는 `frozen=True`를 사용한다
- 상속보다 컴포지션을 선호한다
- 가변 기본값에는 `field(default_factory=...)`를 사용한다

#### dataclass 예시
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

#### attrs 예시 (고급)
```python
from attrs import define, field, validators

@define
class Job:
    job_id: str = field(validator=validators.instance_of(str))
    timeout: int = field(default=300, converter=int)
```

#### 코드 리뷰 체크리스트
- [ ] 데이터 컨테이너 클래스에 `@dataclass`가 사용되었는가?
- [ ] 가변 기본값에 `field(default_factory=...)`가 사용되었는가?
- [ ] 불변 데이터 클래스에 `frozen`이 사용되었는가?

---

### 9. 테스트

#### 원칙
- 구현이 아닌 동작을 검증하는 테스트를 작성한다
- 테스트 이름은 검증 대상을 설명해야 한다
- 테스트는 격리되고 독립적이어야 한다

#### 규칙
- 단위 테스트를 소스 구조를 미러링하는 `tests/` 디렉토리에 배치한다
- 테스트에 `pytest`를 사용한다
- edge case를 테스트한다: 빈 입력, 경계값, 에러 조건
- 외부 의존성을 목(mock)한다

#### 좋은 예시
```python
def test_guard_raises_error_on_missing_job_id():
    """Job.guard는 job_id가 없으면 ValueError를 발생시켜야 한다."""
    data = {"sql_filepath": "test.sql"}
    with pytest.raises(ValueError, match="job_id"):
        Job.guard(data)
```

#### 코드 리뷰 체크리스트
- [ ] 테스트 이름이 검증 대상을 설명하는가?
- [ ] edge case가 테스트되었는가?
- [ ] 외부 의존성이 목(mock)되었는가?
- [ ] 구현이 아닌 동작을 테스트하는가?

---

### 10. 문서화

#### 원칙
- 주석은 "왜"를 설명한다 (코드가 "무엇"을 설명)
- 공개 API를 docstring으로 문서화한다
- 문서를 코드와 동기화한다

#### 규칙
- Google 스타일 docstring을 일관되게 사용한다
- 모든 공개 모듈, 클래스, 함수에 docstring을 작성한다
- 비자명한 로직에 인라인 주석을 사용한다
- 미완성 작업은 `# TODO:` 주석으로 표시한다

#### Docstring 스타일
```python
def load_config(path: str, *, validate: bool = True) -> Config:
    """YAML 파일에서 설정을 로드한다.

    Args:
        path: 설정 파일 경로.
        validate: 설정 검증 여부.

    Returns:
        로드된 설정 객체.

    Raises:
        ConfigError: 파일을 읽거나 파싱할 수 없는 경우.
    """
```

#### 코드 리뷰 체크리스트
- [ ] 공개 API에 docstring이 있는가?
- [ ] docstring에 Args, Returns, Raises 섹션이 포함되었는가?
- [ ] 주석이 "무엇"이 아닌 "왜"를 설명하는가?
