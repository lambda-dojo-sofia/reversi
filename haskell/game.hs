import Data.List

data Pos = B | W | Dot | Move
    deriving (Show, Eq)

data Player = Black | White

type Matrix = [[Pos]]

board :: Matrix
board = [
    [Dot, B, W, Move, Dot],
    [Move, W, B, Dot, Dot],
    [Dot, Move, Dot, Dot, Dot]]

showMatrix :: Matrix -> String
showMatrix = unlines . (fmap f)
    where f :: [Pos] -> String
          f = unwords . (fmap show)

solve :: Player -> Matrix -> Matrix
solve pl m = [[ res x y | x <- [0..maxX]] | y <- [0..maxY]]
    where maxX = (length m) - 1
          maxY = (length (m !! 1)) - 1
          res x y = if canMove m x y pl then Move else ((m!!x)!!y)

canMove :: Matrix -> Int -> Int -> Player -> Bool
canMove m x y pl
    | outOfBounds x y m = False
    | current /= Dot = False
    | ((m!!(x-1))!!y) == op && ((m!!(x-2))!!y) == pl = True
    | ((m!!(x+1))!!y) == op && ((m!!(x+2))!!y) == pl = True
    | ((m!!x)!!(y-1)) == op && ((m!!x)!!(y-2)) == pl = True
    | ((m!!x)!!(y+1)) == op && ((m!!x)!!(y+2)) == pl = True
    | otherwise                                      = False
        where current = ((m!!x)!!y)
              op = otherPlayer pl


otherPlayer :: Player -> Pos
otherPlayer White = B
otherPlayer Black = W

outOfBounds :: Int -> Int -> Matrix -> Bool
outOfBounds x y m | x < 0 = True
                  | y < 0 = True
                  | length m <= x = True
                  | length (m !! 1) <= y = True
                  | otherwise = False