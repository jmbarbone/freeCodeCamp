import unittest


def sym_diff(x):
    res = []

    for i in range(0, len(x)):
        res = do_sym_diff(res, x[i])

    return res


def do_sym_diff(x, y):
    a = [i for i in x if i not in y]
    b = [j for j in y if j not in x]
    return sorted(set(a + b))


class testSymDiff(unittest.TestCase):

    def test_sym_diff(self):
        self.assertEqual(sym_diff([[1, 2, 3],    [5, 2, 1, 4]]),               [3, 4, 5])
        self.assertEqual(sym_diff([[1, 2, 3, 3], [5, 2, 1, 4]]),               [3, 4, 5])
        self.assertEqual(sym_diff([[1, 2, 3],    [5, 2, 1, 4, 5]]),            [3, 4, 5])
        
        self.assertEqual(sym_diff([[1, 2, 5],    [2, 3, 5],    [3, 4, 5]]),    [1, 4, 5])
        self.assertEqual(sym_diff([[1, 1, 2, 5], [2, 2, 3, 5], [3, 4, 5, 5]]), [1, 4, 5])

        self.assertEqual(
            sym_diff([
                [3, 3, 3, 2, 5],
                [2, 1, 5, 7],
                [3, 4, 6, 6],
                [1, 2, 3],
                [5, 3, 9, 8],
                [1]
            ]),
            [1, 2, 4, 5, 6, 7, 8, 9]
        )


if __name__ == '__main__':
    unittest.main()
