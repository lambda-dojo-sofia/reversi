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

(defn neighbours [board [dot-x dot-y] player]
  (let [candidates (filter (fn [[a b]]
                             (and (valid-index? a)
                                  (valid-index? b)))
                           [[(dec dot-x) dot-y]
                            [dot-x (dec dot-y)]
                            [(inc dot-x) dot-y]
                            [dot-x (inc dot-y)]])
        enemies (filter (fn [[enemy-x enemy-y]]
                          (= (opposite player) (r board enemy-x enemy-y))) candidates)
        allies (map (fn [[enemy-x enemy-y]]
                      (let [[a b] (cond
                                    (and (= dot-x enemy-x) (= (dec dot-y) enemy-y)) [(repeat enemy-x) (range 1 enemy-y)]
                                    (and (= dot-x enemy-x) (= (inc dot-y) enemy-y)) [(repeat enemy-x) (range enemy-y 9)]
                                    (and (= dot-y enemy-y) (= (dec dot-x) enemy-x)) [(range 1 enemy-x) (repeat enemy-y)]
                                    (and (= dot-y enemy-y) (= (inc dot-x) enemy-x)) [(range enemy-x 9) (repeat enemy-y)])]
                        (filter #{player} (map #(r board %1 %2) a b))))
                    enemies)]
    (first allies)))

(defn legal-moves [board player]
  (let [dots (for [x (range 1 9)
                   y (range 1 9)
                   :when (= \. (r board x y))]
               [x y])]
    (filter #(neighbours board % player) dots)))

(println (legal-moves board \B))