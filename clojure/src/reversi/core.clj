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

(defn neighbours [board [x y] player]
  (let [candidates (filter (fn [[[a b] v]]
                             (and (valid-index? a)
                                  (valid-index? b)))
                           {[(dec x) y] :left
                            [x (dec y)] :up
                            [(inc x) y] :right
                            [x (inc y)] :down})
        enemies (filter (fn [[[x y] direction]]
                          (= (opposite player) (r board x y))) candidates)
        allies (map (fn [[[x y] direction]]
                      (let [[a b] (condp = direction
                                    :up    [(repeat x) (range 1 y)]
                                    :down  [(repeat x) (range y 9)]
                                    :left  [(range 1 x) (repeat y)]
                                    :right [(range x 9) (repeat y)])]
                        (some #{player} (map #(r board %1 %2) a b))))
                    enemies)]
    (not (empty? allies))))

(defn legal-moves [board player]
  (let [dots (for [x (range 1 9)
                   y (range 1 9)
                   :when (= \. (r board x y))]
               [x y])]
    (filter #(neighbours board % player) dots)))

(println (legal-moves board \B))