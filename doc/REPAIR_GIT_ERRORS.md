# Repair Guide: `snap` / Git corruption (`empty object` + bad branch ref)

Acest ghid documentează fix-ul corect pentru incidentul:

```bash
[snap] Error: Command failed: 'git status --porcelain'
error: object file .git/objects/XX/YYYY... is empty
fatal: loose object XXYYYY... is corrupt
```

și/sau:

```bash
fatal: bad object refs/heads/master
# sau
fatal: bad object refs/heads/main
```

## Ce s-a întâmplat

- A existat un loose object gol (`.git/objects/...` de 0 bytes) sau un **tag gol** (`.git/refs/tags/...` de 0 bytes) care a stricat comenzi Git de bază.
- Simptom: `fatal: bad object refs/tags/vXXX` sau `warning: ignoring broken ref`.
- Ref-ul local al branch-ului activ (`refs/heads/master`, `refs/heads/main`, etc.) fie pointează la un SHA inexistent, fie **lipsește complet**.
- `.git/HEAD` poate conține un SHA brut în loc de `ref: refs/heads/<branch>`.
- `snap restore` / `snap new` folosesc intern comenzi Git care depind de ref-uri valide; când un tag sau branch-ul activ e invalid, operația cade chiar dacă tag-ul dorit există.
- Poți rula întâi `snap doctor` pentru diagnostic read-only.
- Pentru cazurile sigure, poți rula `snap doctor --repair`; creează backup complet `.git.backup.YYYYMMDD-HHMMSS`, cere confirmare și apoi repară automat.

## Reparare automată recomandată

Începe cu diagnosticul:

```bash
snap doctor
```

Dacă raportul arată probleme de tip fișier gol în `.git/objects` / `.git/refs`, branch ref invalid sau `HEAD` brut/detached care poate fi asociat sigur cu un branch, rulează:

```bash
snap doctor --repair
```

Ce face:

- creează backup complet al directorului `.git`;
- șterge doar fișiere Git de 0 bytes din `.git/objects` și `.git/refs`;
- repară branch-ul activ către ultimul snapshot valid când poate determina branch-ul sigur;
- normalizează `.git/HEAD` la `ref: refs/heads/<branch>` când poate determina branch-ul sigur;
- verifică metadata snap din tag-uri (`Snap-Metadata-Ref`);
- pin-uiește blob-urile metadata valide sub `refs/snap-metadata/<hash>`, ca să nu fie șterse de `git gc`;
- dacă metadata lipsește pentru snapshot-ul activ, o regenerează din worktree-ul curent și retag-uiește snapshot-ul activ pe același commit;
- reconstruiește indexul cu `git reset --mixed HEAD`;
- rulează verificare finală.

Ce nu face:

- nu rulează fără confirmare;
- nu ghicește între `main`, `master` sau alt branch dacă nu are un indiciu concret;
- nu șterge fișiere Git non-goale;
- nu inventează metadata pentru snapshot-uri istorice non-active, pentru că empty dirs și atributele hidden/read-only nu pot fi reconstruite mereu din commit;
- nu schimbă formatul snapshot-urilor.

Dacă repair-ul automat spune că nu poate determina branch-ul în siguranță, folosește pașii manuali de mai jos.

## Metadata blob lipsă după `git gc`

Simptom:

```bash
[snap] Error: Snapshot "v91.83" references metadata blob 'd84e...', but snap could not read it.
Run `snap doctor --repair` to repair safe cases.
```

Cauza: snapshot tag-ul conține `Snap-Metadata-Ref: <hash>`, dar hash-ul era doar text în mesajul tag-ului. Dacă blob-ul metadata nu era pin-uit printr-un ref real, `git gc --prune=now` îl putea șterge.

Flux recomandat:

```bash
snap doctor
snap doctor --repair
```

Repair-ul automat:

- detectează metadata lipsă, invalidă sau nepinuită;
- creează backup `.git.backup.YYYYMMDD-HHMMSS`;
- pin-uiește blob-urile metadata care încă există;
- regenerează metadata lipsă numai pentru snapshot-ul activ, folosind starea curentă a worktree-ului.

Pentru snapshot-uri vechi non-active, `snap doctor --repair` raportează problema, dar nu ghicește metadata. Folosește manual un snapshot apropiat sau restore într-un worktree separat doar dacă știi exact ce metadata vrei să păstrezi.

## Reparare corectă (pas cu pas)

Rulează din rădăcina repo-ului.

### Pas 0 — Backup minim

```bash
cp -a .git ".git.backup.$(date +%Y%m%d-%H%M%S)"
```

### Pas 1 — Curăță obiectele corupte goale

```bash
find .git/objects .git/refs -type f -size 0 -delete
```

### Pas 2 — Identifică ultimul tag valid

Găsește cel mai recent snapshot tag funcțional:

```bash
snap list
```

Notează label-ul (ex: `v43.789`). Verifică că tag-ul pointează la un commit valid:

```bash
git show -s --oneline v43.789
```

Dacă comanda de mai sus afișează commit-ul fără eroare, tag-ul e bun.

### Pas 3 — Inspectează HEAD și branch ref

```bash
echo "=== HEAD ==="
cat .git/HEAD

branch=$(git symbolic-ref --short HEAD 2>/dev/null || true)
if [ -z "$branch" ]; then
  branch="master" # fallback pentru incidente vechi; schimbă manual dacă proiectul folosește main
fi
echo "Branch verificat: $branch"

echo "=== branch ref ==="
cat ".git/refs/heads/$branch" 2>/dev/null || echo "(LIPSEȘTE)"

echo "=== packed-refs branch ==="
grep "refs/heads/$branch" .git/packed-refs 2>/dev/null || echo "(nu e în packed-refs)"
```

**Diagnosticul:**

| HEAD conține | refs/heads/<branch> | Situație |
|---|---|---|
| `ref: refs/heads/<branch>` | SHA valid (git cat-file -t returnează `commit`) | ✅ Sănătos |
| `ref: refs/heads/<branch>` | SHA invalid / lipsește | ⚠️ Repară branch ref (Pas 4) |
| SHA brut | orice | ⚠️ Repară branch ref (Pas 4) + HEAD (Pas 5) |

### Pas 4 — Repară branch ref

Extrage commit-ul valid din ultimul tag bun și repară ref-ul:

```bash
good_commit=$(git rev-parse v43.789^{commit})
echo "Commit valid: $good_commit"

# Setează branch-ul real al proiectului: main, master, etc.
branch=$(git symbolic-ref --short HEAD 2>/dev/null || echo master)
echo "Branch reparat: $branch"

# Repară ref-ul branch-ului
git update-ref "refs/heads/$branch" "$good_commit"

# Verificare
git rev-parse --verify "refs/heads/$branch"
```

> **IMPORTANT:** Înlocuiește `v43.789` cu tag-ul tău cel mai recent din `snap list`.

### Pas 5 — Normalizează HEAD

HEAD trebuie să conțină `ref: refs/heads/<branch>`, NU un SHA brut:

```bash
branch=${branch:-master}
printf 'ref: refs/heads/%s\n' "$branch" > .git/HEAD

# Verificare
cat .git/HEAD
# Trebuie să afișeze: ref: refs/heads/<branch>
```

### Pas 6 — Reconstruiește indexul

```bash
rm -f .git/index
git reset --mixed HEAD
```

### Pas 7 — Verificare completă

```bash
# Toate trebuie să ruleze fără eroare
git status --short | head -5
git rev-parse --verify "refs/heads/$branch"
git show -s --oneline HEAD
```

### Pas 8 — Test snap

```bash
snap list
snap new vXX.YY "descriere"
```

## Flowchart rapid

```
git status --porcelain eșuează?
│
├─ "empty object" / "corrupt" → Pas 1 (delete empty objects)
│
├─ cat .git/HEAD
│   ├─ conține SHA brut → Pas 4 + Pas 5
│   └─ conține "ref: refs/heads/<branch>" → ok, verifică branch ref
│
├─ cat .git/refs/heads/<branch>
│   ├─ lipsește (No such file) → Pas 4
│   ├─ SHA invalid (git cat-file -t eșuează) → Pas 4
│   └─ SHA valid → ref-ul e ok
│
└─ După Pas 4+5 → Pas 6 (rebuild index) → Pas 7 (verify) → Pas 8 (test snap)
```

## Ce să NU faci

- **Nu** scrie `main` în `.git/HEAD` dacă branch-ul real este `master`, și invers.
- **Nu** scrie un hash brut arbitrar în `.git/HEAD`.
- **Nu** rula `git reset --mixed HEAD` înainte ca `HEAD` + branch ref să pointeze la commit valid.
- **Nu** interpreta `dangling blob/tag` din `git fsck` ca eroare critică imediată; ele sunt frecvent non-fatale.
- **Nu** încerca `cat .git/refs/heads/<branch> > /tmp/...` dacă fișierul nu există — va da eroare; folosește `2>/dev/null` sau `|| true`.

## Diagnostic rapid util

```bash
git rev-parse --is-inside-work-tree
branch=$(git symbolic-ref --short HEAD 2>/dev/null || echo master)
git show-ref --verify "refs/heads/$branch"
git rev-parse --verify "refs/heads/$branch"
git show -s --oneline HEAD
cat .git/HEAD
cat ".git/refs/heads/$branch" 2>/dev/null || echo "LIPSEȘTE"
git cat-file -t "$(cat ".git/refs/heads/$branch" 2>/dev/null)" 2>/dev/null || echo "INVALID"
```

## Incident recap

### Incident 1 — v41.33 (Feb 2026)

- Snapshot tag `v41.33` era valid (`e67b1e9...`), dar `refs/heads/master` era corupt (`3fdb...` inexistent).
- Fix: `git update-ref refs/heads/master e67b1e9...` + `printf 'ref: refs/heads/master\n' > .git/HEAD`

### Incident 2 — v43.789 (30 Mar 2026)

- `.git/HEAD` conținea SHA brut `108d92aa...` în loc de `ref: refs/heads/master`.
- `.git/refs/heads/master` lipsea complet (fișierul nu exista).
- Obiectul `108d92aa...` era gol/corupt (0 bytes, șters la Pas 1).
- `packed-refs` avea o referință veche `e67b1e9... refs/heads/master` dar nu ajuta.
- Fix: `git update-ref refs/heads/master 3b475dc...` (commit-ul de la tag v43.789) + `printf 'ref: refs/heads/master\n' > .git/HEAD` + rebuild index.

### Incident 3 — v46.637 (7 Apr 2026)

- `snap restore v47.251` eșua cu `fatal: bad object refs/tags/v46.637`.
- Tag-ul `v46.637` era un fișier de 0 bytes pe disc.
- Fix: `find .git/refs -type f -size 0 -delete`. După ștergere, Git a putut procesa corect restul tag-urilor și a permis checkout-ul.

## Prevenție în aplicația `snap` (Rust)

Implementat:

1. Verifică ref-uri critice:
- rulează verificări pentru `HEAD` și branch-ul activ detectat, nu hardcoded `master`.
- dacă eșuează, oprește operația cu mesaj clar și recomandă `snap doctor`.

2. Preflight rapid pentru comenzile normale:
- `snap new`, `restore`, `update`, `edit`, `delete` rulează un preflight sub-secundă în mod normal;
- verifică `.git`, `git status --porcelain`, `HEAD`, branch-ul activ și fișiere goale doar în `.git/refs`;
- nu scanează `.git/objects` și nu validează toate tag-urile, ca să nu încetinească proiectele mari în WSL.

3. Diagnostic complet separat:
- `snap doctor` scanează `.git/objects`, `.git/refs`, `HEAD`, branch-ul activ și snapshot tag-urile;
- poate dura mai mult, dar este comandă explicită de diagnostic.

4. Nu depinde de branch implicit fragil:
- pentru `restore`, rezolvă snapshot-ul la commit (`tag^{commit}`) și folosește `git reset --hard <commit>` pe branch-ul activ;
- nu bloca restore pe `master` dacă proiectul folosește `main` sau alt branch.

5. Reparare controlată:
- `snap doctor` este read-only;
- `snap doctor --repair` creează backup, cere confirmare și repară doar cazurile sigure.

6. Mesaje de eroare orientate pe acțiune:
- include detectarea exactă: `invalid ref`, `empty loose object`, `bad HEAD`.
- arată comenzi exacte de fix (copy-paste ready).
