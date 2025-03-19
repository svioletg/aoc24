-- validate :: Ord a => [a] -> Bool
-- validate (x:y:xs) =

pairwise :: [a] -> [(a, a)]
pairwise [] = []
pairwise [_] = []
pairwise (x:y:xs) = (x, y):pairwise (y:xs)

main :: IO ()
main = do
    puzzle <- readFile "../input.txt"
    let reps = [map read (words i) :: [Int] | i <- lines puzzle]
    let reps = map (map read . words) . lines $ puzzle :: [[Int]]
    let safe = 0
    print "Day 1, Part 1"
