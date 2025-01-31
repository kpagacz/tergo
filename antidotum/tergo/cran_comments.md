# Author's comments

## Package size

During my last submission, I was asked to decrease the size of the package tarball:

> Size of tarball: 7737948 bytes
> A CRAN package should not be larger than 5 MB. Please reduce the size.
> For more details:
> <https://contributor.r-project.org/cran-cookbook/general_issues.html#package-size>

`tergo` contains source code of the Rust dependencies (according to CRAN submission
guidelines) that are archived and compressed in an archive. The size of the tarball
exceeds 7 MB, which is over the recommended 5 MB. Any additional reduction in size
of the tarball is impossible without effectively copying and rewriting the Rust
dependencies. The original size of the uncompressed source code of the dependencies
is over 90MB.

Due to above, I kindly ask to alleviate the "soft" limit on the tarball size for
this package. The package size does not exceed the hard limit of 10 MB
in the guidelines and already has the team behind `nest` (the suite of `teal`
packages for exploratory data analysis) interested in using this package
once it is hosted on CRAN, no matter the tarball size.

## Writing to user's home directory

During my last submission, I was asked:

> Please ensure that your functions do not write by default or in your
> examples/vignettes/tests in the user's home filespace (including the
> package directory and getwd()). This is not allowed by CRAN policies.
> Please omit any default path in writing functions. In your
> examples/vignettes/tests you can write to tempdir().
> -> inst/bench.R; man/style_file.Rd; tools/configure.R
> For more details:
> <https://contributor.r-project.org/cran-cookbook/code_issues.html#writing-files-and-directories-to-the-home-filespace>

- I had moved inst/bench.R to data-raw/bench.R and added data-raw to .Rbuildignore.
- I changed the example in style_file to write to a tempfile() and unlink it once
  the example is finished.
- Regarding tools/configure.R: this file contains scripts to make sure
  that cargo, rustc and rustup are installed on the user's host.
  The only instance of explicit writing to the user's file system is to
  a temporary file, which is later unlinked with (`on.exit`). Nevertheless,
  I refactored `on.exit` out and use `unlink` directly in the code.
  If I missed something here, please let me know.
