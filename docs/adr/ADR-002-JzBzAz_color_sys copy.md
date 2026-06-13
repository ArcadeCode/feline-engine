# ADR-002 | JzBzAz color system 

**Statut** : Rejected  
**Date** : 2026-06-13

## Context
I discovered the JzBzAz color system, which is a perceptually uniform color space that is designed to be more accurate and consistent than other color spaces like RGB or HSL.

## Decision
JzBzAz is a complex color space that may not be necessary for a simple 2D games. While it offers advantages in terms of color representation and manipulation, it also adds complexity to the engine's color handling.

## Results
- Rejected the use of JzBzAz color system in favor of a simpler sRGB color space.
- Possibly adding support for JzBzAz, HSL or HSV color spaces in the future if needed but not for the initial release.

## Considered alternatives:
- Using HSL or HSV color spaces: These are simpler and more commonly used in 2D graphics applications, but they may not offer the same level of perceptual uniformity as JzBzAz.