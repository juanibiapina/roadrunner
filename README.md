# Roadrunner

A very fast and configurable prompt for shells.

## Overview

```sh
$ export ROADRUNNER_PROMPT='[{red}%username%{reset}@{magenta}%hostname%{reset}:{rbenv:{green}[Ruby %version%] }{blue}%cwd%{reset}{git: ({magenta}%branch%{reset})}{reset}]
:) '
$ roadrunner
```

Output (with colors stripped):

```
[juanibiapina@MacBookPro:[Ruby 2.5.1] /Users/juanibiapina/roadrunner (master)]
:) 
```

## Syntax

Configuration is done using the `ROADRUNNER_PROMPT` environment variable. There
are four types of expressions: literals, placeholders, sections and colors.

### Literals

Literals as written out exactly as passed. All characters are allowed except:

- `{`
- `}`
- `%`

### Placeholders

Placeholders are surrounded by `%`. They are predefined and will cause an error
if they cannot be resolved. Currently available placeholders are:

- `%cwd%`: Path of current working directory ($HOME is replaced with `~`)
- `%hostname%`: Machine hostname
- `%username%`: Current user name

### Sections

Sections are delimited by `{` and `}`. Inside a section, a tag is used to
identify the type of section, which will determine if this section will be
rendered at all. After the tag followed by a `:`, any literals or colors are
allowed plus any placeholders defined by that specific section. Example:

```
$ export ROADRUNNER_PROMPT="{git:(%branch%)}"
```

This outputs the current git branch in parenthesis if inside a git repository.
Otherwise it prints nothing.

### Colors

Colors are also delimited by `{` and `}`, but they don't have a tag. The
content between the brackets can be either a terminal color name, `reset`, or a
color ANSI code. Examples:

- `{red}` - color red
- `{blue}` - color blue
- `{reset}` - reset color
- `{23}` - ANSI color 23

#### git section

Triggers when current directory or any of its ancestors is a git repository.
Placeholders:

- `%branch%`: Current git branch name

#### rbenv section

Triggers when current directory or any of its ancestors contains a
`.ruby-version` file

- `%version%`: The contents of the `.ruby-version` file
