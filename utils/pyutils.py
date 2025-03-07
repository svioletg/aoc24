from collections.abc import Callable, Generator
from copy import deepcopy
from math import floor
from operator import add, floordiv, mod, mul, neg, sub, truediv
from pprint import pprint
from time import perf_counter
from typing import Any, Literal, Optional, Self, overload

from colorama import Back, Fore, Style
from IPython.display import clear_output
from PIL import Image, ImageDraw

Matrix = list[list]
IntMatrix = list[list[int]]
StrMatrix = list[list[str]]
Point = tuple[int, int]

# Misc
def readutf8(fp: str) -> str:
    with open(fp, 'r', encoding='utf-8') as f:
        content = f.read()
    return content

def colored(s: str, colstr: str) -> str:
    """Returns `s` with preceding color codes per `colstr`.

    :param colstr: A string dictating what color to set, in the format "<foreground> on <background>",
        or just "<foreground>", e.g. "red on white" or "blue"
    """
    args = colstr.split('on')
    fg = args[0].strip()
    bg = None
    if len(args) == 2:
        bg = args[1].strip()
    fg = getattr(Fore, fg.upper())
    bg = Back.BLACK if not bg else getattr(Back, bg.upper())
    return f'{fg}{bg}{s}{Style.RESET_ALL}'

class Stopwatch:
    def __init__(self, label: Optional[str] = None):
        self.label: str = label or str(id(label))
        self.ta = perf_counter()

    def lap(self) -> float:
        return perf_counter() - self.ta

    def print_lap(self):
        print(f'Stopwatch {self.label}: {self.lap():.8f}')

# Matrices
def strtomat(s: str, map_fn: Optional[Callable] = None) -> Matrix:
    """Turns a string into a matrix. Will be a matrix of strings unless `map_fn` converts them to something else."""
    mat: Matrix = []
    for row in s.split('\n'):
        if row == '':
            continue
        mat.append(list(row if map_fn is None else map(map_fn, row)))
    return mat

def mat_flat(mat: Matrix) -> list:
    """Flattens a matrix into a single list of elements."""
    return [b for a in mat for b in a]

def mat_restring(mat: Matrix) -> str:
    """Returns a matrix as one single rectangular string."""
    return '\n'.join([''.join(map(str, row)) for row in mat])

def matslice(mat: Matrix, topleft: Point, bottomright: Point, hardfail: bool = False) -> Matrix | None:
    """Returns a slice, or sub-matrix, of the provided matrix, starting from `topleft` and ending at `bottomright`.
    As an example, with the following matrix:
    ```
    mat = \"\"\"
    ABCDE
    FGHIJ
    KLMNO
    \"\"\"
    ```
    Calling `matslice(mat, (0, 1), (2, 3))` on it would result in a matrix like this:
    ```
    BCD
    GHI
    LMN
    ```
    If a corner point provided is out of bounds, returns `None`. If `hardfail` is passed `True`, `IndexError` is raised
    in this case instead.
    """
    if not (point_in_matrix(topleft, mat) and point_in_matrix(bottomright, mat)):
        if hardfail:
            raise IndexError(f'One of the provided corner points are out of bounds: {topleft!r}, {bottomright!r}')
        return None

    start_row, start_col = topleft
    end_row, end_col = bottomright
    submat: Matrix = [[] for _ in range(start_row, end_row + 1)]
    for ridx, row in enumerate(mat[start_row:end_row + 1]):
        submat[ridx] = row[start_col:end_col + 1].copy()

    return submat

def matget(mat: Matrix, pt: Point, allow_oob: bool = False, oob_value: Any = None) -> Any:
    """Returns the value of a given point or points in `mat`.
    If `allow_oob` is `True`, out of bounds values will return the value given to `oob_value` (default `None`).
    """
    if allow_oob:
        if not point_in_matrix(pt, mat):
            return None
    return mat[pt[0]][pt[1]]

def matset(mat: Matrix, pt: Point, val: str):
    """Sets the value of a point in `mat` to `val`."""
    mat[pt[0]][pt[1]] = val

def mat_iter(mat: Matrix) -> Generator[tuple['Pt', Any], None, None]:
    """Iterates over every item in `mat` by every column in every row, as in (row 0, column 0) -> (row 0, column 1) ->"""
    for n1, row in enumerate(mat):
        for n2, col in enumerate(row):
            yield (Pt(n1, n2), col)

def traverse_matrix(mat: StrMatrix, start: Point, step: Point, bidi: bool = False) -> list[Point]:
    """Returns all points of a matrix travelled to by beginning at `start`
    and increasing the point by `step` until out of bounds.

    :param bidi: "Bidirectional"; if True, `mat` will be stepped through in both directions from its starting point.
        The returned list will be sorted in ascending order.
    """
    pts: list[Point] = [start]
    cursor = start
    passes = 0
    while True:
        cursor = point_op(add, cursor, step)
        if not point_in_matrix(cursor, mat):
            if bidi and passes < 2:
                passes += 1
                cursor = start
                step = point_op(mul, step, -1)
                continue
            break
        pts.append(cursor)
    pts = sorted(list(set(pts)))
    return pts

def show_in_matrix(mat: Matrix, *pts: Point, col: str = 'white on red', colmap: Optional[dict[Point, str]] = None):
    """Colors the given location(s) in a matrix and returns it restrung."""
    colmap = colmap or {}
    matcopy = deepcopy(mat)
    for pt in list(pts) + list(colmap.keys()):
        matset(matcopy, pt, colored(matget(matcopy, pt), colmap.get(pt, col)))
    return mat_restring(matcopy)

def matimg(mat: Matrix,
        colflt: Optional[dict[Callable[['Pt', Any], bool], str]] = None,
        resize: Optional[tuple[int, int]] = None
    ) -> Image.Image:
    colflt = colflt or {}
    im = Image.new(mode='RGB', size=(len(mat[0]), len(mat)), color='white')
    brush = ImageDraw.Draw(im)
    for pt, val in mat_iter(mat):
        for flt, col in colflt.items():
            if flt(pt, val):
                brush.point((pt[1], pt[0]), col)
    if resize is not None:
        im = im.resize(resize, resample=0)
    return im

# Points, vectors
class Pt:
    def __init__(self, a: int, b: int):
        self.a = a
        self.b = b

    def __repr__(self) -> str:
        return f'Pt({self.a}, {self.b})'

    def __str__(self) -> str:
        return str(self.as_tuple())

    def __hash__(self) -> int:
        return hash(self.as_tuple())

    def __getitem__(self, idx: int) -> int:
        return (self.a, self.b)[idx]

    def __eq__(self, other: 'tuple[int, int] | Pt') -> bool:
        return (self[0] == other[0]) and (self[1] == other[1])

    def __add__(self, other: 'tuple[int, int] | Pt') -> 'Pt':
        return self._math(add, other, 'l')
    def __radd__(self, other: 'tuple[int, int] | Pt') -> 'Pt':
        return self._math(add, other, 'r')
    def __sub__(self, other: 'tuple[int, int] | Pt') -> 'Pt':
        return self._math(sub, other, 'l')
    def __rsub__(self, other: 'tuple[int, int] | Pt') -> 'Pt':
        return self._math(sub, other, 'r')
    def __mul__(self, other: 'tuple[int, int] | Pt') -> 'Pt':
        return self._math(mul, other, 'l')
    def __rmul__(self, other: 'tuple[int, int] | Pt') -> 'Pt':
        return self._math(mul, other, 'r')

    def _math(self, mathop: Callable, other: 'tuple[int, int] | Pt', direction: Literal['l', 'r']) -> 'Pt':
        if direction == 'l':
            return Pt(mathop(self[0], other[0]), mathop(self[1], other[1]))
        if direction == 'r':
            return Pt(mathop(other[0], self[0]), mathop(other[1], self[1]))

    def as_tuple(self) -> tuple[int, int]:
        return (self.a, self.b)

    def distance(self, other: 'tuple[int, int] | Pt') -> int:
        return (abs(self[0] - other[0]) + abs(self[1] - other[1]))

    def turn90(self, counter: bool = False) -> Self:
        cards = self.cardinals()
        idx = (cards.index(self) + 1) if not counter else (cards.index(self) - 1)
        if idx >= len(cards):
            idx -= len(cards)
        return cards[idx]

    @classmethod
    def of(cls, tp: tuple[int, int]) -> Self:
        return cls(tp[0], tp[1])

    @classmethod
    def cardinals(cls) -> tuple[Self, Self, Self, Self]:
        return (cls(-1, 0), cls(0, 1), cls(1, 0), cls(0, -1))

    @classmethod
    def cards8(cls) -> tuple[Self, Self, Self, Self, Self, Self, Self, Self]:
        return (
            cls(-1, 0), cls(-1, 1),
            cls(0, 1), cls(1, 1),
            cls(1, 0), cls(1, -1),
            cls(0, -1), cls(1, -1)
        )

def point_op(fn: Callable, a: Point, b: int | Point) -> Point:
    """Calls `fn` for each pair of vector values and returns a new vector with the result.
    If `b` is a single integer, it will be converted to a point (e.g. `1` -> `(1, 1)`)

    e.g. `vecop(operator.add, (10, 5), (20, 5))` returns `(30, 10)`
    """
    if isinstance(b, int):
        b = (b, b)
    return (fn(a[0], b[0]), fn(a[1], b[1]))

def point_from_index(idx: int, w: int, h: int, xy: bool = False) -> Point:
    """Returns a `Point` converted from an index based on a matrix's given width and height.
    In a matrix with 4 rows and 4 columns, an index of `6` returns `(1, 2)` (as ROW,COLUMN).

    :param xy: If `True`, gives the converted `Point` back in X,Y form rather than ROW,COLUMN form, i.e. horizontal first.
    """
    pt = (idx - w, floor(idx / h))
    return pt if not xy else (pt[1], pt[0])

def index_from_point(pt: Point, w: int, xy: bool = False) -> int:
    """Returns an index of a matrix converted from the given `Point` based on its width and the point's `y`.
    In a matrix with 4 rows and 4 columns, the point `(1, 2)` (as ROW,COLUMN) returns `6`.

    :param xy: Treats `pt` as X,Y form rather than ROW,COLUMN.
    """
    x, y = (pt[0] if xy else pt[1], pt[1] if xy else pt[0])
    return x + (w * y)

def point_in_matrix(pt: Point, mat: Matrix) -> bool:
    """Returns whether `pt` is within the bounds of `mat`."""
    return (pt[0] in range(0, len(mat))) and (pt[1] in range(0, len(mat[0])))

def points_adjacent(pt: Point, mat: Matrix,
        allow_oob: bool = True,
        oob_value: Any = None,
        corners: bool = True,
        relative: bool = False
    ) -> dict[Point, Any]:
    """Returns a dictionary of points adjacent to `pt` and their values in `mat`.
    If any adjacent points would be outside the bounds of the matrix (i.e. if the point is on any "edges"),
    those points will still be present in the returned dictionary, however their values will be equal to `oob_value` (`None` by default).

    :param oob_value: What value to assign points which are outside the bounds of the matrix, if they exist.
    :param allow_oob: Whether to let the function return a dictionary with out-of-bounds points. If `False`, `IndexError` is raised
        upon encountering an out-of-bounds point.
    :param corners: Whether to include adjacent corners in the returned dictionary. Passing `False` will only return points directly "touching" `pt`.
    :param relative: Whether the point keys in the returned dictionary should be relative to
        the original point given, or the actual points as they are in the matrix.
        e.g. with `pt` as `(2, 2)`, `mat` as a 3x3 matrix of numbers 1-9, and relative as `True`, the returned dictionary will look this...
        ```
        { (-1, -1): 1,  (-1, 0): 2,  (-1, 1): 3,
           (0, -1):  4,  (0, 0):  5,  (0, 1):  6,
           (1, -1):  7,  (1, 0):  8,  (1, 1):  9 }
        ```
        ...where passing `relative` as `False` would result in this instead:
        ```
        { (0, 0): 1, (0, 1): 2, (0, 2): 3,
          (1, 0): 4, (1, 1): 5, (1, 2): 6,
          (2, 0): 7, (2, 1): 8, (2, 2): 9 }
        ```
    """
    adj_pts: list[Point] = [
        (-1, -1), (-1, 0), (-1, 1),
         (0, -1),  (0, 0),  (0, 1),
         (1, -1),  (1, 0),  (1, 1)
    ] if corners else [
                (-1, 0),
        (0, -1), (0, 0), (0, 1),
                 (1, 0)
    ]
    adj_values: dict[Point, Any] = {}

    for pt_rel in adj_pts:
        pt_abs = point_op(add, pt_rel, pt)
        pt_key = pt_rel if relative else pt_abs
        val = matget(mat, pt_abs, allow_oob=True)
        if val is None:
            if not allow_oob:
                raise IndexError(f'Unallowed access of an out-of-bounds point in matrix not allowed: {pt_abs!r}')
        adj_values[pt_key] = val

    return adj_values
