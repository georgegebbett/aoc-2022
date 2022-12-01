module Main where
import qualified Data.Text as T
import Data.List.Split as D
import Data.List as L

main :: IO ()
main = do
  contents <- readFile "../src/day1.txt"
  let elves = D.splitOn "\n\n" contents
  let elves2 = map (D.splitOn "\n") elves
  let elves3 = map (map (read)) elves2
  let elves4 = last (L.sort (map sum elves3))
  print elves4