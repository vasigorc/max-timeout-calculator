# max-timeout-calculator

Simple calculator to figure out what should be a maximum backoff for a restart / retry mechanism, as for example in this [Akka library](https://doc.akka.io/docs/akka/current/stream/stream-error.html#delayed-restarts-with-a-backoff-operator).

To run it simply run:

```shell
cargo run
```

I've removed extra logging, but the input - processing - output dialog in a terminal would look as follows:

```shell
➜  max-timeout-calculator git:(main) ✗ cargo run
...`
Enter min_backoff_seconds: u64
10
Enter cap_total_wait_seconds: u64
7200
Enter random_factor: f64
1.5
Current duration is 10s
Current duration is 15s
Current duration is 22.5s
Current duration is 33.75s
Current duration is 50.625s
Current duration is 75.9375s
Current duration is 113.90625s
Current duration is 170.859375s
Current duration is 256.2890625s
Current duration is 384.43359375s
Current duration is 576.650390625s
Current duration is 864.975585938s
Current duration is 1297.463378907s
Current duration is 1946.195068361s
Current duration is 2919.292602541s
The max_timeout_duration for provided input data should be 2919 seconds
```