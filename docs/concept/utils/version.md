# `Version` class & `VersionBuild` class

Version is a custom class who handle `vX.Y.Zt` format. Where `X`, `Y`, `Z`represent an integer and `t` a char caracter mainly made to represent Alpha and Beta versions. `VersionBuild` inherit from `Version` and add another interger to represent the build. Finally representing by `vX.Y.Zt+b`

> By default `t` = empty space ASCII value. 

## Constructing an object
```cpp
#include <FelineEngine.hpp>

// Each functions from version shown here are compatible with VersionBuild
Version version = Version("v34")
Version version = Version().fromString("v34.4.8a");
Version version = Version().fromTyple((34, 4, 8));

Version version = Version().isAlpha();
Version version = Version().isBeta();

Version version = Version().getMajor();
Version version = Version().getMinor();
Version version = Version().getPatch();
Version version = Version().getPrerelease();

VersionBuild build = VersionBuild().getBuild();
VersionBuild build = VersionBuild("v3.5.4+2000")
```

## Comparing versions
```cpp
Version v1 = Version("v1.2.3");
Version v2 = Version("v1.3.0");

bool test = v1 > v2;
bool test = v1[0] > v2[0];

```