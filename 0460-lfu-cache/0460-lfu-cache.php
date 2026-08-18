class LFUCache {
    private int $capacity;
    private int $minFreq = 0;
    
    // Maps key -> [value, frequency]
    private array $keyToValAndFreq = [];
    
    // Maps frequency -> associative array of [key => true] (acts as an ordered LRU queue)
    private array $freqToKeys = [];

    /**
     * @param Integer $capacity
     */
    public function __construct(int $capacity) {
        $this->capacity = $capacity;
    }

    /**
     * @param Integer $key
     * @return Integer
     */
    public function get(int $key): int {
        if (!isset($this->keyToValAndFreq[$key])) {
            return -1;
        }

        $val = $this->keyToValAndFreq[$key][0];
        $this->updateFrequency($key);
        return $val;
    }

    /**
     * @param Integer $key
     * @param Integer $value
     * @return null
     */
    public function put(int $key, int $value): void {
        if ($this->capacity <= 0) {
            return;
        }

        // If key already exists, update value and its frequency
        if (isset($this->keyToValAndFreq[$key])) {
            $this->keyToValAndFreq[$key][0] = $value;
            $this->updateFrequency($key);
            return;
        }

        // If cache capacity is reached, evict the LFU (and LRU if tie) item
        if (count($this->keyToValAndFreq) >= $this->capacity) {
            // Get the first key inserted into the minimum frequency list (LRU)
            $evictKey = array_key_first($this->freqToKeys[$this->minFreq]);
            
            unset($this->freqToKeys[$this->minFreq][$evictKey]);
            unset($this->keyToValAndFreq[$evictKey]);
        }

        // Insert the new key with a frequency of 1
        $this->keyToValAndFreq[$key] = [$value, 1];
        $this->freqToKeys[1][$key] = true;
        $this->minFreq = 1;
    }

    /**
     * Helper method to upgrade a key's frequency bucket
     */
    private function updateFrequency(int $key): void {
        $freq = $this->keyToValAndFreq[$key][1];
        
        // Remove from current frequency bucket
        unset($this->freqToKeys[$freq][$key]);
        
        // If the minimum frequency bucket becomes empty, advance minFreq pointer
        if (empty($this->freqToKeys[$this->minFreq])) {
            unset($this->freqToKeys[$this->minFreq]);
            if ($this->minFreq === $freq) {
                $this->minFreq++;
            }
        }

        // Upgrade frequency and push to the end of the new frequency bucket (Most Recently Used)
        $freq++;
        $this->keyToValAndFreq[$key][1] = $freq;
        $this->freqToKeys[$freq][$key] = true;
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * $obj = new LFUCache($capacity);
 * $ret_1 = $obj->get($key);
 * $obj->put($key, $value);
 */
