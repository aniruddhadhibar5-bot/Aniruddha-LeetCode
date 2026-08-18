class TextEditor {
    // Stacks to hold characters left and right of the cursor
    private array $left = [];
    private array $right = [];

    public function __construct() {
        // Initializes an empty text editor
    }

    /**
     * @param String $text
     * @return null
     */
    public function addText(string $text): void {
        $len = strlen($text);
        for ($i = 0; $i < $len; $i++) {
            $this->left[] = $text[$i];
        }
    }

    /**
     * @param Integer $k
     * @return Integer
     */
    public function deleteText(int $k): int {
        $deleted = 0;
        while ($k > 0 && !empty($this->left)) {
            array_pop($this->left);
            $deleted++;
            $k--;
        }
        return $deleted;
    }

    /**
     * @param Integer $k
     * @return String
     */
    public function cursorLeft(int $k): string {
        while ($k > 0 && !empty($this->left)) {
            $this->right[] = array_pop($this->left);
            $k--;
        }
        return $this->getLeftLast10();
    }

    /**
     * @param Integer $k
     * @return String
     */
    public function cursorRight(int $k): string {
        while ($k > 0 && !empty($this->right)) {
            $this->left[] = array_pop($this->right);
            $k--;
        }
        return $this->getLeftLast10();
    }

    /**
     * Helper to grab the last min(10, len) characters to the left of the cursor
     */
    private function getLeftLast10(): string {
        $len = count($this->left);
        $start = max(0, $len - 10);
        $result = '';
        for ($i = $start; $i < $len; $i++) {
            $result .= $this->left[$i];
        }
        return $result;
    }
}
