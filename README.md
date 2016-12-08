# libpresent

This small library implements the PRESENT lightweight block cipher. The
algorithm was first presented in [this paper](https://link.springer.com/chapter/10.1007%2F978-3-540-74735-2_31).
Although the cipher is designed to be implemented directly on hardware,
this project aims to provide it in software, which can be used e.g. for
testing purposes.

This project is independent of the original paper and not affiliated with
the authors of that paper in any way.

## Security

The algorithm should, according to the original authors, not be used in
environments where a high security level is required. This is caused by
the high priority on performance, which entails relatively small keys
(80 bit). This implementation, however, should not be used in any
environment where security is important, because I do not guarantee
that the algorithm is implemented in a secure manner (it most likely
is not).

## License

The code is licensed under the terms of the GNU Lesser General Public License
(LGPL) Version 3. For more information, please see the LICENSE.md file.
