# Roadrunner

A fast and configurable prompt for shells.

## Overview

```sh
$ export ROADRUNNER_PROMPT='#{fg(reset)};[#{fg(red)}#{username()}#{fg(reset)}@#{fg(magenta)}#{hostname()}#{fg(reset)}:;?rbenv:#{fg(green)}[Ruby #{version}] ;#{fg(blue)}#{cwd()};?git: #{fg(reset)}({#{fg(magenta)}#{tr(head)}}{ #{fg(reset)}{↓#{tr(behind)}}{↑#{tr(ahead)}}}{ {#{fg(green)}●#{tr(index)}}{#{fg(red)}+#{tr(wt)}}{#{fg(reset)}…#{tr(untracked)}}{#{fg(green)}✓#{tr(clean)}}}#{fg(reset)});#{fg(reset)}]
:) '
$ roadrunner
```

Output (with colors stripped):

```
[juanibiapina@MacBookPro:[Ruby 2.5.1] ~/roadrunner (master ↓2↑1 ●2+3…)]
:) 
```

Benchmark on my local machine using
[hyperfine](https://github.com/sharkdp/hyperfine). These numbers may very
depending on the state of the git repo.

```
Time (mean ± σ):       9.6 ms ±   1.0 ms    [User: 3.5 ms, System: 4.1 ms]
Range (min … max):     8.5 ms …  14.8 ms    250 runs
```

## Usage

Add the main binary to your path and export a `ROADRUNNER_PROMPT` environment
variable with your prompt configuration. Refer to the example in the overview
and the syntax section for more details.

### Zsh

```sh
setopt prompt_subst # enable prompt substitution
export ROADRUNNER_PROMPT='...'
PROMPT='$(roadrunner)'
```

## Syntax

Configuration is done using the `ROADRUNNER_PROMPT` environment variable. There
are four types of expressions: literals, interpolations, sections and conditionals.

### Literals

Literals as written out exactly as passed. All characters are allowed except:

- `{`
- `}`
- `;`

### Interpolations

Interpolations are delimited by `#{` and `}`. Inside an interpolation variables can be referenced and functions can be called. Example:

`ROADRUNNER_PROMPT=#{username()}` => `juanibiapina`

### Sections

Sections are conceptual divisions in the prompt line. They are simply separated
by `;`. The `;` character itself is not rendered. Example:

`part1;part2;part3` => `part1part2part3`

A section can be tagged, in which case it has specific rules to render
depending on the tag name. Example with the `rbenv` tag:

`?rbenv:part1;part2` => `part1part2`

Inside the tagged section extra variables are defined.

#### git section

Renders when the current directory or any of its ancestors is a git repository.
It calls `git` once in order to get status and branch information. Since it
checks for untracked files, it might be slow in big repositories. All variables
are precalculated when the section is rendering, regardless of the variable
being used.

Variables:

- `head`: Current git HEAD (usually current branch name)
- `behind`: Number of commits from current branch behind its remote
- `ahead`: Number of commits from current branch ahead of its remote
- `index`: Number of files changed in index (staged)
- `wt`: Number of files changed in working tree
- `untracked`: true if there are untracked files
- `clean`: true if there are no changes in index or working directory and
  no untracked files

#### rbenv integration

Renders when current directory or any of its ancestors contain a
`.ruby-version` file

- `version`: The contents of the `.ruby-version` file

### Conditionals

Conditionals are areas delimited by `{` and `}`. By default a conditional will
not render anything inside of it. In order to make it render, the `tr` function
must be called inside an interpolation. This is useful to hide parts of the
line in case some conditions are met.

Examples that do not render anything:

- `{}`
- `{text}`
- `{#{red}}`

Examples that render:

- `{#{tr(red)}}` => `red`
- `{text #{tr(red)}} => `text red`

Conditionals can be nested. A parent conditional renders if at least one `tr`
appears inside an interpolation or any child conditionals render. Example:

`{ {↓#{tr(behind)}}{↑#{tr(ahead)}}}`

The outer conditional renders if any of the inner conditionals render. The
inner conditionals render if `behind` is different than 0 and `ahead` is
different than 0 respectively. Notice how the conditionals are used to hide the
literal arrow symbols in case they are not needed.

## Reference

### Variables

Variables can be of type `String`, `Number` or `Boolean`.

#### Colors:

Variables for color names are always defined. They simply return the names of
the colors as string. They can be used with `fg` and `bg` functions to generate
the escape code to change the prompt color.

- `reset`
- `black`
- `red`
- `green`
- `yellow`
- `blue`
- `magenta`
- `cyan`
- `white`

### Functions

- `cwd()`: Path of current working directory ($HOME is replaced with `~`)
- `hostname()`: Machine hostname
- `username()`: Current user name
- `tr(variable)`: Triggers a rendering of the surrounding conditional according
  to the value of the variable:
  - String: triggers when non empty
  - Numbers: triggers when different than 0
  - Boolean: triggers when true
- `fg(variable)`: Sets the foreground color
- `bg(variable)`: Sets the background color
