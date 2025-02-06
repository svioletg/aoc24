package.path = "../utils/luautils.lua;" .. package.path
require("luautils")

sample=[[3   4
4   3
2   5
1   3
3   9
3   3]]

content = read_all("input.txt")

left = {}
right = {}

for _, line in pairs(string.split(content, "\n")) do
    nums = string.split(line)
    table.insert(left, nums[1])
    table.insert(right, nums[2])
end

table.sort(left)
table.sort(right)

total = 0
simscore = 0

for n, l in pairs(left) do
    r = right[n]
    total = total + math.abs(tonumber(l) - tonumber(r))
    lcount = #table.filtermatch(l, right)
    simscore = simscore + (l * lcount)
end

print(simscore)
