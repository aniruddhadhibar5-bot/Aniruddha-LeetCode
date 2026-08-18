import 'dart:math';

class Solution {
  int integerBreak(int n) {
    // Base cases handled separately as required by the constraint k >= 2
    if (n == 2) return 1;
    if (n == 3) return 2;

    int numThrees = n ~/ 3;
    int remainder = n % 3;

    if (remainder == 0) {
      return pow(3, numThrees).toInt();
    } else if (remainder == 1) {
      // Instead of 3 * 1, we change it to 2 * 2 = 4
      return (pow(3, numThrees - 1) * 4).toInt();
    } else {
      // Remainder is 2, just multiply by 2
      return (pow(3, numThrees) * 2).toInt();
    }
  }
}
