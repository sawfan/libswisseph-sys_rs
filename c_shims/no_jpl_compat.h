#ifndef SWISSEPH_SYS_NO_JPL_COMPAT_H
#define SWISSEPH_SYS_NO_JPL_COMPAT_H

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Compatibility declarations used when building Swiss Ephemeris without
 * swejpl.c for browser wasm32-unknown-unknown.
 *
 * sweph.c still references some JPL constants and helper entry points even
 * when JPL support is disabled. The real implementations normally come from
 * swejpl.c / swejpl.h; for browser WASM we provide stubs in no_jpl_compat.c.
 */

#ifndef J_MERCURY
#define J_MERCURY 0
#endif

#ifndef J_VENUS
#define J_VENUS 1
#endif

#ifndef J_EARTH
#define J_EARTH 2
#endif

#ifndef J_MARS
#define J_MARS 3
#endif

#ifndef J_JUPITER
#define J_JUPITER 4
#endif

#ifndef J_SATURN
#define J_SATURN 5
#endif

#ifndef J_URANUS
#define J_URANUS 6
#endif

#ifndef J_NEPTUNE
#define J_NEPTUNE 7
#endif

#ifndef J_PLUTO
#define J_PLUTO 8
#endif

#ifndef J_MOON
#define J_MOON 9
#endif

#ifndef J_SUN
#define J_SUN 10
#endif

#ifndef J_SBARY
#define J_SBARY 11
#endif

#ifndef J_EMB
#define J_EMB 12
#endif

#ifndef J_NUT
#define J_NUT 13
#endif

#ifndef J_LIB
#define J_LIB 14
#endif

int swi_open_jpl_file(double *ss, char *fname, char *fpath, char *serr);
void swi_close_jpl_file(void);
int swi_pleph(double tjd, int ntarg, int ncent, double *rrd, char *serr);
int swi_get_jpl_denum(void);

#ifdef __cplusplus
}
#endif

#endif
