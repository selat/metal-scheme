#!/usr/bin/python3

import subprocess
import collections

class OrderedSet(collections.MutableSet):

    def __init__(self, iterable=None):
        self.end = end = []
        end += [None, end, end]         # sentinel node for doubly linked list
        self.map = {}                   # key --> [key, prev, next]
        if iterable is not None:
            self |= iterable

    def __len__(self):
        return len(self.map)

    def __contains__(self, key):
        return key in self.map

    def add(self, key):
        if key not in self.map:
            end = self.end
            curr = end[1]
            curr[2] = end[1] = self.map[key] = [key, curr, end]

    def discard(self, key):
        if key in self.map:
            key, prev, next = self.map.pop(key)
            prev[2] = next
            next[1] = prev

    def __iter__(self):
        end = self.end
        curr = end[2]
        while curr is not end:
            yield curr[0]
            curr = curr[2]

    def __reversed__(self):
        end = self.end
        curr = end[1]
        while curr is not end:
            yield curr[0]
            curr = curr[1]

    def pop(self, last=True):
        if not self:
            raise KeyError('set is empty')
        key = self.end[1][0] if last else self.end[2][0]
        self.discard(key)
        return key

    def __repr__(self):
        if not self:
            return '%s()' % (self.__class__.__name__,)
        return '%s(%r)' % (self.__class__.__name__, list(self))

    def __eq__(self, other):
        if isinstance(other, OrderedSet):
            return len(self) == len(other) and list(self) == list(other)
        return set(self) == set(other)


process = subprocess.Popen(['cargo', 'run', '-q', '--', '--list-functions'], stdout=subprocess.PIPE)
out, err = process.communicate()
out = out.decode("utf-8")
implemented_functions = OrderedSet()
implemented_num = 0
for x in iter(out.splitlines()):
    implemented_functions.add(x)

all_functions = OrderedSet()
fn_list = open('functions-list.txt')
for fn in iter(fn_list.read().splitlines()):
    all_functions.add(fn)
    if fn in implemented_functions:
        implemented_num = implemented_num + 1

print('# of implementd functions: ' + str(implemented_num) + '/' + str(all_functions.__len__()))

markdown_out = open('README.md', 'w')
markdown_out.write('Scheme Lisp interpreter written in Rust.\n\n')
markdown_out.write('Overall, ' + '%.2f' %(float(implemented_num) / float(all_functions.__len__()) * 100.0)
                   + '% of r5rs\'s standard functions are implemented.\n\n')
markdown_out.write('| Function | Implemented | Tests passing|\n')
markdown_out.write('| :----- | :-----: | :-----: |\n')
for x in all_functions:
    markdown_out.write('|' + x + '|')
    if x in implemented_functions:
        markdown_out.write(' **+** |')
    else:
        markdown_out.write(' **-** |')
    if x in implemented_functions:
        markdown_out.write(' **+**  |')
    else:
        markdown_out.write(' **-** |')
    markdown_out.write('\n')
