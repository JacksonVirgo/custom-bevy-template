# Bevy Template

Template to quickly spin up a project without needing to write out a lot of boilerplate.

## Using the Template (without having that "generated from" tag)
This is a little tedious, but Github does not seem to have a native way to remove this tag.
- Create a repo (r1) using this template.
- Mark r1 as a template.
- Create a repo (r2) using r1's template
- Delete r1

## Important Notes
- The renderer uses `default_nearest` by default.

## Features
- Input management using [leafwing](https://github.com/Leafwing-Studios/leafwing-input-manager)
- /bin/game.rs for the main function to allow for easy other builds (level editor, etc)
