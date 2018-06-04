# Roadrunner

A very fast and configurable prompt for shells.

## Overview

```sh
$ export ROADRUNNER_PROMPT='[${fg:red}%username%${fg:reset}@${fg:magenta}%hostname%${fg:reset}:#{rbenv:${fg:green}[Ruby %version%] }${fg:blue}%cwd%${fg:reset}#{git: ({${fg:magenta}%head%{reset}}{ ${fg:reset}{↓%behind%}{↑%ahead%}}{ {${fg:green}●%index%${fg:reset}}{${fg:red}+%wt%${fg:reset}}{${fg:reset}…%untracked%}{${fg:green}✓%clean%${fg:reset}})})}{reset}]
:) '
$ roadrunner
```

Output (with colors stripped):

```
[juanibiapina@MacBookPro:[Ruby 2.5.1] ~/roadrunner (master ↓2↑1 ●2+3…)]
:) 
```

## Syntax

Configuration is done using the `ROADRUNNER_PROMPT` environment variable. There
are five types of expressions: literals, colors, placeholders, sections and integrations.

### Literals

Literals as written out exactly as passed. All characters are allowed except:

- `{`
- `}`
- `%`

### Colors - `${fg:value}` or `${bg:value}`

Colors are delimited by `${` and `}`. The content between the brackets is the
type `fg` or `bg`, followed by `:` followed by either a terminal color name,
`reset`, or a color ANSI code. Examples:

- `${fg:red}` - foreground color red
- `${bg:blue}` - background color blue
- `${fg:reset}` - reset foreground color
- `${bg:23}` - background ANSI color 23

### Placeholders - `%name%`

Placeholders are surrounded by `%`. They are predefined and will cause an error
if they cannot be resolved. Currently available placeholders are:

- `%cwd%`: Path of current working directory ($HOME is replaced with `~`)
- `%hostname%`: Machine hostname
- `%username%`: Current user name

More placeholders are available inside specific integrations.

### Sections - `{...}`

Sections are optional parts. They are only rendered if at least one placeholder
or nested section inside the section renders. Example:

```sh
{Name is %name%}
```

If the placeholder `%name` doesn't render anything, the whole section is
ignored. Sections can be nested.

### Integrations - `#{tag:...}`

Integrations are delimited by `#{` and `}`. Inside an integration, a tag is used
to identify the type of integration, which will determine if this integration
will be rendered at all. After the tag followed by a `:`, any expression is
allowed. Extra placeholders are defined for each integration.  Example:

```
$ export ROADRUNNER_PROMPT="#{git:(%head%)}"
```

This outputs the current git HEAD in parenthesis if inside a git repository.
Otherwise it prints nothing.

#### git integration

Triggers when current directory or any of its ancestors is a git repository. It
calls `git` once in order to get status and branch information. Since it checks
for untracked files, it might be slow in big repositories. All placeholders are
precalculated when the integration is triggered, regardless of being used.
Placeholders:

- `%head%`: Current git HEAD (usually current branch name)
- `%behind%`: Shows number of commits from current branch behind its remote
- `%ahead%`: Shows number of commits from current branch ahead of its remote
- `%index%`: Number of files changed in index (staged)
- `%wt%`: Number of files changed in working tree
- `%untracked%`: Triggers if there are untracked files (but renders nothing)
- `%clean%`: Triggers if there are no changes in index or working directory and
  no untracked files (but renders nothing)

#### rbenv integration

Triggers when current directory or any of its ancestors contain a
`.ruby-version` file

- `%version%`: The contents of the `.ruby-version` file
