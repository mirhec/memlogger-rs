# memlogger

This is a simple CLI tool which enables you to log the memory and cpu consumption of one ore more processes into csv files.

```
USAGE:
    memlogger [OPTIONS] -p <PROCESS1,PROCESS2,...>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --interval <INTERVAL>            Sets the interval (in seconds) in which the memory is gathered [default: 1]
    -o, --output <FILE>                  Path to the output file [default = memlogger_[datetime].csv
    -p <PROCESS1,PROCESS2,...>... 
```

Here are some examples:

```bash
# Write memory and cpu consumption for each process that starts with "procA*" 
# every 10s.
memlogger -i 10 -o procA.csv -p procA
```

The resulting CSV file has the following columns:

- Timestamp
- PID
- Name
- Status
- Memory (kB)
- CPU

If you don't specify the output file it defaults to `memlogger_[datetime].csv`