l <- lapply(
  manifests,
  \(x) {
    cat("parsing ", x, "\n")
    RcppTOML::parseTOML(file.path(VENDOR_PATH, x))$package
  }
)
