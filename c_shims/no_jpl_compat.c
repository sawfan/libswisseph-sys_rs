#include "no_jpl_compat.h"

#include <stddef.h>
#include <string.h>

static void set_serr(char *serr) {
    const char *msg = "JPL ephemeris support is disabled in this WASM build";

    if (serr != NULL) {
        /*
         * Swiss Ephemeris error buffers are conventionally AS_MAXCH sized.
         * This shim avoids depending on that internal constant.
         */
        strcpy(serr, msg);
    }
}

int swi_open_jpl_file(double *ss, char *fname, char *fpath, char *serr) {
    (void)ss;
    (void)fname;
    (void)fpath;

    set_serr(serr);
    return -1;
}

void swi_close_jpl_file(void) {
}

int swi_pleph(double tjd, int ntarg, int ncent, double *rrd, char *serr) {
    (void)tjd;
    (void)ntarg;
    (void)ncent;

    if (rrd != NULL) {
        for (int i = 0; i < 6; i++) {
            rrd[i] = 0.0;
        }
    }

    set_serr(serr);
    return -1;
}

int swi_get_jpl_denum(void) {
    return 0;
}
