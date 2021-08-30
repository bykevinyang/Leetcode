# Two Sum Problem (Easy)
[Description](https://leetcode.com/problems/two-sum/)

## Approach
The naive approach to this problem would be to have two loops and brute force through all sums. However, this is slow and runs in O(n^2) speed. 

A faster solution can be acheived by sorting the input list, and then running a binary search for the proper sum. Since the list is now in order, by searchin in a certain direction the sum will be directly affected (i.e moving to the upper half will increase the sum, vice versa for moving to the lower half). The runtime of this algorithm is the sum of rust's built-in sort algorithm (a variation of timsort) and binary search. 

Timsort's runtime: O(n log n)
 