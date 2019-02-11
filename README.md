# bin.
a pastebin.



There's no good open source pastebin solutions. I'm sorry to everyone who has one on GitHub but I have to say it. We try to cram as many little features as humanly possible into them and still try and call them minimalist. I don't want to run Redis, I don't want commenting functionality, I don't want self-destructing or time bomb messages and I especially don't want social media integration—I don't know about you but normally I just need to send a quick little snippit of code to someone, it doesn't need a title and I don't *really* mind when it disappears as long as its around long enough for them to see. Honestly, [I'm guilty of it myself](https://github.com/w4/hidden-note), we've all made a pastebin at one point or another but when it comes to making one to release to the public we create abominations.

[bin.](https://bin.doyle.la/) is written in Rust in around 100 lines of code. It's fast, it's simple, there's code highlighting and you can ⌘+A without going to the 'plain' page. Revolutionary in the pastebin industry, disrupting markets and pushing boundaries never seen before.

##### curl support?

```bash
$ curl -X PUT --data 'hello world' bin.doyle.la
https://bin.doyle.la/cateettary
$ curl https://bin.doyle.la/cateettary
hello world
```

##### how do you run bin?

```bash
$ ./bin
```

##### good one, what settings are there?

bin. uses [rocket](https://rocket.rs) so you can add a [rocket config file](https://api.rocket.rs/v0.3/rocket/config/) if you like. You can set `ROCKET_PORT` in your environment if you want to change the default port (8820).

bin's only configuration value is `BIN_BUFFER_SIZE` which defaults to 2000. Change this value if you want your bin to hold more pastes.

##### how does syntax highlighting work?

To get syntax highlighting you need to add the file extension at the end of your paste URL.