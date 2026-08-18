class Solution {
  String reverseVowels(String s) {
    // Convert string to a mutable list of characters
    List<String> chars = s.split('');
    
    int left = 0;
    int right = chars.length - 1;
    
    // A quick lookup string for vowels
    const vowels = "aeiouAEIOU";

    while (left < right) {
      // Move left pointer forward if it's not a vowel
      if (!vowels.contains(chars[left])) {
        left++;
        continue;
      }
      
      // Move right pointer backward if it's not a vowel
      if (!vowels.contains(chars[right])) {
        right--;
        continue;
      }
      
      // Both are vowels -> Swap them
      String temp = chars[left];
      chars[left] = chars[right];
      chars[right] = temp;
      
      // Move both pointers inward
      left++;
      right--;
    }

    return chars.join('');
  }
}
