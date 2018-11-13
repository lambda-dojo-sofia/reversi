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

(defn legal-move [board dot-x dot-y player]
  (when (= \. (r board dot-x dot-y))
    (let [neighbours (filter (fn [[a b]]
                               (and (valid-index? a)
                                    (valid-index? b)))
                             [[(dec dot-x) dot-y]
                              [dot-x (dec dot-y)]
                              [(inc dot-x) dot-y]
                              [dot-x (inc dot-y)]])
          enemies (filter (fn [[enemy-x enemy-y]]
                            (= (opposite player) (r board enemy-x enemy-y))) neighbours)
          allies (map (fn [[enemy-x enemy-y]]
                        (for [ally-x (condp = enemy-x
                                       dot-x       (repeat enemy-x)
                                       (dec dot-x) (range 1 enemy-x)
                                       (inc dot-x) (range enemy-x 9))
                              ally-y (condp = enemy-y
                                       dot-y (repeat enemy-y)
                                       (dec dot-y) (range 1 enemy-y)
                                       (inc dot-y) (range enemy-y 9))
                              :when (= player (r board ally-x ally-y))]
                          true))
                      enemies)]
      (first allies))))

(defn legal-moves [board player]
  (for [x (range 1 9)
        y (range 1 9)
        :when (legal-move board x y player)]
    (str (nth "ABCDEFGH" (dec x)) y)))

(println (legal-moves board \B))