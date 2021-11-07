from typing import Sequence
import unittest

def pairwise(x, n) :
    # get the index values
    rx = range(len(x))

    # initialize empty object
    finds = []
    # loop through indexes twice -- need to identify just two values
    for i in rx :
        for j in rx :
            # same value (by index) cannot be added to itself
            # values must add to n
            # index cannot be used twice
            if (i != j) and (x[i] + x[j] == n) and (i not in finds) and (j not in finds) : 
                finds = finds + [i] + [j]

    return sum(finds)


class testPairwise(unittest.TestCase):

    def test_pairwise(self):
        self.assertEqual(pairwise([1, 4, 2, 3, 0, 5], 7), 11)
        self.assertEqual(pairwise([1, 3, 2, 4], 4), 1)
        self.assertEqual(pairwise([1, 1, 1], 2), 1)
        self.assertEqual(pairwise([0, 0, 0, 0, 1, 1], 1), 10)
        self.assertEqual(pairwise([], 100), 0)


if __name__ == "__main__" :
  unittest.main()

