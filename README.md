# Auto Domain Blocker

This is a CLI app that help me get rid of internet addiction.

## Why

Every day I find myself lacking time to get my job done, but I keep
watching YouTube, BiliBili, or Twitter and other random stuff.

<b>I am going to f\*\*k up my life!</b>

I must quit my internet addiction, and I try to redirect those domains
to IP `0.0.0.0` in the `/etc/hosts` file.
But it is inconvenient to maintain the host file manually.
So I write this program to help me manage a list of blocked domains.

## How to use

Install rustup here: <https://rustup.rs/>. Setup rust and then clone this
repository via git. Run command `cargo build --release` to build the executable.

The executable is placed in the `target/release` directory. Move it into the
`/usr/local/bin` directory or any directory inside the `$PATH` variable.

Finally, use command `iac block` to read config and block domains.
You can use it with sudo, it can invoke
sudo itself when it doesn't have enough permission to write file.

See the help page by command `iac --help`.

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
