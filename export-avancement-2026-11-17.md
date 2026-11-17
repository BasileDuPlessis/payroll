---
id: export-avancement-hrz-2026-11-17
type: report
title: "Export d'avancement Horizon — MORPHEUS (Résiliation) et NEO (Vente) — 17/11/2026"
status: draft
date: 2026-11-17
perimetre: [resiliation, vente, i-know-kung-fu]
source: "/edfdco-cockpit — relevé du dépôt sur origin/main 16534037"
---

# Export d'avancement Horizon — 17/11/2026

Relevé fait dans le dépôt (`origin/main` au commit `16534037`), sans appel aux SI. Chaque chiffre renvoie à sa source.

## Synthèse

| Indicateur | MORPHEUS — Résiliation | NEO — Vente (produit 1) |
|---|---|---|
| Début de l'instruction | 01/09/2026 | 12/10/2026 |
| Instruction | **4/4 par décision d'équipe**, documents encore en `draft` (1/4 +3 brut) | **4/4** (statuts `active` / `complete`) |
| Epics | 15 : 3 en cours, 12 non commencés | 8 : 1 en cours, 7 non commencés |
| US | 93, dont **7 terminées** (5 done, 2 review) | 44 au sprint-status (46 au backlog), dont **7 terminées** |
| Avancement dev | E1 et E2 visés au 30/11 : 7/20 stories | 16 % des stories |
| Questions ouvertes | 125 ouvertes sur 202 | 12 ouvertes sur 21 (PRD § 11) |
| Flux d'intégration | 61 flux, 22 MuleApps | 26 flux, 0 MuleApp |
| Code produit en propre | SF + PlaaSMA + Camunda + SAP | SF (en grande partie repris de PVG), pas encore d'intégration |

## 1. Complétude de l'instruction

Règles appliquées : celles de l'intent `dashboard`. Un asset est **fait** s'il porte `status: active|final`, **en cours** s'il existe en `draft`, **absent** sinon.

### Résiliation (MORPHEUS)

| Asset | Fichier | État brut |
|---|---|---|
| Brief | `briefs/brief-resiliation/brief.md` | draft → en cours |
| PRD | 3 slices `01-preparees`, `02-non-preparees`, `03-transverses` (`prd-fonctionnel.md`) | toutes en draft → en cours |
| Architecture | `architecture.md` (`stepsCompleted: []`) + 10 fichiers de 01 à 10 | en cours |
| Backlog | `epics/epics-resiliation/epics.md` : 15 epics, 93 US | fait |

- L'instruction est **close par décision d'équipe** : `config.json` porte `instructionStatut: "terminee"`, d'où le 4/4 affiché sur le dashboard. Les documents BMAD n'ont pas été passés en `active`.
- Questions ouvertes d'architecture (`questions-ouvertes.yaml`) : 202 au total.
  - 77 fermées : 71 résolues, 6 caduques.
  - 125 encore ouvertes : 75 ouvertes, 43 instruites, 7 arbitrées.
- Registre des flux (`exchanges-registry.yaml`) : 61 flux.

### Vente (NEO)

| Asset | Fichier | État |
|---|---|---|
| Brief | `brief-Vente-2026-10-13/brief.md` | active → fait |
| PRD | `prd-Vente-2026-10-20/prd.md` (sans slices) | active → fait |
| Architecture | `architecture.md` (`status: complete`, étapes 1 à 8) + 7 fichiers de 01 à 07 | fait |
| Backlog | `epics-pvg-produit1.md` : 8 epics ; `backlog-produit1.md` : 46 stories | fait |

- Pas de registre de questions ouvertes. La § 11 du PRD en liste 12 ouvertes et 9 résolues.
- Registre des flux : 26 flux.
- Écart : le backlog compte 46 stories, le sprint-status 44.

## 2. Backlog et avancement

### Résiliation — suivi par epic

Source : `python scopes/hrz/suivi-delivery/suivi_core.py epics-status resiliation`. Une story en `review` compte comme terminée.

| Epic | État | Stories | Détail |
|---|---|---|---|
| E0 Socle transverse et filets de sécurité | En cours | 12 | 10 backlog, 2 en cours |
| **E1 Résiliation élec préparée CC — socle Linky** | En cours | 14 | 4 backlog, 3 en cours, 2 review, 5 done |
| **E2 Suivi async et clôture** | En cours | 6 | 3 backlog, 3 en cours |
| E3 à E14 (gaz, bi-énergie, CHF, successeur, services CI, cascade, réengagement, RIF/MHS, annulation) | Non commencés | 61 | tous en backlog |

- Objectif fin novembre (E1 et E2) : **7/20 stories terminées**. Le statut des stories 1.8, 2.1 et 2.2 est porté par leurs branches `feat/us-*`.
- Statuts de branche écartés : 11, issus de l'ancien epic 13 retiré, présents sur `feat/us-1-7`. Ils ne sont pas comptés.
- Backlog d'équipe (`backlog-morpheus.md`, mis à jour le 16/11) : **85 actions**, dont 62 à faire, 5 en cours, 16 faites, 1 bloquée et 1 en attente. Prochaine échéance : 30/11.

### Vente — pourcentage de stories

Source : `suivi_core.py dev-status vente`.

- E1 : en cours, 7 stories done sur 9 (1.1 à 1.6 et 1.9). Les stories 1.7 et 1.8 sont en `ready-for-dev`.
- E2 à E8 : 35 stories, toutes en backlog.
- **16 %**, soit 7/44. Les 7 sont toutes sur main, aucune n'est portée par une branche.
- ⚠ Backlog d'équipe (`backlog-neo.md`) **non tenu depuis le 09/10** : 7 actions (6 à faire, 1 en cours), échéance du 13/10 dépassée.

### Hors périmètre demandé — I Know Kung Fu (NEO, expérimental)

8 % d'avancement publié sur le dashboard.

## 3. Assets de capitalisation

Source : `capitalisation-grille.md` (édition du 16/11), calculée par `capitalisation-deck.js`. **La grille est commune aux deux équipes** : elle ne distingue pas MORPHEUS et NEO.

- **77 points sur 104, soit 74 %** sur 13 domaines.
- Domaines : 5 couverts, 6 partiels, 1 amorcé, 1 non couvert (éditique).
- Scores : IM 8 · UX 3 · AR 8 · CA 7 · PL 8 · CM 5 · FA 8 · ED 0 · TQ 6 · SE 4 · EX 7 · DB 5 · MP 8.
- Par famille : Métier et conception 11/16 · Architecture et SI 36/48 · Transverse 30/40.
- Évolution :
  - 70 % le 05/11 sur 14 domaines ;
  - 73 % le 09/11, après le retrait du domaine Digital ;
  - 74 % le 16/11, soit +1 point à périmètre constant (PL/Produire passe à ●).
- **148 assets inventoriés dans la grille** :
  - par critère : Produire 67 · Connaître 34 · Contrôler 29 · Accéder 18 ;
  - par type : 78 skills · 22 savoirs · 14 scripts · 11 serveurs MCP · 10 customisations BMAD · 9 hooks · 4 divers.
- Plugin plaasma 0.4.0 : 19 skills, 22 règles, 31 références, entièrement intégrés.
- ⚠ **29 nouveaux assets** apparus depuis le 03/11 (`generate_petit_comite.py --inventaire`) ne sont pas encore reportés dans la grille : guide-parcours, anomalie, warmup-mcp, rapport-consolide, etc. La note actuelle est donc **sous-estimée**, à revoir lors de la prochaine mise à jour hebdomadaire.

## 4. Quantité de code produit

Mesuré dans le dépôt : `ls`, `wc -l`, `git log --numstat` filtré sur les membres de `presences-*.md`. Déploiement non vérifié, sauf pour Camunda.

| SI | MORPHEUS — Résiliation | NEO — Vente |
|---|---|---|
| **Salesforce** | 11 classes Apex `RES_*` + 10 classes de test (8 634 lignes) · 38 LWC `res*` (14 001 lignes, 36 tests Jest) · 2 objets | 29 classes `Sales*` + 3 de test (11 043 lignes) · 20 LWC `sales*` (11 756 lignes) · 2 objets (83 champs) + 3 CMDT ⚠ |
| **PlaaSMA (MuleSoft)** | 13 MuleApps touchées (2 exp · 5 prc · 6 sys) + route Flex `flex-morpheus` · ~18 500 lignes ajoutées | 47 lignes (`canopee-conso-exp`) · `flex-neo` et `mocks-neo` vides |
| **Camunda (BPMN)** | 6 BPMN dans le dépôt, 11 tâches connecteur HTTP, 1 130 lignes de tests · `mcp-resiliation-preparee` **déployé en v28** sur devm1 | 0 BPMN exécutable · 7 brouillons de conception |
| **Facthori (SAP)** | 3 services OData pilotes (`zwr_ping`, `zwr_orch1`, `zwr_mnd130`) · moteur SEGW headless : 61 fichiers ABAP (13 176 lignes) + 23 scripts JS (7 213 lignes) | 0 (i-know-kung-fu : 1 service OData `zwr_kgf_fact`) |
| **Commits hors merges** | `projects/resiliation` 1 029 · facthori 234 · plaasma 211 · camunda 37 · sf-dev 15 | `projects/vente` 72 · sf-dev 6 (+ 44 sur i-know-kung-fu) |

**Réserves de mesure**

- ⚠ Le Salesforce de la vente provient presque entièrement du **démonstrateur PVG**, versionné en bloc le 10/11 (commit `485e7b45`, repris de devneo). Ce code existait déjà dans l'org avant d'entrer dans le dépôt. La production propre de NEO se limite aux commits `feat(vente)` : stories 1.1 à 1.9, quelques champs et le bouton de création de processus.
- 4 sous-modules PlaaSMA sont vides sur ce poste : `canopee-tdc-sync-prc`, `opus-tdc-sys`, `tdc-demande-tiers-prc`, `tdc-sync-role-prc`. Leurs flows et MUnit n'ont pas pu être comptés, seul leur historique git l'a été.
- `tdc-demande-tiers-prc` et `tdc-sync-role-prc` ne figurent pas dans les apps de MORPHEUS déclarées dans `teams.json`. Leur rattachement à la résiliation est incertain : white-rabbit possible.
- Arnaud Verhille figure dans les deux équipes : ses commits ne sont attribués à aucune. Les LWC `res*` signés par un NNI non résolu sont attribués à MORPHEUS d'après leur préfixe.
- Non comptés : le code legacy extrait pour la rétrodocumentation (batchs Facthori, ~44 000 lignes ABAP), l'héritage RAP DeLorean et le miroir `sf-main`.

## Points d'attention

1. **NEO** : `backlog-neo.md` n'est plus tenu depuis le 09/10, et aucune intégration (MuleApp, BPMN) n'a démarré alors que le PRD décrit 26 flux.
2. **MORPHEUS** : à 13 jours de l'échéance, 7 stories sur 20 sont terminées pour l'objectif E1+E2. 125 questions d'architecture restent ouvertes. Les documents d'instruction sont toujours en `draft`, malgré la clôture décidée par l'équipe.
3. **Capitalisation** : la grille a pris du retard sur l'inventaire (29 assets non intégrés).
