# bin
a paste bin.

A paste bin that's actually minimalist. No database requirement, no commenting functionality, no self-destructing or time bomb messages and no social media integration—just an application to quickly send snippits of text to people.

[bin](https://bin.doyle.la/) is written in Rust in around 200 lines of code. It's fast, it's simple, there's code highlighting and you can ⌘+A without going to the 'plain' page. It's revolutionary in the paste bin industry, disrupting markets and pushing boundaries never seen before.

##### so how do you get bin?

Download the latest version from the [releases](https://github.com/w4/bin/releases) page, extract it and run the `./bin` executable. You can also compile it from source using Cargo if you swing that way:

```bash
# nix-shell provides an environment with rust/cargo installed
$ nix-shell

[nix-shell:~/Code/bin]$ cargo build --release
   Compiling bin v1.0.0 (/Users/jordanjd/Code/bin)
    Finished release [optimized] target(s) in 3.61s

[nix-shell:~/Code/bin]$ ./target/release/bin
    ...
```

##### how do you run it?

```bash
$ ./bin
```

##### funny, what settings are there?

bin uses [rocket](https://rocket.rs) so you can add a [rocket config file](https://api.rocket.rs/v0.3/rocket/config/) if you like. You can set `ROCKET_PORT` in your environment if you want to change the default port (8820).

bin's only configuration value is `BIN_BUFFER_SIZE` which defaults to 2000. Change this value if you want your bin to hold more pastes.

##### is there curl support?

```bash
$ curl -X PUT --data 'hello world' bin.doyle.la
https://bin.doyle.la/cateettary
$ curl https://bin.doyle.la/cateettary
hello world
```

##### how does syntax highlighting work?

To get syntax highlighting you need to add the file extension at the end of your paste URL.
