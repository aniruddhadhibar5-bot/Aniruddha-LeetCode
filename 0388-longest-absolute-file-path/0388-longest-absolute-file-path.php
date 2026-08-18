class Solution {

    /**
     * @param String $input
     * @return Integer
     */
    function lengthLongestPath($input) {
        // Split the input into lines representing files and directories
        $lines = explode("\n", $input);
        
        $depthLengths = [];
        $maxLength = 0;

        foreach ($lines as $line) {
            // Count leading tabs to determine the current depth level (0-indexed)
            $depth = strspn($line, "\t");
            
            // Calculate the actual clean length of the folder or file name
            $nameLength = strlen($line) - $depth;
            
            // Total length up to the parent directory + 1 for the '/' separator
            $parentLength = $depth > 0 ? $depthLengths[$depth - 1] + 1 : 0;
            $currentLength = $parentLength + $nameLength;
            
            // If the name contains a dot, it is a file
            if (strpos($line, '.') !== false) {
                if ($currentLength > $maxLength) {
                    $maxLength = $currentLength;
                }
            } else {
                // Otherwise, save the directory's total length at this depth level
                $depthLengths[$depth] = $currentLength;
            }
        }

        return $maxLength;
    }
}
