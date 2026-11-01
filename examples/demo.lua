local ttk = require("ttk")

-- Source - https://stackoverflow.com/a/27028488
-- Posted by hookenz, modified by community. See post 'Timeline' for change history
-- Retrieved 2026-11-01, License - CC BY-SA 4.0

function dump(o)
   if type(o) == 'table' then
      local s = '{ '
      for k,v in pairs(o) do
         local k = k
         if type(k) ~= 'number' then k = '"'..k..'"' end
         s = s .. '['..k..'] = ' .. dump(v) .. ','
      end
      return s .. '} '
   else
      return tostring(o)
   end
end


ttk.print.info(1, "Welcome to the TTK Lua demo!")
ttk.print.ln()

local username = ttk.query.line(2, "Enter your username:")
print(dump(username))
if username == "" or string.find(username, " ") then
    ttk.exit.err(3, "Invalid username")
end

ttk.print.ln()
local id = ttk.query.num(2, "Enter your ID (integer):")
if id < 0 then
    ttk.print.warn(3, "Negative IDs are dangerous!")
end

ttk.print.ln()
local countries = {"USA", "CAN", "GBR", "DEU", "JPN", "RUS"}
local country = ttk.query.one(2, "Choose your country:", countries)

ttk.print.ln()
local roles = {"Admin", "Writer", "Announcer", "Editor", "Reader"}
local selected_roles = ttk.query.many(2, "Choose your roles:", roles, {4})

local function filter_roles(t1, t2)
    local filtered = {}
    local n = math.min(#t1, #t2)
    
    for i = 1, n do
        if t1[i] then
            table.insert(filtered, t2[i])
        end
    end
    
    return filtered
end

ttk.print.ln()
ttk.print.dbg(
    2,
    "Username: " .. username .. ", ID: " .. id .. ", Country: " .. countries[country] .. ", Roles: " .. ttk.display.from_iter(
        filter_roles(selected_roles, roles),
        ", "
    ))

ttk.print.info(1, "Thanks for viewing the demo!")
ttk.exit.ok()
