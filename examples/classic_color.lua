result = fg(reset) .. "["
result = result .. fg(red) .. username()
result = result .. fg(reset) .. "@"
result = result .. fg(magenta) .. hostname()
result = result .. fg(reset) .. ":"

rbenv = rbenv_init()
if rbenv:enabled() then
  result = result .. fg(green)
  result = result .. "[Ruby "
  result = result .. rbenv:version()
  result = result .. "]"
end

result = result .. fg(blue)
result = result .. " " .. cwd()

git = git_init()
if git:enabled() then
  result = result .. fg(reset)
  result = result .. " ("

  result = result .. fg(magenta)
  result = result .. git:head()
  result = result .. " "

  result = result .. fg(reset)
  if git:behind() > 0 then
    result = result .. "↓" .. git:behind()
  end
  if git:ahead() > 0 then
    result = result .. "↑" .. git:ahead()
  end

  if git:behind() > 0 or git:ahead() > 0 then
    result = result .. " "
  end

  if git:index() > 0 then
    result = result .. fg(green)
    result = result .. "●" .. git:index()
  end
  if git:wt() > 0 then
    result = result .. fg(red)
    result = result .. "+" .. git:wt()
  end
  if git:untracked() > 0 then
    result = result .. fg(reset)
    result = result .. "…"
  end
  if git:index() == 0 and git:wt() == 0 and git:untracked() == 0 then
    result = result .. fg(green)
    result = result .. "✓"
  end

  result = result .. fg(reset)
  result = result .. ")"
end

result = result .. fg(reset)
result = result .. "]"
result = result .. "\n"
result = result .. ":) "

return result
