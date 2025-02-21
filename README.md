**dsntk** | DecisionToolkit

# Histogram generator

[![MIT licensed][mit-badge]][mit-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]
[![Contributor Covenant][cc-badge]][cc-url]

[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: LICENSE
[apache-notice-url]: NOTICE
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[cc-url]: CODE_OF_CONDUCT.md

## Overview

Histogram generator reads an input file named **benchmarks.txt** placed in [**data**](./data) directory.
Input file must contain results from Rust benchmarks, exactly like in the example shown below:

```
test compatibility::level_2::dmn_2_0001::_0001       ... bench:         631.09 ns/iter (+/- 8.76)
test compatibility::level_2::dmn_2_0002::_0001       ... bench:         558.11 ns/iter (+/- 19.47)
test compatibility::level_2::dmn_2_0003::_0001       ... bench:       1,089.91 ns/iter (+/- 15.71)
test compatibility::level_2::dmn_2_0003::_0002       ... bench:       1,191.99 ns/iter (+/- 15.49)
test compatibility::level_2::dmn_2_0004::_0001       ... bench:       3,951.72 ns/iter (+/- 36.64)
test compatibility::level_2::dmn_2_0004::_0002       ... bench:       3,999.83 ns/iter (+/- 49.33)
test compatibility::level_2::dmn_2_0004::_0003       ... bench:       3,912.72 ns/iter (+/- 50.94)
test compatibility::level_2::dmn_2_0005::_0001       ... bench:       3,047.81 ns/iter (+/- 49.33)
test compatibility::level_2::dmn_2_0005::_0002       ... bench:       3,060.80 ns/iter (+/- 24.60)
test compatibility::level_2::dmn_2_0005::_0003       ... bench:       2,991.84 ns/iter (+/- 21.22)
test compatibility::level_2::dmn_2_0006::_0001       ... bench:       3,079.90 ns/iter (+/- 27.66)
test compatibility::level_2::dmn_2_0006::_0002       ... bench:       3,088.53 ns/iter (+/- 23.93)
test compatibility::level_2::dmn_2_0006::_0003       ... bench:       3,038.80 ns/iter (+/- 31.30)
test compatibility::level_2::dmn_2_0007::_0001       ... bench:       2,103.06 ns/iter (+/- 24.66)
test compatibility::level_2::dmn_2_0007::_0002       ... bench:       2,058.55 ns/iter (+/- 29.46)
test compatibility::level_2::dmn_2_0007::_0003       ... bench:       2,043.08 ns/iter (+/- 22.84)
```

As a result, this application generates output files (also placed in [**data**](./data) directory): 

- **README.md** - performance summary.
- **benchmarks.hgrm** - histogram data file.
- **benchmarks.svg** - histogram chart in SVG format.

To generate PNG file from SVG file, use the [ImageMagic](https://imagemagick.org/):

```
$ magick -size 2000x600 data/benchmarks.svg data/benchmarks.png
```

There is a [Task](https://taskfile.dev) command provided to automate generating the histogram in PNG format.

```
$ task gen
```

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions are greatly appreciated.
If you would like to get involved, please don't hesitate to reach out to us.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
