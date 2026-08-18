(define/contract (find-critical-and-pseudo-critical-edges n edges)
  (-> exact-integer? (listof (listof exact-integer?)) (listof (listof exact-integer?)))
  
  ;; Union-Find Helper structures
  (define (make-uf size)
    (let ([parent (make-vector size)])
      (for ([i (in-range size)]) (vector-set! parent i i))
      parent))

  (define (find-set uf i)
    (if (= (vector-ref uf i) i)
        i
        (let ([root (find-set uf (vector-ref uf i))])
          (vector-set! uf i root)
          root)))

  (define (union-set uf i j)
    (let ([root-i (find-set uf i)]
          [root-j (find-set uf j)])
      (if (not (= root-i root-j))
          (begin (vector-set! uf root-i root-j) #t)
          #f)))

  ;; Helper function to calculate MST weight
  (define (get-mst sorted-edges skip-idx force-idx)
    (define uf (make-uf n))
    (define weight 0)
    (define count 0)
    
    ;; Force an edge into the MST if requested
    (when (>= force-idx 0)
      (let* ([edge (list-ref sorted-edges force-idx)]
             [u (car edge)]
             [v (cadr edge)]
             [w (caddr edge)])
        (union-set uf u v)
        (set! weight (+ weight w))
        (set! count (+ count 1))))
        
    ;; Process all remaining edges sequentially
    (for ([i (in-range (length sorted-edges))])
      (unless (or (= i skip-idx) (= i force-idx))
        (let* ([edge (list-ref sorted-edges i)]
               [u (car edge)]
               [v (cadr edge)]
               [w (caddr edge)])
          (when (union-set uf u v)
            (set! weight (+ weight w))
            (set! count (+ count 1))))))
            
    (if (= count (- n 1)) weight -1))

  ;; 1. Attach original index to each edge: (u v weight original-index)
  (define indexed-edges
    (for/list ([edge edges] [idx (in-naturals)])
      (list (car edge) (cadr edge) (caddr edge) idx)))

  ;; 2. Sort edges by weight
  (define sorted-edges
    (sort indexed-edges (lambda (e1 e2) (< (caddr e1) (caddr e2)))))

  ;; 3. Get the baseline MST weight
  (define base-weight (get-mst sorted-edges -1 -1))

  ;; 4. Check each edge for critical or pseudo-critical status
  (define critical '())
  (define pseudo '())
  
  (for ([i (in-range (length sorted-edges))])
    (let* ([edge (list-ref sorted-edges i)]
           [orig-idx (cadddr edge)]
           [weight-without (get-mst sorted-edges i -1)])
      (if (or (= weight-without -1) (> weight-without base-weight))
          ;; If deleting it breaks connectivity or increases weight -> Critical
          (set! critical (cons orig-idx critical))
          ;; Else check if forcing it yields the optimal baseline weight -> Pseudo-Critical
          (let ([weight-with (get-mst sorted-edges -1 i)])
            (when (= weight-with base-weight)
              (set! pseudo (cons orig-idx pseudo)))))))

  ;; Return results sorted or in any order as allowed
  (list (reverse critical) (reverse pseudo)))
