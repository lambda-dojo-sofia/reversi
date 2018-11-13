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

(defn r [board x y] (nth (nth board x) y))

(defn coords [c]
  (for [x (range 0 8)
        y (range 0 8)
        :when (= c (nth (nth board x) y))]
    [x y]))

(defn valid-index? [n]
  (<= 0 n 7))

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
                                    :up    [(repeat x) (range 0 y)]
                                    :down  [(repeat x) (range y 8)]
                                    :left  [(range 0 x) (repeat y)]
                                    :right [(range x 8) (repeat y)])]
                        (some #{player} (map #(r board %1 %2) a b))))
                    enemies)]
    (not (empty? allies))))

(defn legal-moves [board player]
  (let [dots (coords \.)]
    (filter #(neighbours board % (opposite player)) dots)))

(println (legal-moves board "B"))