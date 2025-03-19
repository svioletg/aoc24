module Main where

pairwise :: [a] -> [(a, a)]
pairwise [] = []
pairwise [_] = []
pairwise (x:y:xs) = (x, y) : pairwise (y:xs)

stableDir :: [Int] -> Bool
stableDir [] = True
stableDir [_] = True
stableDir lst = do
    let diffs = map (uncurry (-)) (pairwise lst)
    abs (sum (map (\i -> if i >= 0 then 1 else -1) diffs)) == length lst - 1
        && length (filter (\i -> abs i `elem` [1..3]) diffs) == length lst - 1

main :: IO ()
main = do
    puzzle <- readFile "../input.txt"
    let reps = map (map read . words) . lines $ puzzle :: [[Int]]
    let safe = filter stableDir reps
    print "Day 1, Part 1"
    print (length safe)
