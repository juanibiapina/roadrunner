# Roadrunner

A very fast and configurable prompt for shells.

## Overview

Define a `prompt.lua` script that builds the prompt string and returns it:

```lua
result = "[" .. username() .. "@" .. hostname() .. ":"

rbenv = rbenv_init()
if rbenv:enabled() then
  result = result .. "[Ruby "
  result = result .. rbenv:version()
  result = result .. "]"
end

result = result .. " " .. cwd()

git = git_init()
if git:enabled() then
  result = result .. " ("
  result = result .. git:head()
  result = result .. " "
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
    result = result .. "●" .. git:index()
  end
  if git:wt() > 0 then
    result = result .. "+" .. git:wt()
  end
  if git:untracked() > 0 then
    result = result .. "…"
  end
  if git:index() == 0 and git:wt() == 0 and git:untracked() == 0 then
    result = result .. "✓"
  end
  result = result .. ")"
end

result = result .. "]"
result = result .. "\n"
result = result .. ":) "

return result
```

And run roadrunner pointing to the prompt script:

```sh
$ roadrunner prompt.lua
```

Output:

```
[juanibiapina@MacBookPro:[Ruby 2.5.1] ~/roadrunner (master ↓2↑1 ●2+3…)]
:) 
```

## Usage

Add the main binary to your path and run it pointing to a Lua script that
builds the prompt. Refer to the example in the overview and the API section for
more details.

### Zsh

```sh
setopt prompt_subst # enable prompt substitution
PROMPT='$(roadrunner prompt.lua)'
```

## Configuration

Configuration is done using the Lua language. Make sure the script returns a
string that will be rendered as a prompt. An API is provided that can be used
to fetch information about the environment.

### Lua API

- `username()`: Current username
- `hostname()`: Current hostname
- `cwd()`: Current working directory
- `fg(color)`: Render a foreground color
- `bg(color)`: Render a background color

#### Colors

Some variables are defined to make it easier to work with colors:

- `black`
- `blue`
- `cyan`
- `green`
- `light_black`
- `light_blue`
- `light_cyan`
- `light_green`
- `light_magenta`
- `light_red`
- `light_white`
- `light_yellow`
- `magenta`
- `red`
- `red`
- `white`
- `yellow`
- `reset`

#### Integrations

External integration can be initialized by the following functions:

- `rbenv_init()`
- `git_init()`

They return an object with methods to talk to the external integration.

##### git integration

Available if the current directory or any of its ancestors is a git repository. It
calls `git` once in order to get status and branch information. Since it checks
for untracked files, it might be slow in big repositories.

Methods:

- `enabled()`: Current directory or any of its ancestors is a git repository
- `head()`: Current git HEAD (usually current branch name)
- `behind()`: Number of commits from current branch behind its remote
- `ahead()`: Number of commits from current branch ahead of its remote
- `index()`: Number of files changed in index (staged)
- `wt()`: Number of files changed in working tree
- `untracked()`: Whether there are untracked files

##### rbenv integration

Available if the current directory or any of its ancestors contain a
`.ruby-version` file

Methods:

- `enabled()`: Current directory or any of its ancestors contain a
  `.ruby-version` file
- `version()`: The contents of the `.ruby-version` file
