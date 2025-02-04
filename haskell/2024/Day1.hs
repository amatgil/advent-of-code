module Day1 where
import Data.List (transpose, sort)


sample =
  "3   4\n\
  \4   3\n\
  \2   5\n\
  \1   3\n\
  \3   9\n\
  \3   3\n"

parse :: String -> ([Int], [Int])
parse input = (left, right) where
  dat = transpose $ fmap (fmap read) $ map words $ lines input
  left = head dat
  right = last dat

part1 :: ([Int], [Int]) -> Int
part1 (left, right) = sum $ zipWith (-) (sort right) (sort left)

part2 :: ([Int], [Int]) -> Int
part2 (left, right) = sum $ map (\n -> n * countInstances n right) left
  where countInstances x = length . filter (== x) 
