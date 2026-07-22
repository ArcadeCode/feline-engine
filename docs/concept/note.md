# Notes

This is my personal notes on this project.

### Funny targets integration
It can be funny to try adding compile target for unused architecture such as consoles like the Nintendo Wii, 3DS, PlayStation 1. Only to try pushing the limit of the engine optimization. The main task will be to develop a new `IGraphicBackend` using the console SDK. We also be very limited by the EDA + ECS system which requiert a lot of ram to work.

## Notes dedicate for CLI tools
> Normally Feline Editor will implement each CLI tools with GUI.

### Size & Memory usage
It will be really useful for optimization purpose to have an helper to measure each lib and the full engine on disk and in RAM, also when being integrated with an Game.

## Notes dedicate for Feline Editor

### Graph viewer
A graph viewer will be very useful for many reasons such as :
- GUI tool for edition (Visual Novel choices).
- Files connexions (to see usage of Ressources and debugging programs).
- Git branch of the current projet.

The graph will be represented an an adjacency matrix and will be show using imGui in Feline Editor.