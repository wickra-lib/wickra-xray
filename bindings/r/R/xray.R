#' The wickra-xray library version.
#' @return A version string.
#' @export
wkxray_version <- function() {
  .Call(C_wkxray_version)
}

#' Build an xray from a spec JSON string.
#' @param spec_json A JSON spec string.
#' @return A `wickra_xray` handle (an external pointer).
#' @export
wkxray_new <- function(spec_json) {
  .Call(C_wkxray_new, spec_json)
}

#' Apply a command JSON and return the resulting response JSON.
#' @param xray An xray handle from [wkxray_new()].
#' @param cmd_json A command JSON string.
#' @return The response as a JSON string.
#' @export
wkxray_command <- function(xray, cmd_json) {
  .Call(C_wkxray_command, xray, cmd_json)
}
