#!/usr/bin/env python
# coding: utf-8

# In[1]:


from typing import Any, Callable, Generator, Iterable

# In[2]:


sample="""190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"""


# In[3]:


with open('input.txt', 'r', encoding='utf-8') as f:
    content = f.read()


# In[4]:


def mapl[T](fn: Callable, it: Iterable[T]) -> list[T]:
    return list(map(fn, it))


# In[5]:


def pairwise(it: Iterable) -> Generator:
    for n, i in enumerate(it):
        if n + 1 == len(it):
            break
        yield it[n], it[n + 1]


# In[6]:


def product(it: Iterable, repeat: int) -> Generator:
    pools = [tuple(it)] * repeat
    result = [[]]
    for pool in pools:
        result = [x + [y] for x in result for y in pool]
    for r in result:
        yield(r)


# In[7]:


operations = ['+', '*']
equations = [{'result': int(l.split(':')[0]), 'values': mapl(int, l.split(':')[1].split())} for l in content.split('\n') if ':' in l]


# In[8]:


def tryops(eqs, ops):
    solvable = []
    for e in eqs:
        possible_seqs = [[None] + opseq for opseq in product(ops, len(e['values']) - 1)]
        eq_done = False
        while True:
            test_result = 0
            for n1, opseq in enumerate(possible_seqs):
                for n2, v in enumerate(e['values']):
                    if n2 == 0:
                        test_result = v
                        continue
                    if opseq[n2] == '+':
                        test_result += v
                    elif opseq[n2] == '*':
                        test_result *= v
                if test_result == e['result']:
                    eq_done = True
                    solvable.append({'values': e['values'].copy(), 'result': test_result, 'opseq': opseq})
            else:
                eq_done = True
            if eq_done:
                break
    return solvable


# In[12]:


solvable = tryops(equations, operations)


# In[11]:


print(sum(set(group['result'] for group in solvable)))


# In[ ]:




