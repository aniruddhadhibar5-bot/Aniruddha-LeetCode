(define/contract (critical-connections n connections)
  (-> exact-integer? (listof (listof exact-integer?)) (listof (listof exact-integer?)))
  
  ;; 1. Build Adjacency List using vectors for O(1) lookups
  (define adj (make-vector n '()))
  (for ([conn connections])
    (let ([u (car conn)]
          [v (cadr conn)])
      (vector-set! adj u (cons v (vector-ref adj u)))
      (vector-set! adj v (cons u (vector-ref adj v)))))
  
  ;; 2. Initialize tracking states
  (define disc (make-vector n -1))
  (define low (make-vector n -1))
  (define time 0)
  (define critical '())
  
  ;; 3. Define recursive DFS helper
  (define (dfs u p)
    (vector-set! disc u time)
    (vector-set! low u time)
    (set! time (+ time 1))
    
    (for ([v (vector-ref adj u)])
      (unless (= v p) ; Skip the immediate edge to the parent
        (if (= (vector-ref disc v) -1)
            ;; If neighbor 'v' is not visited, visit it recursively
            (begin
              (dfs v u)
              (vector-set! low u (min (vector-ref low u) (vector-ref low v)))
              ;; Check bridge condition
              (when (> (vector-ref low v) (vector-ref disc u))
                (set! critical (cons (list u v) critical))))
            ;; If 'v' is already visited, update the low-link value using its discovery time
            (vector-set! low u (min (vector-ref low u) (vector-ref disc v)))))))
  
  ;; 4. Run DFS starting from node 0 (since the graph is fully connected)
  (dfs 0 -1)
  critical)
