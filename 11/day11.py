#!/usr/bin/env python
# coding: utf-8

# In[1]:


import sys
import math
import time

sys.path.append('../utils')
from pyutils import *


# In[2]:


puzzle_part = 2 if sys.argv[1] == 'b' else 1
#puzzle_part = 1 # have to do this manually for notebook


# In[3]:


sample = "125 17"


# In[4]:


with open('input.txt', 'r', encoding='utf-8') as f:
    puzzle = f.read()


# In[5]:


stones = [(int(i), 0) for i in puzzle.split()]


# In[6]:


def digits(num: int) -> int:
    return math.floor(math.log(num, 10)) + 1


# In[11]:


def process_stones(stones: list[tuple[int, int]]) -> int:
    stones = stones.copy()
    MAX_BLINK = 25 if puzzle_part == 1 else 75

    n = 0
    stonecount = len(stones)
    def _blink(n: int):
        nonlocal stonecount
        st, level = stones.pop(n)

        if st == 0:
            stones.insert(n, (1, level + 1))
        elif digits(st) % 2 == 0:
            spl = int(str(st)[:digits(st) // 2]), int(str(st)[digits(st) // 2:])
            stones.insert(n, (spl[0], level + 1))
            stones.insert(n + 1, (spl[1], level + 1))
            stonecount += 1
        else:
            stones.insert(n, (st * 2024, level + 1))

    while n < len(stones):
        while stones[n][1] < MAX_BLINK:
            _blink(n)
            # time.sleep(0.005)
            # clear_output(wait=True)
            # print(stonecount, stones)
        if stones[n][1] == MAX_BLINK:
            stones.pop(n)
            continue
        n += 1

    return stonecount


# In[12]:


ta = time.perf_counter()
process_stones(stones)
tb = time.perf_counter()
print(f'Finished in {tb - ta:.05f}')


# In[ ]:




