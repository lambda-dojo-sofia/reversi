(ns reversi.core)

(def board
  ["........"
   "........"
   "........"
   "...BW..."
   "...WB..."
   "........"
   "........"
   "........"])

(defn r [board x y] (nth (nth board (dec x)) (dec y)))

(defn valid-index? [n]
  (<= 1 n 8))

(defn opposite [c]
  (if (= c \B) \W \B))

(defn legal-move [board x y player]
  (when (= \. (r board x y))
    (first (for [[enemy-x enemy-y] [[(dec x)      y ]
                                    [     x  (dec y)]
                                    [(inc x)      y ]
                                    [     x  (inc y)]]
                 :when (and (valid-index? enemy-x)
                            (valid-index? enemy-y)
                            (= (opposite player) (r board enemy-x enemy-y)))]
             (for [[flank-x flank-y] [(condp = enemy-x
                                                   x  (repeat enemy-x)
                                              (dec x) (range 1 enemy-x)
                                              (inc x) (range enemy-x 9))
                                      (condp = enemy-y
                                                   y  (repeat enemy-y)
                                              (dec y) (range 1 enemy-y)
                                              (inc y) (range enemy-y 9))]
                   :while (not= player (r board flank-x flank-y))]
               true)))))

(defn legal-moves [board player]
  (for [x (range 1 9)
        y (range 1 9)
        :when (legal-move board x y player)]
    (str (nth "ABCDEFGH" (dec x)) y)))

(println (legal-moves board \B))