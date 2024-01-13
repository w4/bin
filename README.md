# bin
a paste bin.

A paste bin that's actually minimalist. No database requirement, no commenting functionality, no self-destructing or time bomb messages and no social media integration—just an application to quickly send snippets of text to people.

[bin](https://bin.gy/) is written in Rust in around 300 lines of code. It's fast, it's simple, there's code highlighting and you can ⌘+A without going to the 'plain' page. It's revolutionary in the paste bin industry, disrupting markets and pushing boundaries never seen before.

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

```
$ ./bin

Usage: bin [<bind_addr>] [--buffer-size <buffer-size>] [--max-paste-size <max-paste-size>]

a pastebin.

Positional Arguments:
  bind_addr         socket address to bind to (default: 127.0.0.1:8820)

Options:
  --buffer-size     maximum amount of pastes to store before rotating (default:
                    1000)
  --max-paste-size  maximum paste size in bytes (default. 32kB)
  --help            display usage information
```

##### is there curl support?

```bash
$ curl -X PUT --data 'hello world' https://bin.gy
https://bin.gy/cateettary
$ curl https://bin.gy/cateettary
hello world
```

##### how does syntax highlighting work?

To get syntax highlighting you need to add the file extension at the end of your paste URL.

##### running it behind Apache httpd as reverse proxy
Add the following to your httpd.conf file:
```
LoadModule substitute_module modules/mod_substitute.so
...
<Location /bin/>
    ProxyPass http://localhost:80/
    ProxyPassReverse /
    AddOutputFilterByType SUBSTITUTE text/html
    Substitute "s|action=\"/\"|action=\"/bin/\"|"
</Location>
```
It exposes bin on http://HOST/bin
