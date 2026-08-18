(define/contract (min-trio-degree n edges)
  (-> exact-integer? (listof (listof exact-integer?)) exact-integer?)
  (let* ([n+1 (+ n 1)]
         [grid (make-vector (* n+1 n+1) #f)]
         [degrees (make-vector n+1 0)])
    
    ;; 1. Build the adjacency matrix and track node degrees
    (for ([edge edges])
      (let ([u (car edge)]
            [v (cadr edge)])
        (vector-set! grid (+ (* u n+1) v) #t)
        (vector-set! grid (+ (* v n+1) u) #t)
        (vector-set! degrees u (+ (vector-ref degrees u) 1))
        (vector-set! degrees v (+ (vector-ref degrees v) 1))))
    
    ;; 2. Optimize the trio search using high-performance named loops
    (let loop-u ([u 1] [ans -1])
      (if (> u n)
          ans
          (let loop-v ([v (+ u 1)] [current-ans ans])
            (if (> v n)
                (loop-u (+ u 1) current-ans)
                (if (vector-ref grid (+ (* u n+1) v))
                    (let loop-w ([w (+ v 1)] [inner-ans current-ans])
                      (if (> w n)
                          (loop-v (+ v 1) inner-ans)
                          (if (and (vector-ref grid (+ (* v n+1) w))
                                   (vector-ref grid (+ (* u n+1) w)))
                              (let* ([deg-u (vector-ref degrees u)]
                                     [deg-v (vector-ref degrees v)]
                                     [deg-w (vector-ref degrees w)]
                                     [trio-deg (- (+ deg-u deg-v deg-w) 6)]
                                     [next-ans (if (= inner-ans -1) trio-deg (min inner-ans trio-deg))])
                                (loop-w (+ w 1) next-ans))
                              (loop-w (+ w 1) inner-ans))))
                    (loop-v (+ v 1) current-ans))))))))
