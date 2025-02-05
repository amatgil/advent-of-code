module Day2 where

import Data.List (sort, transpose)
import Text.Printf

sample =
  "7 6 4 2 1\n\
  \1 2 7 8 9\n\
  \9 7 6 2 1\n\
  \1 3 2 4 5\n\
  \8 6 4 4 1\n\
  \1 3 6 7 9\n"

parse :: String -> [[Int]]
parse input = map read . words <$> lines input

isSafe :: [Int] -> Bool
isSafe xs = isMono xs && smallDiff xs
  where
    stencil _ [] = []
    stencil _ [x] = []
    stencil f (x : y : rest) = f x y : stencil f (y : rest)
    isMono = and . stencil (==) . map signum . stencil (-)
    smallDiff = all ((\d -> d >= 1 && d <= 3) . abs) . stencil (-)

part1 :: [[Int]] -> Int
part1 = length . filter isSafe

part2 :: [[Int]] -> Int
part2 = length . filter isSafe'
  where
    isSafe' line = any isSafe $ removalMat line
    removalMat [] = []
    removalMat (x : xs) = xs : map (x :) (removalMat xs)

day2 :: IO ()
day2 = do
  input <- readFile "inputs/day2.txt"
  printf "Day 2, Part 1: %d\n" $ part1 $ parse input
  printf "Day 2, Part 2: %d\n" $ part2 $ parse input
