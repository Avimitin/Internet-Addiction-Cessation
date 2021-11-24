# Auto Domain Blocker

This is a CLI app that help me get rid of internet addiction.

## Why

I am going to have a examination in December but I keep watching YouTube
video and BiliBili video. I am going to f\*\*k up my score. I try to redirect
those domain to IP `0.0.0.0` in the `/etc/hosts` file. It works for a few
days. But I am lack of self-awareness. Also it is unconvenient to maintain
the host file manually. So I write this program to help me get rid of the
current situation.

## How to use

Install rustup here: https://rustup.rs/. Then clone this repository via
git. And run command `cargo build --release` to build the executable.

The executable is under the `target/release` directory. Move it into the
`/usr/local/bin` directory or any directory inside the `$PATH` variable.

Finally, use command `auto-domain-blocker block` to generate list of URL
and insert them into `/etc/hosts` file.

See help page with command `auto-domain-blocker --help`.

## How to write configuration

I am using `TOML` as the configuration language. Create a file and name it
with extension `.toml`. The program will read config at the current directory.
You can also use the `-c/--config` option to specify the configuration path.

An example configuration:

```toml
[duration]
start = "08:30"  # program will not generate URL before this time
end = "21:30"    # program will not remove URL before this time

[block_domains]
"youtube.com"=["www"]                # use the domain as the key, and assign a list of subdomains
"bilibili.com"=["www", "live", "@"]  # @ means "bilibili.com" itself without subdomain
```

## License

MIT License (MIT)
