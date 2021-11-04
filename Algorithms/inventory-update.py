
# using dictionaries instead of arrays

import unittest


def updateInventory(old, new):
    res = checkInventory(old, new)
    res = dict(sorted(res.items()))
    return res


def checkInventory(x, y):
    if len(x) == 0:
        return y

    if len(y) == 0:
        return x

    for key, val in y.items():
        if key in x:
            x[key] = x.get(key) + val
        else:
            x[key] = val
    return x


class testInventoryUpdate(unittest.TestCase):

    def test_update_inventory(self):
        self.assertEqual(
            updateInventory(
                {
                    "Bowling Ball": 21,
                    "Dirty Sock": 2,
                    "Hair Pin": 1,
                    "Microphone": 5
                },
                {
                    "Hair Pin": 2,
                    "Half-Eaten Apple": 3,
                    "Bowling Ball": 67,
                    "Toothpaste": 7
                }
            ),
            {
                "Bowling Ball": 88,
                "Dirty Sock": 2,
                "Hair Pin": 3,
                "Half-Eaten Apple": 3,
                "Microphone": 5,
                "Toothpaste": 7
            }
        )

        self.assertEqual(
            updateInventory(
                {"Bowling Ball": 21, "Dirty Sock": 2, "Hair Pin": 1, "Microphone": 5},
                {}
            ),
            {"Bowling Ball": 21, "Dirty Sock": 2, "Hair Pin": 1, "Microphone": 5}
        )

        self.assertEqual(
            updateInventory(
                {},
                {"Hair Pin": 2, "Half-Eaten Apple": 3, "Bowling Ball": 67, "Toothpaste": 7}
            ),
            {"Hair Pin": 2, "Half-Eaten Apple": 3, "Bowling Ball": 67, "Toothpaste": 7}
        )

        self.assertEqual(
            updateInventory(
                {
                    "Bowling Ball": 0,
                    "Dirty Sock": 0,
                    "Hair Pin": 0,
                    "Microphone": 0
                },
                {
                    "Hair Pin": 1,
                    "Half-Eaten Apple": 1,
                    "Bowling Ball": 1,
                    "Toothpaste": 1
                }
            ),
            {
                "Bowling Ball": 1,
                "Dirty Sock": 0,
                "Hair Pin": 1,
                "Half-Eaten Apple": 1,
                "Microphone": 0,
                "Toothpaste": 1
            }
        )


if __name__ == '__main__':
    unittest.main()
