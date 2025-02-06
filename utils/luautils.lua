-- Read all text from file
function read_all(fp)
    f = io.open(fp, "r")
    io.input(f)
    content = ""
    while true do
        line = io.read()
        if line == nil then
            break
        end
        content = content .. line .. "\n"
    end  
    io.close()
    return content
end

-- Splits a string into a table of matches by `sep`
function string.split(s, sep)
    sep = sep or "%s"
    groups = {}
    for match in string.gmatch(s, "([^"..sep.."]+)") do
        table.insert(groups, match)
    end
    return groups
end

-- Applies `fn` to every item in `t`
function table.map(fn, t)
    mapped = {}
    for n, i in pairs(t) do
        mapped[n] = fn(i)
    end
    return t
end

-- Returns a table of all values in `t` that evaluate true when called with `fn`
function table.filter(fn, t)
    filtered = {}
    for n, i in pairs(t) do
        if fn(i) then
            table.insert(filtered, i)
        end
    end
    return filtered
end

-- Returns a table of all values in `t` that are equal to `val`
function table.filtermatch(val, t)
    filtered = {}
    for n, i in pairs(t) do
        if i == val then
            table.insert(filtered, i)
        end
    end
    return filtered
end
