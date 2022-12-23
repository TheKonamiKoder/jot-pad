# jot-pad #

- [jot-pad](#jot-pad)
  - [About](#about)
  - [Installation](#installation)
  - [How to use jot-pad?](#how-to-use-jot-pad)
    - [add](#add)
    - [del](#del)
    - [log](#log)
  - [License](#license)


## About ##
This is a very simple CLI tool made with rust to help you write quick notes to yourself. This is useful when an idea suddenly pops into your head, and you want to quickly *jot* it down somewhere for later.

## Installation ##
NOTE: jot-pad was written on Windows, so may not work on other operating systems as I have only tested it on Windows. 

To download jot-pad you can clone the repository like shown below:

```git clone https://github.com/TheKonamiKoder/jot-pad <your-repository-name>```

and then build the project yourself using cargo, if you want to edit the source code.

Or, you could just download the /bin directory. The /bin directory will have the jot-pad.exe file, which will be the main application and a jots.json file, which is where all of the jots are stored. After that, you can add the /bin directory to your PATH.

Then, you will be able to use jot-pad on the CLI

## How to use jot-pad? ##

jot-pad has 3 different commands:

- [add](#add)
- [del](#del)
- [log](#log)

### add ###

The `add` command adds a jot to the jots list.

The command takes two parameters. One of them is the `short` parameter. This is the short version of the jot - a few words which you want to rememer. Most jots should only be short.

The second parameter (which is optional) is the detailed parameter. This takes in a longer version of the jot which can contain more details. For most jots, this shouldn't really be necessary.

Although there is no limit to how many characters/words you can put in the short parameter, it is better to only put as little as possible. The same thing goes for the long parameter as well - jots as a whole should be relatively short and simple, mainly just notes to yourself in case you forget.

To use the command and only have a short jot you can run `jot-pad add "This is a short jot"` on the command line.

To add a jot with a detailed part you can run `jot-pad add "This is the short part of the jot" -d "This is the very very very very very very very long part of the jot with lots of boring details blah blah blah"`
to create a jot that is very long.

Depending on which terminal you use (it works in windows powershell, but doesn't work in command prompt), multi-line jots may work as well.

To do a multiline jot you can do:

```
jot-pad add "my jot short" -d "
my
long
multi
line
jot "
```

### del ###

The `del` command deletes commands from the jots list. If you want to delete a command, you must know its id (whenever a jot is added, it comes with an id. To see the id of your jots, you can run the command `jot-pad log`, which logs all of your commands to the screen), or otherwise jot-pad will delete all jots (of course you will be asked for confirmation).

To delete a single jot, when you know the id, you can do: `jot-pad del <jot-id>`

To delete all jots (be careful about this, as there is no way to get them back), you can do: `jot-pad del`

### log ###

The `log` command logs all your jots to the screen, and shows you their id and when they were written as well. In the future, I plan to add more functionality to the `log` command, but for now, it will remain like so.

## License ##

MIT License

Copyright (c) 2022 Pratyush Joshi

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.