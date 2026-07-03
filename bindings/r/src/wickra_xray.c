/* R .Call glue for the wickra-xray C ABI hub. */
#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>
#include <stddef.h>
#include "wickra_xray.h"

/* --- handle lifetime ----------------------------------------------------- */

static void wkxray_finalize(SEXP ext) {
    WickraXray *h = (WickraXray *)R_ExternalPtrAddr(ext);
    if (h) {
        wickra_xray_free(h);
    }
    R_ClearExternalPtr(ext);
}

static WickraXray *handle_of(SEXP ext) {
    WickraXray *h = (WickraXray *)R_ExternalPtrAddr(ext);
    if (!h) {
        Rf_error("wickra-xray: handle is closed");
    }
    return h;
}

/* --- exported .Call entries ---------------------------------------------- */

SEXP wkxray_version(void) {
    return Rf_mkString(wickra_xray_version());
}

SEXP wkxray_new(SEXP spec_json) {
    WickraXray *h = wickra_xray_new(CHAR(STRING_ELT(spec_json, 0)));
    if (!h) {
        Rf_error("wickra-xray: invalid spec");
    }
    SEXP ext = PROTECT(R_MakeExternalPtr(h, R_NilValue, R_NilValue));
    R_RegisterCFinalizerEx(ext, wkxray_finalize, TRUE);
    UNPROTECT(1);
    return ext;
}

SEXP wkxray_command(SEXP ext, SEXP cmd_json) {
    WickraXray *h = handle_of(ext);
    const char *cmd = CHAR(STRING_ELT(cmd_json, 0));

    /* Length-out protocol: learn the length, then read into a caller buffer.
       Domain errors come back in-band as {"ok":false,...} JSON, not a negative
       code; only unusable arguments / a caught panic return < 0. */
    int len = wickra_xray_command(h, cmd, NULL, 0);
    if (len < 0) {
        Rf_error("wickra-xray: command failed (code %d)", len);
    }
    char *buf = (char *)R_alloc((size_t)len + 1, 1);
    wickra_xray_command(h, cmd, buf, (size_t)len + 1);
    return Rf_mkString(buf);
}

/* --- registration -------------------------------------------------------- */

static const R_CallMethodDef CallEntries[] = {
    {"wkxray_version", (DL_FUNC)&wkxray_version, 0},
    {"wkxray_new", (DL_FUNC)&wkxray_new, 1},
    {"wkxray_command", (DL_FUNC)&wkxray_command, 2},
    {NULL, NULL, 0}};

void R_init_wickraxray(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
