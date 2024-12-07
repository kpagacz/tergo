% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/styling.R
\name{style_pkg}
\alias{style_pkg}
\title{Style a package}
\usage{
style_pkg(
  path = ".",
  config_file = "tergo.toml",
  configuration = list(),
  force = FALSE,
  extensions = c(".R", ".r")
)
}
\arguments{
\item{path}{(\code{character}) The path to the package.}

\item{config_file}{(\code{character}) The path to the configuration file}

\item{configuration}{(\code{list}) The path to the configuration for formatting}

\item{force}{(\verb{logical(1}) Whether to format the files even
if no package was found. \code{TRUE} - format the \code{.R} and \code{.r} files
found in the directory (recursive). \code{FALSE} exit without formatting
anything.}

\item{extensions}{(\code{character}) The extensions of the files to format.}
}
\description{
Style a package
}
