import Data.List
import Text.Printf

count :: Eq a => [a] -> a -> Int
count arr target = length [i | i <- arr, i == target]

main = do
    puzzle <- readFile "../input.txt"
    let locids = [map read (words i) :: [Integer] | i <- lines puzzle]
    let left = sort [i !! 0 | i <- locids]
    let right = sort [i !! 1 | i <- locids]
    let dist = sum [abs (fst i - snd i) | i <- zip left right]
    let simscore = sum [i * fromIntegral (count right i) | i <- left]

    print "Day 1, Part 1"
    print dist
    print "Day 1, Part 2"
    print simscore
