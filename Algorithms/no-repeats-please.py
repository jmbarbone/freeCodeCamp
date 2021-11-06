import unittest
import itertools


def perm_alone(x):
    letters = list(x)
    n = len(letters)
    
    if n == 1 :
        return 1
    
    if len(set(letters)) == 1 :
        return 0
    
    permutations = list(itertools.permutations(letters))
    
    res = sum([not any_dupes(i) for i in permutations])
    return res


def any_dupes(x) :
    dupes_found = False
    
    for i in range(0, len(x) - 1) :
        if x[i] == x[i + 1] :
            dupes_found = True
            break
    
    return dupes_found  



class testPermAlone(unittest.TestCase):

    def test_perm_alone(self):
        self.assertEqual(perm_alone("aab"), 2)
        self.assertEqual(perm_alone("aab"), 2)
        self.assertEqual(perm_alone("aaa"), 0)
        self.assertEqual(perm_alone("aabb"), 8)
        self.assertEqual(perm_alone("abcdefa"), 3600)
        self.assertEqual(perm_alone("zzzzzzzz"), 0)
        self.assertEqual(perm_alone("a"), 1)
        self.assertEqual(perm_alone("aaab"), 0)
        self.assertEqual(perm_alone("aaabb"), 12)


if __name__ == "__main__" :
  unittest.main()
