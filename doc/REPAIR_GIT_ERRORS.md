# Repair Guide: `snap` / Git corruption (`empty object` + `bad object refs/heads/master`)

Acest ghid documentează fix-ul corect pentru incidentul:

```bash
[snap] Error: Command failed: 'git status --porcelain'
error: object file .git/objects/XX/YYYY... is empty
fatal: loose object XXYYYY... is corrupt
```

și/sau:

```bash
fatal: bad object refs/heads/master
```

## Ce s-a întâmplat

- A existat un loose object gol (`.git/objects/...` de 0 bytes) sau un **tag gol** (`.git/refs/tags/...` de 0 bytes) care a stricat comenzi Git de bază.
- Simptom: `fatal: bad object refs/tags/vXXX` sau `warning: ignoring broken ref`.
- Ref-ul local `refs/heads/master` fie pointează la un SHA inexistent, fie **lipsește complet**.
- `.git/HEAD` poate conține un SHA brut în loc de `ref: refs/heads/master`.
- `snap restore` / `snap new` folosesc intern comenzi Git care depind de ref-uri valide; când un tag sau `master` e invalid, operația cade chiar dacă tag-ul dorit există.

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

### Pas 3 — Inspectează HEAD și master ref

```bash
echo "=== HEAD ==="
cat .git/HEAD

echo "=== master ref ==="
cat .git/refs/heads/master 2>/dev/null || echo "(LIPSEȘTE)"

echo "=== packed-refs master ==="
grep "refs/heads/master" .git/packed-refs 2>/dev/null || echo "(nu e în packed-refs)"
```

**Diagnosticul:**

| HEAD conține | refs/heads/master | Situație |
|---|---|---|
| `ref: refs/heads/master` | SHA valid (git cat-file -t returnează `commit`) | ✅ Sănătos |
| `ref: refs/heads/master` | SHA invalid / lipsește | ⚠️ Repară master ref (Pas 4) |
| SHA brut | orice | ⚠️ Repară master ref (Pas 4) + HEAD (Pas 5) |

### Pas 4 — Repară master ref

Extrage commit-ul valid din ultimul tag bun și repară ref-ul:

```bash
good_commit=$(git rev-parse v43.789^{commit})
echo "Commit valid: $good_commit"

# Repară ref-ul master
git update-ref refs/heads/master "$good_commit"

# Verificare
git rev-parse --verify refs/heads/master
```

> **IMPORTANT:** Înlocuiește `v43.789` cu tag-ul tău cel mai recent din `snap list`.

### Pas 5 — Normalizează HEAD

HEAD trebuie să conțină `ref: refs/heads/master`, NU un SHA brut:

```bash
printf 'ref: refs/heads/master\n' > .git/HEAD

# Verificare
cat .git/HEAD
# Trebuie să afișeze: ref: refs/heads/master
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
git rev-parse --verify refs/heads/master
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
│   └─ conține "ref: refs/heads/master" → ok, verifică master ref
│
├─ cat .git/refs/heads/master
│   ├─ lipsește (No such file) → Pas 4
│   ├─ SHA invalid (git cat-file -t eșuează) → Pas 4
│   └─ SHA valid → ref-ul e ok
│
└─ După Pas 4+5 → Pas 6 (rebuild index) → Pas 7 (verify) → Pas 8 (test snap)
```

## Ce să NU faci

- **Nu** scrie `main` în `.git/HEAD` dacă branch-ul real este `master`.
- **Nu** scrie un hash brut arbitrar în `.git/HEAD`.
- **Nu** rula `git reset --mixed HEAD` înainte ca `HEAD` + branch ref să pointeze la commit valid.
- **Nu** interpreta `dangling blob/tag` din `git fsck` ca eroare critică imediată; ele sunt frecvent non-fatale.
- **Nu** încerca `cat .git/refs/heads/master > /tmp/...` dacă fișierul nu există — va da eroare; folosește `2>/dev/null` sau `|| true`.

## Diagnostic rapid util

```bash
git rev-parse --is-inside-work-tree
git show-ref --verify refs/heads/master
git rev-parse --verify refs/heads/master
git show -s --oneline HEAD
cat .git/HEAD
cat .git/refs/heads/master 2>/dev/null || echo "LIPSEȘTE"
git cat-file -t "$(cat .git/refs/heads/master 2>/dev/null)" 2>/dev/null || echo "INVALID"
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

## Sugestii pentru aplicația `snap` (Rust) ca prevenție

Adaugă un preflight de sănătate Git înainte de `new/restore/edit/diff`:

1. Verifică ref-uri critice:
- rulează `git rev-parse --verify refs/heads/master`
- dacă eșuează, oprește operația cu mesaj clar și pași de recovery automatizați.

2. Verifică obiecte goale:
- scan rapid `.git/objects` pentru fișiere de 0 bytes;
- dacă există, oprește fluxul normal și oferă `snap doctor --repair`.

3. Nu depinde de branch implicit fragil:
- pentru `restore`, checkout direct după commit-ul snapshot-ului (`tag^{commit}`), apoi opțional repoziționare branch;
- nu bloca restore pe `master` dacă tag-ul țintă este valid.

4. Adaugă comandă dedicată:
- `snap doctor` (read-only checks),
- `snap doctor --repair` (safe repair flow: delete empty objects, repair invalid branch ref to chosen snapshot commit, reset index).

5. Mesaje de eroare orientate pe acțiune:
- include detectarea exactă: `invalid ref`, `empty loose object`, `bad HEAD`.
- arată comenzi exacte de fix (copy-paste ready).
