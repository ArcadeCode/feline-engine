# Questions majeur de développement
Ici arrive toutes les questions majeur nécessaire à être répondu pour développer le moteur de jeu Féline Engine.

## Global/Architecture
- [ ] C'est quoi un joueur ?
- [ ] C'est quoi une scene (d'un point de vue mémoire) ?
- [ ] C'est quoi `Feline Engine` d'un point de vue code ?
    - Est-ce une librairie ? Statique ? Dynamique ? (Idiome Unreal)
    - Est-ce un framework intégrer à chauque jeu ? (Idiome Unity)
- [ ] Comment le moteur gère-t-il la boucle principale (main loop) ? Qui contrôle le deltaTime, la synchronisation, etc. ?
- [ ] Comment est organisé le cycle de vie du moteur ? (initialisation, update, shutdown)
- [ ] Comment se gèrent les modules ? (audio, render, input...) → chargés dynamiquement ? statiquement liés ?
  - Chaque module proposera un ou plusieurs **services** dans une structure **EDA** (event driven architecture), chaque module peut **s'abonner** à des services d'autres modules.
- [ ] Quelle est la philosophie d’extension du moteur ? (plugin system, hooks, scripting API, etc.)
- [ ] Comment se fait la séparation entre engine et jeu ? (ex: le jeu appelle le moteur ou inversement ?)
- [ ] Comment se gère la compatibilité multi-plateforme (Windows/Linux/macOS) ?
- [ ] Comment sont gérées les build configurations (debug, release, shipping) ?
- [X] Comment versionner le moteur indépendamment des jeux ?
    - Le moteur utilisera un système de version (v.majeur.mineur.patch), sera ajouter à la fin un 'a' pour "alpha", exemple : `v4.52.140a`

## 🧩 Core / ECS / Job System
- [X] A quoi peut servir un job sheduler ?
    - A exécuter des ECS updates de manière paralléliser garantissant une meilleur vitesse d'exécution.
- [ ] C'est quoi une ressource ?
- [ ] Quels allocateurs proposé ?
- [ ] Le scheduler connaît-il les dépendances entre systèmes ECS ? (graph DAG des jobs ?)
- [ ] Comment les composants sont-ils stockés ? (SoA, AoS, chunked memory, archetypes ?)
- [ ] Comment se gère la synchronisation des données ECS entre threads ?
- [ ] Comment les entités sont-elles identifiées ? (ID 64-bit, handles, génération...)
- [ ] Quelle est la politique mémoire globale ? (allocateurs par sous-système, pools, arenas, etc.)
- [ ] Quelles sont les types de ressources (textures, meshes, scripts, etc.) et leur cycle de vie ?
- [ ] Faut-il implémenter un Asset Manager ? Si oui, gère-t-il le hot-reload ?

## 🔊 Audio
- [X] Qui mix l'audio entre FE, miniaudio et SDL3 ?
    - C'est SDL3 qui gère le mixage et mini-audio qui gère l'audio par dessus.
- [X] Comment sont gérés les bus audio, effets, spatialisation ?
    - Miniaudio s'occupera des effets appliqué dynamiquement, mais un système d'asset audio avec effet appliqué statiquement devra être mis en place aussi.
- [ ] Comment synchroniser audio et gameplay (latence, clock system) ?

## 🎨 Rendering
- [X] Le moteur doit-il fournir une abstraction graphique unifiée (backend OpenGL/Vulkan/DirectX) ?
    - Non, le moteur se concentrera que sur un seul backend, de préférence portable comme Vulkan. Néanmoins le moteur proposera une couche d'abstraction graphique pouvant être éditer pour l'adapter à un nouveau backend au besoin sans avoir à réécrire un grand pourcentage du moteur par nous même.
- [ ] Faut-il séparer le renderer (haut niveau) du graphics backend (bas niveau) ?
- [ ] Comment sont gérés les shaders ? (compilation à runtime ? précompilation ? formats personnalisés ?)
- [ ] Quelle est la représentation d’une scène graphique (scene graph, BVH, visibility system...) ?
- [ ] Faut-il supporter le multi-pass rendering ou des pipelines custom ?
- [ ] Comment gérer les materials (unified shading model ? PBR ?)
- [ ] Est-ce que le moteur fournit un UI renderer intégré ?

## 🎮 Gameplay / Input / Scene
- [ ] Comment représenter une scene ? (graph de nodes ? liste d’entités ? world ECS ?)
- [ ] Qu’est-ce qu’un joueur d’un point de vue logique ? (input device ? entité ECS ? contrôleur ?)
- [ ] Comment gérer plusieurs scenes simultanément (menu + monde en arrière-plan, etc.) ?
- [ ] Comment se fait la transition de scène ? (streaming, destruction/reload, etc.)
- [ ] Quelle est la API scripting (C, Lua, Python, Rust côté jeu...) ?

## 💾 Ressources / Assets / Data
- [ ] Comment sont chargés les assets ? (runtime vs précompilés en “packages”)
- [ ] Faut-il créer un format de ressource propre au moteur (ex: .fpack, .fmat, etc.) ?
- [ ] Faut-il un Resource Compiler pour convertir les formats externes ?
- [ ] Gérer le caching (in-memory, disque, GPU...)
- [ ] Supporter le hot reload des assets et scripts ?

## 🧠 Outils / Pipeline
- [ ] Faut-il un éditeur intégré ou des outils externes (type CLI, YAML, JSON...) ?
- [ ] Comment les développeurs interagissent avec le moteur ? (API C, GUI, scripting)
- [ ] Faut-il un système de logs unifié et configurable ?
- [ ] Faut-il un profiling system intégré (perf, mémoire, allocations, draw calls...) ?
- [ ] Comment gérer le debugging en multi-thread ?

## 📦 Build / Packaging / Distribution
- [ ] Comment empaqueter les librairies du moteur ?
- [ ] Le jeu final doit-il contenir le moteur compilé statiquement ou dynamique (DLL/.so) ?
- [ ] Comment organiser un runtime folder clair (bin/, data/, shaders/, assets/...) ?
- [ ] Faut-il proposer un launcher pour gérer les versions et DLC ?
- [ ] Comment gérer la distribution Steam/itch.io (scripts, manifests, updates) ?