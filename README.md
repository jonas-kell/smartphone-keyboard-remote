# Smartphone Keyboard Remote

Was unhappy finding a server-app combo, that lets me use my smartphone as a keyboard in my LAN.
Everything riddled with bugs and disconnection issues.
Also why trust someone elses encryption and secrecy, when you can fail to implement your stuff securely yourself.

## Hosted version

Hosted and installable (as a [PWA](https://web.dev/progressive-web-apps/)) under [https://jonas-kell.github.io/smartphone-keyboard-remote/#/](https://jonas-kell.github.io/smartphone-keyboard-remote/#/).

![QR](QR.png)

## Run the backend

```cmd
# make sure to have rust installed: https://www.rust-lang.org/tools/install

sudo apt-get install libxdo-dev

# make sure you have node installed: https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating

cd client
npm install
npm run build
cd ..
cd server
cargo run
```

## Dev the frontend

```cmd
# make sure you have node installed: https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating

cd client
npm install
npm run dev
```

<!--
https://cthedot.de/icongen/
https://realfavicongenerator.net/
 -->
