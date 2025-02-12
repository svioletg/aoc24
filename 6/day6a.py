#!/usr/bin/env python
# coding: utf-8

# In[1]:


from copy import deepcopy
from time import sleep

from IPython.display import clear_output

# In[2]:


sample="""....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."""


# In[3]:


with open('input.txt', 'r', encoding='utf-8') as f:
    content = f.read()


# In[4]:


def restring(mat: list[list[str]]) -> str:
    return '\n'.join([''.join(row) for row in mat])


# In[5]:


def turn_vec_90deg(vec: tuple[int, int]) -> tuple[int, int]:
    match vec:
        case (-1, 0):
            return (0, 1)
        case (0, 1):
            return (1, 0)
        case (1, 0):
            return (0, -1)
        case (0, -1):
            return (-1, 0)
    raise ValueError(f'Unexpected: {vec!r}')


# In[28]:


def predict(mat: list[list[str]],
            guard_chars: dict[str, tuple[int, int]],
            obj_char: str = '#',
            fill_char: str = '.',
            visited_char: str = 'X',
            show: bool = False
           ) -> list[list[str]]:
    """
    :param guard_chars: Dictionary of characters to 2-integer tuples representing a direction as (row, column), i.e.
        {'^': (-1, 0), '>': (0, 1), 'v': (1, 0), '<': (0, -1)}
    """
    srcmat = mat
    mat = deepcopy(srcmat)
    guard_chars_bydir = {v:k for k, v in guard_chars.items()}

    # Find guard
    guard: dict = {'char': None, 'pos': None}
    for row, _ in enumerate(mat):
        for col, _ in enumerate(mat[row]):
            if mat[row][col] in guard_chars:
                guard['char'] = mat[row][col]
                guard['pos'] = [row, col]
                break
        else: # First time i've used else in a for-loop, strange choice of word imo but i guess it worked
            continue
        break
    assert(guard['pos'] is not None)

    # Simulate path
    guard_in_map = True
    while True:
        if show:
            clear_output(wait=True)
            print(restring(mat))
            sleep(0.1)
        direction = guard_chars[guard['char']]
        while True:
            # Check future pos, turn 90 degrees until there's no obstruction in front of guard
            future = (guard['pos'][0] + direction[0], guard['pos'][1] + direction[1])
            if (future[0] not in range(0, len(mat))) or (future[1] not in range(0, len(mat[0]))):
                # Guard has exited map
                print(future)
                guard_in_map = False
                break
            print(future, len(mat), len(mat[0]))
            if mat[future[0]][future[1]] == obj_char:
                # Turn
                direction = turn_vec_90deg(direction)
                guard['char'] = guard_chars_bydir[direction]
            else:
                break
        if not guard_in_map:
            break
        # Turning done, we can move forward
        mat[guard['pos'][0]][guard['pos'][1]] = visited_char
        guard['pos'] = list(future)
        mat[guard['pos'][0]][guard['pos'][1]] = guard['char']
    mat[guard['pos'][0]][guard['pos'][1]] = visited_char
    return mat


# In[30]:


matrix: list[list[str]] = [list(line) for line in content.split('\n')]
guard_chars = {'^': (-1, 0), '>': (0, 1), 'v': (1, 0), '<': (0, -1)}
result: str = restring(predict(matrix, guard_chars, show=False))
print(result.count('X'))


# In[ ]:




