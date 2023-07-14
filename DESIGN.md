# Hare Design


## Objective

Supervisor trees are awesome, but you've generally got to write code to use
them.  Hare seeks to remedy this by providing a command-line tool that makes it
easy to pack the commands you run into a supervisor tree.

## Desired Features

- job control
- monitoring
- hup and resume abilities

## Usage Examples

Calculate the fibonacci numbers of 1 through 50 using 16 supervised workers.

```
seq 1 50 | psup supervise --redundancy=16 -- fib.sh
```