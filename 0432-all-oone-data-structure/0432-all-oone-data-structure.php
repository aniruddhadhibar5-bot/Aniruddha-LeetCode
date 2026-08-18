class Node {
    public int $count;
    public array $keys = []; // Key-value store for O(1) key additions/removals
    public ?Node $prev = null;
    public ?Node $next = null;

    public function __construct(int $count) {
        $this->count = $count;
    }
}

class AllOne {
    private array $keyToNode = [];
    private Node $head;
    private Node $tail;

    public function __construct() {
        // Initialize dummy head and tail nodes
        $this->head = new Node(0);
        $this->tail = new Node(0);
        $this->head->next = $this->tail;
        $this->tail->prev = $this->head;
    }

    /**
     * @param String $key
     * @return null
     */
    public function inc(string $key): void {
        if (isset($this->keyToNode[$key])) {
            $currNode = $this->keyToNode[$key];
            $nextNode = $currNode->next;

            // Create next node if it doesn't match the incremented count
            if ($nextNode === $this->tail || $nextNode->count !== $currNode->count + 1) {
                $nextNode = $this->insertNodeAfter($currNode, $currNode->count + 1);
            }

            $nextNode->keys[$key] = true;
            $this->keyToNode[$key] = $nextNode;

            unset($currNode->keys[$key]);
            if (empty($currNode->keys)) {
                $this->removeNode($currNode);
            }
        } else {
            // Key is new, insert into a node with count = 1
            $firstNode = $this->head->next;
            if ($firstNode === $this->tail || $firstNode->count !== 1) {
                $firstNode = $this->insertNodeAfter($this->head, 1);
            }

            $firstNode->keys[$key] = true;
            $this->keyToNode[$key] = $firstNode;
        }
    }

    /**
     * @param String $key
     * @return null
     */
    public function dec(string $key): void {
        if (!isset($this->keyToNode[$key])) {
            return;
        }

        $currNode = $this->keyToNode[$key];
        unset($currNode->keys[$key]);

        if ($currNode->count > 1) {
            $prevNode = $currNode->prev;

            // Create previous node if it doesn't match the decremented count
            if ($prevNode === $this->head || $prevNode->count !== $currNode->count - 1) {
                $prevNode = $this->insertNodeAfter($prevNode, $currNode->count - 1);
            }

            $prevNode->keys[$key] = true;
            $this->keyToNode[$key] = $prevNode;
        } else {
            unset($this->keyToNode[$key]);
        }

        if (empty($currNode->keys)) {
            $this->removeNode($currNode);
        }
    }

    /**
     * @return String
     */
    public function getMaxKey(): string {
        if ($this->tail->prev === $this->head) {
            return "";
        }
        // Return any key from the maximum frequency node
        return (string) array_key_first($this->tail->prev->keys);
    }

    /**
     * @return String
     */
    public function getMinKey(): string {
        if ($this->head->next === $this->tail) {
            return "";
        }
        // Return any key from the minimum frequency node
        return (string) array_key_first($this->head->next->keys);
    }

    // Helper to insert a new node after a given node in the DLL
    private function insertNodeAfter(Node $node, int $count): Node {
        $newNode = new Node($count);
        $newNode->next = $node->next;
        $newNode->prev = $node;
        $node->next->prev = $newNode;
        $node->next = $newNode;
        return $newNode;
    }

    // Helper to completely unlink an empty node from the DLL
    private function removeNode(Node $node): void {
        $node->prev->next = $node->next;
        $node->next->prev = $node->prev;
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * $obj = new AllOne();
 * $obj->inc($key);
 * $obj->dec($key);
 * $ret_3 = $obj->getMaxKey();
 * $ret_4 = $obj->getMinKey();
 */
