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
- [X] Comment se gèrent les modules ? (audio, render, input...) → chargés dynamiquement ? statiquement liés ?
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
- [ ] Utiliseons nous JzAzBz color space pour le rendu ? (pour une meilleure gestion de la luminosité et du contraste) ou restons nous sur du RGB classique ?

## 🎮 Gameplay / Input / Scene
- [ ] Comment représenter une scene ? (graph de nodes ? liste d’entités ? world ECS ?)
- [ ] Qu’est-ce qu’un joueur d’un point de vue logique ? (input device ? entité ECS ? contrôleur ?)
- [ ] Comment gérer plusieurs scenes simultanément (menu + monde en arrière-plan, etc.) ?
- [ ] Comment se fait la transition de scène ? (streaming, destruction/reload, etc.)
- [X] Quelle est la API scripting (C, Lua, Python, Rust côté jeu...) ?
    - Les jeux seront coder en C++ principalement, mais nous pourrons utiliser du Lua pour faire du scripting, par scripting on entandra manipuler les ECS, créers de nouveaux, éditer les anciens etc... En revanche, si un utilisateur voudrais "hacker" un composer de l'engine, il lui faudrais faire de la POO avec C++ (ou C pour les composants bas niveau).
    - Pour l'instant le scripting Lua est mis à part, il sera mis en place concrètement plus tard, cela sera mise en place en utilisant un compilateur qui viendra créer une librairie statique intégrer au jeu, typiquement nous aurons un jeu avec un system intialisé en lua `system.lua`, le compilateur va lire ce fichier et renvoyer un fichier `system.scrt.cpp`, ce ficher sera ensuite lié statiquement au programme.

## 💾 Ressources / Assets / Data
- [ ] Comment sont chargés les assets ? (runtime vs précompilés en “packages”)
- [ ] Faut-il créer un format de ressource propre au moteur (ex: .fpack, .fmat, etc.) ?
- [ ] Faut-il un Resource Compiler pour convertir les formats externes ?
- [ ] Gérer le caching (in-memory, disque, GPU...)
- [ ] Supporter le hot reload des assets et scripts ?

## 🧠 Outils / Pipeline
- [X] Faut-il un éditeur intégré ou des outils externes (type CLI, YAML, JSON...) ?
    - Pour la v1.0, il n'y aura pas d'éditeur, uniquement des librairies.
    - Il existera un outil CLI pour le build d'un jeu en un package prêt à la publication, cette outil viendra emballer les assets entre eux de manière optimisé, compilera le code en un executable en proposant dans le package les DLLs du Feline engine.
    >[!WARNING] Proposer de compiler en un exécutable tous le code du développeur utilisateur implique de linker statiquement ses librairies dans le programme, il pourrait être intéressant d'avoir une commande `--dlls` pour proposer aux utilisateurs d'installer des librairies dynamiques.
    - Par la suite existera un éditeur complet permettant beaucoup de tâches comme manipuler les scènes et les assets.
- [X] Comment les développeurs interagissent avec le moteur ? (API C, GUI, scripting)
    - Les développeurs utilise les headers de Féline engine, ainsi ils peuvent communiquer avec l'engine en utilisant des interfaces POO C++.
    - Les déveoppeurs auront à leur disposition un outil CLI pour compiler un programme pour l'engine.
    - FUTURE: Il y aura une interface GUI dans le future pour l'édition du jeu en temps réel.
- [ ] Faut-il un système de logs unifié et configurable ?
- [ ] Faut-il un profiling system intégré (perf, mémoire, allocations, draw calls...) ?
- [ ] Comment gérer le debugging en multi-thread ?

## 📦 Build / Packaging / Distribution
- [ ] Comment empaqueter les librairies du moteur ?
- [ ] Le jeu final doit-il contenir le moteur compilé statiquement ou dynamique (DLL/.so) ?
- [ ] Comment organiser un runtime folder clair (bin/, data/, shaders/, assets/...) ?
- [ ] Faut-il proposer un launcher pour gérer les versions et DLC ?
- [ ] Comment gérer la distribution Steam/itch.io (scripts, manifests, updates) ?