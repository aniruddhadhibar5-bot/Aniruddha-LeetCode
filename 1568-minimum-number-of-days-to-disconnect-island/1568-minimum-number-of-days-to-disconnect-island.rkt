(define/contract (min-days grid)
  (-> (listof (listof exact-integer?)) exact-integer?)
  
  (let* ([m (length grid)]
         [n (length (car grid))]
         ;; Convert list of lists to a flat vector for fast mutable operations
         [vec (make-vector (* m n) 0)])
    
    (for ([r (in-range m)]
          [row grid])
      (for ([c (in-range n)]
            [val row])
        (vector-set! vec (+ (* r n) c) val)))

    ;; Helper function to count components using a simple iterative flood fill
    (define (count-islands)
      (let ([visited (make-vector (* m n) #f)]
            [count 0])
        
        (define (dfs r c)
          (let ([stack (list (cons r c))])
            (vector-set! visited (+ (* r n) c) #t)
            (let loop ()
              (unless (null? stack)
                (let* ([curr (car stack)]
                       [cr (car curr)]
                       [cc (cdr curr)])
                  (set! stack (cdr stack))
                  ;; Check 4 neighbors
                  (for ([dir '((-1 . 0) (1 . 0) (0 . -1) (0 . 1))])
                    (let ([nr (+ cr (car dir))]
                          [nc (+ cc (cdr dir))])
                      (when (and (>= nr 0) (< nr m) (>= nc 0) (< nc n))
                        (let ([idx (+ (* nr n) nc)])
                          (when (and (= (vector-ref vec idx) 1) (not (vector-ref visited idx)))
                            (vector-set! visited idx #t)
                            (set! stack (cons (cons nr nc) stack))))))))
                (loop)))))

        (for ([r (in-range m)])
          (for ([c (in-range n)])
            (let ([idx (+ (* r n) c)])
              (when (and (= (vector-ref vec idx) 1) (not (vector-ref visited idx)))
                (set! count (+ count 1))
                (dfs r c)))))
        count))

    ;; 1. Check if already disconnected (Day 0)
    (let ([initial-islands (count-islands)])
      (if (not (= initial-islands 1))
          0
          ;; 2. Check if removing one single island node works (Day 1)
          (let loop-r ([r 0])
            (if (>= r m)
                2 ; 3. If neither 0 nor 1, it must be Day 2
                (let loop-c ([c 0])
                  (if (>= c n)
                      (loop-r (+ r 1))
                      (let ([idx (+ (* r n) c)])
                        (if (= (vector-ref vec idx) 1)
                            (begin
                              (vector-set! vec idx 0) ; Remove land
                              (let ([new-count (count-islands)])
                                (vector-set! vec idx 1) ; Revert back
                                (if (not (= new-count 1))
                                    1
                                    (loop-c (+ c 1)))))
                            (loop-c (+ c 1))))))))))))
