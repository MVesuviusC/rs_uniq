# rs_uniq

## Purpose
The way that the normal `uniq` command works is that it takes in a sorted list of lines and removes any duplicates. This is useful, but sometimes you want to remove duplicates from a list that is not sorted. This is where `rs_uniq` comes in. It reads in a list of lines and removes any duplicates. It does this by reading in the lines and storing them in a hash table with the value being the number of observed copies. The hash table is then written out to stdout with the duplicates removed. If -c is passed in, then the number of copies of each line is printed out as well as a separate column.

## Usage

```bash

```

## Installation



## Notes
This is a project I'm using to practice rust. I've been programming for quite a while, so I'm going to try to put this together well, but I can't make any guarantees that something better doesn't already exist. I'm doing this for fun and to learn, so if you have any suggestions or feedback, please let me know.

## to do
Write like, anything.