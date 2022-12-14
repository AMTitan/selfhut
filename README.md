# [Selfhut](https://git.arthurmelton.com)

This is my git site, it took inspiration from [Sourcehut](https://sourcehut.org/).
Its not a fork but its practically a remake, just for my use case. Please support
Drew DeVault at [https://sourcehut.org/pricing/](https://sourcehut.org/pricing/).
If you are going to run this yourself. I really recommend supporting Drew DeVault
because without him this would not exist!

## Install

To install you will need:

- git
- rust
- libgit2

Once you have these just run these steps.

```sh
git clone https://git.arthurmelton.com/selfhut
cd selfhut
cargo build --release
```

The binary executable will be at `./target/release/selfhut`, you can just run this
and your server should start working!

## Hosting

Simply run the binary and a website is going to be hosted at port 8000. I would
also recommend using something like nginx to change the port and to do caching. If 
you want to change the config (this would make sense because at default it only
has a example user), edit the file in `$(XDG_CONFIG_HOME)/selfhut/selfhut.toml`
to your liking. To add a `favicon.ico` you need to add it to 
`$(XDG_CONFIG_HOME)/selfhut/favicon.ico`. 

## Config

This would be a server config that has every attribute set.
```toml
name = "Billy Bob Jr"
description = "I am supper cool and think that [This](https://exmaple.com) is really cool!"
git_location = "/var/git"
domain = "https://billyscoolwebsite.com"
payment_link = "https://paypal.me/billy" # this is optinal, if you dont want to take donations then just remove the line dont set it to ""

[mailing_list] # This whole section is optinal, just remove it if you dont want email support
password = "********"
imap_url = "imap.billyscoolwebsite.com"
port = 993
```

## Repo Config

A repo config is a file called `repo.toml` in the `HEAD` of your git repo. This
controls some things like like the repo description and website. A example
would be this.
```toml
description="A really cool program I made"
website="https://cool-program.com"
```

## Setup git

You can do all the things in this to get fully setup. You will need to use ssh 
git to make commits. You will need to run the `update-server-info` that is below
after you do the `git init`. [https://landchad.net/git](https://landchad.net/git)

## Making a new git

To make a new git repo you need to run 
```sh
git init --bare my-repo.git
```
After you do this you will need to cd into the directory and make a file at the 
path `hooks/post-update` and set its contents to
```sh
#!/bin/sh

exec git update-server-info
```
This should create the git and make it so that people can actually clone it!

## How to edit a repo

The way that you are going to edit a repo you own is through ssh.

## Email

This can also run on any email server though a open imap connection. The actual
mailing list and creation of accounts are handled through different programs.
For every repo you need to make a email for it (ex. if I have a repo called
"My-Cool-Program" you need to make an email for it called 
"My-Cool-Program@example.com").
