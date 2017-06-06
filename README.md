# rslcli - Rust SL (Stockholm Public Transport) cli interface (unoffical)

The missing interface for your daily commute in Stockholm.

## Usage

Getting the available travel option:

```
$ rslcli Stadshagen Slussen
```

The SL API, which is used in the background is pretty smart and will figure
out what station you meant. So specifing 'Centralen' will be matched to
'T-Centralen'.

To show more travel alternatives use the `-n` parameter. The following will list
3 different trips.

```
$ rslcli -n 3 Stadshagen Centralen
```

For more information use `rslcli -h`.

## TODO

* update hyper to latest version

## License

Copyright (C) 2017 by Armin Widegreen

This is free software, licensed under The [ISC License](LICENSE).

