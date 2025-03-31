# Documentation des Expressions Régulières (Regex) Utilisées

## 1. Extraction des Blocs de Code
**Regex**: `r"```(?:plaintext|bash|sh|txt|)\s*([\s\S]*?)```"`

### Explication:
- ``` : Détecte les délimiteurs de blocs de code
- `(?:plaintext|bash|sh|txt|)` : Types de blocs reconnus (optionnels)
- `\s*` : Espaces éventuels après le type
- `([\s\S]*?)` : Capture tout le contenu du bloc (y compris les sauts de ligne)
  - `\s` : caractères d'espacement
  - `\S` : caractères non-espace
  - `*?` : quantificateur non-greedy

### Exemple:
```markdown
```plaintext
src/
main.rs
```
```
Capture: "src/\nmain.rs"

## 2. Détection des Indentations et Arborescence
**Regex**: `r"(?m)^[ \t]*[├└]─+\s+([^/\n]+)(/?)$"`

### Explication:
- `(?m)` : Mode multi-ligne
- `^[ \t]*` : Espaces ou tabulations en début de ligne
- `[├└]` : Caractères d'arborescence
- `─+` : Un ou plusieurs traits horizontaux
- `\s+` : Espaces après les traits
- `([^/\n]+)` : Capture le nom (tout sauf '/' et saut de ligne)
- `(/?)$` : Détecte si c'est un dossier (/) en fin de ligne

### Exemple:
```
├── src/
│   └── main.rs
```
Capture: "src/" et "main.rs"

## 3. Extraction des Chemins
Trois patterns sont utilisés:

### Pattern 1: `r"[├└]─+\s+([./\w-]+(?:/[./\w-]+)*"`
- Détecte les chemins après les symboles d'arborescence
- Capture les chemins avec `.`, `/`, `\w` (alphanumérique), `-`

### Pattern 2: `r"^([./\w-]+(?:/[./\w-]+)*)"`
- Détecte les chemins en début de ligne

### Pattern 3: `r"^\s+([./\w-]+(?:/[./\w-]+)*)"`
- Détecte les chemins indentés

## 4. Nettoyage des Lignes
- `line.find('#')` : Supprime les commentaires après #
- `trim()` : Supprime les espaces superflus

## Tableau Récapitulatif

| Regex | Objectif | Exemple de Capture |
|-------|----------|---------------------|
| ```(?:...)``` | Blocs de code | "src/\nmain.rs" |
| `^[ \t]*[├└]─+` | Structure arborescente | "├── src/" → "src/" |
| `[./\w-]+(?:/[./\w-]+)*` | Chemins relatifs | "src/main.rs" |

## Bonnes Pratiques
1. Les regex sont pré-compilées pour meilleure performance
2. Gestion des erreurs intégrée (unwrap seulement après vérification)
3. Plusieurs patterns pour couvrir différents formats de README
4. Nettoyage systématique des entrées

Cette documentation explique comment le code analyse et interprète les différentes représentations de structures de fichiers dans les README.md pour reconstruire fidèlement l'arborescence.