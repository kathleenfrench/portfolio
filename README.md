# kathleenfrench.co

_this is the repo for my personal portfolio_

----

## local dev

### make commands

simply running `make` in your terminal will output some useful commands for running the site locally

#### run it locally

to build and run the local environment, simply run: `make local`

#### hot reloading

if you want to toy around with things and enable hot reloading:
- for `rust` code: `make watch`
- for `css` code: `make hot-css`
- more TK

### ssl

_note: this is handled by the `make cert` target, and runs as part of `make local`_

1. install [mkcert](https://github.com/FiloSottile/mkcert)
2. generate a cert for your local dev: `mkcert 127.0.0.1`
3. change the names to `cert.pem` and `key.pem`
4. run `mkcert -install`
    - <small>_note_: you may need to reload the site in an incognito window for these changes to take effect</small>
5. site can now be reached at [https://127.0.0.1:3000](https://127.0.0.1:3000)