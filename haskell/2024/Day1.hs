module Day1 (day1) where

import Data.List (sort, transpose)
import Text.Printf

sample =
  "3   4\n\
  \4   3\n\
  \2   5\n\
  \1   3\n\
  \3   9\n\
  \3   3\n"

parse :: String -> ([Int], [Int])
parse input = (left, right)
  where
    [left, right] = transpose $ fmap (fmap read) $ map words $ lines input

part1 :: ([Int], [Int]) -> Int
part1 (left, right) = sum $ map abs $ zipWith (-) (sort right) (sort left)

part2 :: ([Int], [Int]) -> Int
part2 (left, right) = sum $ map (\n -> n * countInstances n right) left
  where
    countInstances x = length . filter (== x)

day1 :: IO ()
day1 = do
  input <- readFile "inputs/day1.txt"
  printf "Day 1, Part 1: %d\n" $ part1 $ parse input
  printf "Day 1, Part 2: %d\n" $ part2 $ parse input
