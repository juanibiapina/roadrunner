# Roadrunner

A very fast and configurable prompt for shells.

## Overview

```sh
$ export ROADRUNNER_PROMPT="[%username%@%hostname%:{rbenv:%version% }%cwd%{git: (%branch%)}]"
$ roadrunner
```

Output:

```
[juanibiapina@MacBookPro:/Users/juanibiapina/roadrunner (master)]
```

## Syntax

Configuration is done using the `ROADRUNNER_PROMPT` environment variable. There
are three types of expressions: literals, placeholders and sections.

### Literals

Literals as written out exactly as passed. The following characters are
accepted:

```
[] : () @ <space>
```

Pull requests to handle other characters and alphanumeric in general are
welcome.

### Placeholders

Placeholders are surrounded by `%`. They are predefined and will cause an error
if they cannot be resolved. Currently available placeholders are:

- `%cwd%`: Path of current working directory
- `%hostname%`: Machine hostname
- `%username%`: Current user name

### Sections

Sections are delimited by `{` and `}`. Inside a section, a tag is used to
identify the type of section, which will determine if this section will be
rendered at all. After the tag followed by a `:`, any literals are allowed plus
any placeholders defined by that specific section. Example:

```
$ export ROADRUNNER_PROMPT="{git:(%branch%)}"
```

This outputs the current git branch in parenthesis if inside a git repository.
Otherwise it prints nothing.

#### git section

Triggers when current directory is a git repository. Placeholders:

- `%branch%`: Current git branch name

#### rbenv section

Triggers when current directory contains a `.ruby-version` file

- `%version%`: The contents of the `.ruby-version` file
