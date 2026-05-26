# Core design

The Code under the `<felineEngineCore>` library, without this library none project can run. The code contain all main computing tools and sensible functions of the engine.

## Languages
The core is mainly write in Rust for mainy reasons :
1. **Memory safe** : The ownership help the engine to know for sure than a ressource is handle safely. It's essential to parts such as the [Job scheduler](./job/main.md) who need atomic concurrent computing logic.
2. **Fastest** : Rust is more fast than C++ or C.

### Rust implemented components
- **Gameloop** : The Gameloop is the center of everything, the loop call all logics, prepare interpollations of ticks. It also manage when the rendered is called.
- **Job scheduler** : As say below, the urge for safe memory allocation when using conccurent computing logic make Rust the best tool for this type of component.

> TODO: Do we need EDA sys to be coded in Rust ?