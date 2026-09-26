class Solution {
  int thirdMax(List<int> nums) {
    // Track the top three values as nullable integers
    int? first;
    int? second;
    int? third;

    for (int num in nums) {
      // Discard duplicate items to maintain distinct counts
      if (num == first || num == second || num == third) {
        continue;
      }

      // Shift leaderboard values downward as higher scores appear
      if (first == null || num > first) {
        third = second;
        second = first;
        first = num;
      } else if (second == null || num > second) {
        third = second;
        second = num;
      } else if (third == null || num > third) {
        third = num;
      }
    }

    // Fall back to the absolute max value if a third distinct max does not exist
    return third ?? first!;
  }
}
