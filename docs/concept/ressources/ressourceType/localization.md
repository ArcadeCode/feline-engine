# `Ressource<Localization>` ressource type
The `Ressource<Localization>` ressource type in Feline Engine is designed to manage localization files used for supporting multiple languages in applications and games. This resource type allows developers to easily load, manage, and switch between different language files to provide a localized experience for users.

## Overview
To enable localization in your application, you can create a `Ressource<Localization>` instance by specifying the path to the localization file. The `RessourceManager` will handle the loading and caching of the localization data, ensuring efficient memory usage and preventing duplicate loading.

For each language supported in your application, you will typically have a separate localization file containing key-value pairs for text strings used in the application. The `Ressource<Localization>` resource type allows you to load these files and retrieve localized text based on the current language setting.

`Ressource<Localization>` are cold stored into `.fe.lang` file.

### Format
Feline Engine supports various formats for localization files, including JSON, XML, YAML, and CSV. The choice doesn't change anything because to call a key you will use `ressource.get(key.subkey.sub_subkey)`.

The structure use a key-value pair system, where each key represents a unique identifier for a text string, and the value is the localized text in the target language. The targeted language is specified by the file name.

Feline Engine store all language localization files in individuals `Ressource<Localization>` ressource wich is a compressed dictionary of all key-value pairs for a specific language.

To use the Feline Engine localization system, you typically follow these steps:
1. Initialize the localization system into the EDA system.
2. Set the desired language for the application (it will load the correct `Ressource<Localization>` ressource)
3. Retrieve localized text strings using their unique keys from any part of the application.

### Example
```cpp
#include <FelineEngine/EDA/EDA.h>
#include <FelineEngine/Ressources/Ressource.h>
#include <FelineEngine/Ressources/RessourceType.h>
#include <FelineEngine/Localization/LocalizationSystem.h>

// 1. Initialize the Localization System
EDA::getInstance().initializeSystem<LocalizationSystem>();

// 2. Load the Localization Ressource
Ressource<Localization> localizationRessource("path/to/localization_file.json");
// It can be JSON, XML, YAML, CSV or a compiled .fr.lang
// If the format isn't compiled, the Localization will compiled him.

// 3. Set the desired language
LocalizationSystem::getInstance().setLanguage("fr"); // Set to French

// 4. Retrieve localized text strings
std::string welcomeText = LocalizationSystem::getInstance().getText("level0.welcome_message");
```

> [!NOTE]
> The `LocalizationSystem` need to accept hot-reload to change language. To do it we need to operate in a specific thread.