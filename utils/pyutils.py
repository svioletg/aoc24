from copy import deepcopy
from operator import add, sub, mul, truediv, floordiv, mod, neg
from pprint import pprint
from typing import Callable, Literal

from IPython.display import clear_output
from colorama import Fore, Back, Style

StrMatrix = list[list[str]]
Point = tuple[int, int]

def colored(s: str, colstr: str) -> str:
    """Returns `s` with its colored changed per `colstr`.
    
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

# Matrices
def strtomat(s: str) -> StrMatrix:
    """Turns a string into a matrix - a list of lists of strings."""
    mat: StrMatrix = []
    for row in s.split('\n'):
        mat.append(list(row))
    return mat

def mat_restring(mat: StrMatrix) -> str:
    """Returns a matrix of strings as one single string."""
    return '\n'.join([''.join(row) for row in mat])

def matget(mat: StrMatrix, loc: Point) -> str:
    """Returns the value of a given point or points in `mat`."""
    return mat[loc[0]][loc[1]]

def matset(mat: StrMatrix, loc: Point, val: str):
    """Sets the value of a point in `mat` to `val`."""
    mat[loc[0]][loc[1]] = val

def traverse_matrix(mat: StrMatrix, start: Point, step: Point, bidi: bool=False) -> list[Point]:
    """Returns all points of a matrix travelled to by beginning at `start`
    and increasing the point by `step` until out of bounds.

    :param bidi: "Bidirectional"; if True, `mat` will be stepped through in both directions from its starting point.
        The returned list will be sorted in ascending order.
    """
    maxrows = len(mat) - 1
    maxcols = len(mat[0]) - 1
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

def show_in_mat(mat: StrMatrix, *locs: tuple[int, int]):
    """Colors the given location(s) in a string matrix and returns it restrung."""
    matcopy = deepcopy(mat)
    for loc in locs:
        matset(matcopy, loc, colored(matget(matcopy, loc), 'white on red'))
    return mat_restring(matcopy)

# Points, vectors
def point_op(fn: Callable, a: Point, b: int | Point) -> Point:
    """Calls `fn` for each pair of vector values and returns a new vector with the result.
    If `b` is a single integer, it will be converted to a point (e.g. `1` -> `(1, 1)`)

    e.g. `vecop(operator.add, (10, 5), (20, 5))` returns `(30, 10)`
    """
    if isinstance(b, int):
        b = (b, b)
    return (fn(a[0], b[0]), fn(a[1], b[1]))

def point_in_matrix(pt: Point, mat: StrMatrix) -> bool:
    """Returns whether `pt` is within the bounds of `mat`."""
    return (pt[0] in range(0, len(mat))) and (pt[1] in range(0, len(mat[1])))
