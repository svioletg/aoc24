import Data.List

count :: Eq a => [a] -> a -> Int
count lst target = length $ filter (== target) lst

main = do
    puzzle <- readFile "../input.txt"
    let locids::[[Int]] = map (map read . words) . lines $ puzzle
    let left::[Int] = sort $ map head locids
    let right::[Int] = sort $ map (!! 1) locids
    let dist::Int = sum $ map abs $ zipWith (-) left right
    let simscore::Int = sum $ map (\i -> i * count right i) left

    print "Day 1, Part 1"
    print dist
    print "Day 1, Part 2"
    print simscore
