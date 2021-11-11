# `wait-until`
Repeatedly run a command until it's successful.

## Wait until a server is up
```sh
wait-until ping -c1 example.com && echo server is up
```

## Pause between runs of your command
```sh
wait-until --wait 1s ping -c1 example.com
```

## Print the output of each run
```
~$ wait-until --verbose ping -c1 example.com
PING example.com (93.184.216.34) 56(84) bytes of data.
64 bytes from 93.184.216.34 (93.184.216.34): icmp_seq=1 ttl=49 time=106 ms

--- example.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss, time 0ms
rtt min/avg/max/mdev = 106.226/106.226/106.226/0.000 ms
~$ 
```

## Turn on logging
```
~$ RUST_LOG=wait_until wait-until true
 DEBUG wait_until > opt = Opt { verbose: false, wait: None, cmd: ["true"] }
 DEBUG wait_until > cmd = "true"
 DEBUG wait_until > status = ExitStatus(ExitStatus(0))
~$
```

# Notes
Better APIs exist for kernel objects like files - see `inotifywait`.
