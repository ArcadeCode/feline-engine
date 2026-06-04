# 🔧 **Feline Engine — Polymorphic Stencil Compute Engine (PSCE) Design Note**

> [WARNING] THIS IS NOT PLANNED
> This PSCE is not planned before long time maybe will never be implemented.

> Is Polymorphic cause it can use CPU on demand to compute, but was not design for this, this is a fallback.
>
> Project name : Panthera (Panthera)

## 1. Objectif

Feline Engine prévoit, à moyen terme, l’intégration d’un **moteur de calcul massivement parallèle sur grille 2D**, capable d’exécuter des règles locales sur des tuiles (tiles) via **CPU ou GPU** de façon interchangeable.

Ce moteur est appelé :

> **PSCE — Polymorphic Stencil Compute Engine**

Son rôle est de fournir une abstraction unifiée pour :

* diffusion thermique
* gaz
* pollution
* réactions
* automates cellulaires
* tout champ physique discret

---

## 2. Modèle mathématique

Le PSCE repose sur le modèle suivant :

[
out(x,y) = f\big( in(x,y), neighbors(x,y) \big)
]

Chaque cellule est calculée **indépendamment** selon une règle locale.

Ce modèle correspond aux solveurs scientifiques de type :

* diffusion
* advection
* Laplacien
* phase-field
* automates cellulaires

---

## 3. API conceptuelle

```cpp
fe::compute_tiles_rule<TileGrid, Rule>(Backend backend);
```

* `TileGrid` : grille 2D des tuiles
* `Rule` : fonction pure locale (cellule + voisins)
* `Backend` : `CPU` ou `GPU`

Le moteur choisit dynamiquement :

* l’implémentation CPU multithread
* ou un pipeline GPU (compute shader)

---

## 4. Exécution CPU vs GPU

| CPU backend        | GPU backend           |
| ------------------ | --------------------- |
| Boucles parallèles | Compute shaders       |
| C++                | SPIR-V / HLSL / Slang |
| Cache CPU          | VRAM                  |
| Thread pool        | Dispatch massif       |

Les deux exécutent **exactement la même règle**.

---

## 5. Compilation GPU

Les règles GPU sont :

* validées pour compatibilité GPU
* transpilées vers shader
* compilées à la volée
* stockées dans un cache de pipelines

La compilation est faite **une seule fois par règle**.

---

## 6. Découplage architectural

Le PSCE est conçu comme une **librairie indépendante** de Feline Engine :

```
psce/
  cpu_backend
  gpu_backend
  rule_compiler
  runtime
```

Feline Engine l’utilise via son interface EDA :

```
PSCE Tick → Event → Gameplay Systems
```

Exemples :

* heat updated → overheat event
* gas pressure → explosion event

### PSCE EDA events
Les events proposer par PSCE sont :
- `stencil_compute_done` : émis à la fin d’un tick de calcul, avec les résultats
- `stencil_compute_error` : émis en cas d’erreur de calcul ou de compilation.
- `stencil_compute_progress` : émis périodiquement pour indiquer la progression du calcul, utile pour les tâches longues sur GPU.
- `stencil_compute_resource_warning` : émis lorsque les ressources GPU sont insuffisantes pour exécuter la règle demandée, permettant à Feline Engine de basculer sur CPU ou d’avertir le joueur.

PSCE s'abonne aux événnements :
- `gameplay_tick` : pour déclencher le calcul à chaque tick de jeu.
- `rule_update` : pour mettre à jour les règles de calcul en fonction des changements dans le gameplay ou les paramètres du jeu.
- `resource_change` : pour ajuster les ressources allouées au PSCE en fonction de la charge du jeu ou des préférences du joueur.

---

## 7. Intégration dans Feline Engine

Le PSCE est **optionnel**.

Feline Engine **v1.0.0** :

* ne dépend pas du PSCE
* fonctionne avec un moteur classique CPU

Le PSCE est prévu comme :

> **un module avancé pour versions futures**

---

## 8. Justification du découplage

Le PSCE est :

* extrêmement complexe
* low-level GPU heavy
* comparable à un moteur scientifique

Le garder indépendant permet :

* stabilité de Feline Engine
* itération progressive
* open-source séparé
* réutilisation hors jeu

---

## 9. Avantage stratégique

Avec le PSCE, Feline Engine pourra à terme supporter la simulation de phénomènes physiques complexes tels que :

* atmosphère
* chaleur
* pollution
* fluides
* incendies
* écologie
* réactions chimiques

à des échelles **impossibles pour les moteurs CPU-only**.

---

## 10. Vision

Feline Engine reste un moteur de jeu.

PSCE devient :

> un moteur de simulation massivement parallèle pour mondes discrets.

Les deux communiquent par événements, jamais par dépendance directe.