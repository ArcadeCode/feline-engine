# VectorN

`fe::utils::Vector<T, N>` is a class to perform vector manipulations of type `T` and dimensions `N`, for example :
```cpp
#include "felineEngine.h"
fe::utils::Vector<float, 3> = (10.5, 40.9, 88.0);
fe::utils::Vector<float, 3> = fe::utils::Vector<float, 3>.new(10.5, 40.9, 88.0);
```

To help developers, there is alias for some Vectors expressions, [see it here](./constants.md)