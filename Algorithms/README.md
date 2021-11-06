> Note: Text here is copied from the appropriate URL.
> This is added just to give quicker context

# Algorithms

These free programming exercises will teach you about some common algorithms that you will likely encounter in real life.
They are a great opportunity to improve your logic and programming skills.

These algorithms are frequently used in job interviews to test a candidate's skills.
We'll give you clear and concise explanations of how these different algorithms work so you can implement a solution for each one.

https://www.freecodecamp.org/learn/coding-interview-prep/#algorithms

## Find the symmetric difference

The mathematical term _symmetric difference_ (`△` or `⊕`) of two sets is the set of elements which are in either of the two sets but not in both.
For example, for sets` A = {1, 2, 3}` and `B = {2, 3, 4}`, `A △ B = {1, 4}`.

Symmetric difference is a binary operation, which means it operates on only two elements.
So to evaluate an expression involving symmetric differences among three elements (`A △ B △ C`), you must complete one operation at a time.
Thus, for sets `A` and `B` above, and `C = {2, 3}`, `A △ B △ C = (A △ B) △ C = {1, 4} △ {2, 3} = {1, 2, 3, 4}`.

Create a function that takes two or more arrays and returns an array of their symmetric difference.
The returned array must contain only unique values (no duplicates).

## Inventory Update

Compare and update the inventory stored in a 2D array against a second 2D array of a fresh delivery.
Update the current existing inventory item quantities (in `arr1`).
If an item cannot be found, add the new item and quantity into the inventory array.
The returned inventory array should be in alphabetical order by item.


## No repeats please

Return the number of total permutations of the provided string that don't have repeated consecutive letters. Assume that all characters in the provided string are each unique.

For example, `aab` should return 2 because it has 6 total permutations (`aab`, `aab`, `aba`, `aba`, `baa`, `baa`), but only 2 of them (`aba` and `aba`) don't have the same letter (in this case `a`) repeating.
