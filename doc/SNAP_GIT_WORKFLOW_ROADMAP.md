# Snap Git Workflow Roadmap

Acest document descrie directia propusa pentru evolutia aplicatiei `snap` dintr-un
manager de snapshoturi bazat pe Git intr-un layer prietenos, sigur si profesionist
peste Git si GitHub.

Scopul nu este sa rescriem Git. Scopul este ca `snap` sa orchestreze comenzi Git
si GitHub CLI in workflow-uri clare, cu verificari, mesaje bune, confirmari serioase
si comportament predictibil.

## 1. Viziune

Astazi, `snap` este un tool pentru snapshoturi:

```bash
snap init
snap new v1 "initial version"
snap list
snap restore v1
snap update
snap doctor
```

Intern, `snap` foloseste deja Git pentru commituri, taguri, restore si metadata.
Directia propusa este sa extindem aceasta idee: `snap` poate deveni o interfata
mai umana peste workflow-uri Git obisnuite.

Exemplu de diferenta:

```bash
git status --porcelain
git add -A
git commit -m "fix release scripts"
git pull --rebase
git push origin HEAD
git push origin refs/tags/*:refs/tags/*
git push origin refs/snap-metadata/*:refs/snap-metadata/*
```

Ar putea deveni:

```bash
snap save "fix release scripts"
snap sync
```

`snap` ar ramane un wrapper peste Git, dar unul care intelege modelul propriu de
snapshoturi, metadata refs, branchuri si remote sync.

## 2. Ce inseamna "inlocuitor pentru Git"

In contextul acestui roadmap, "inlocuitor pentru Git" nu inseamna ca `snap`
implementeaza propriul sistem de versionare. Git ramane motorul de stocare.

Inseamna ca pentru majoritatea operatiilor zilnice utilizatorul nu mai trebuie sa
tina minte secvente lungi de comenzi Git. In loc sa ruleze manual `git add`,
`git commit`, `git pull`, `git push`, `git tag`, `git switch` si `gh repo create`,
utilizatorul poate rula comenzi `snap` mai clare.

Modelul corect:

```text
snap = friendly Git workflow layer + snapshot manager + remote sync helper
```

Modelul gresit:

```text
snap = script batch care ascunde Git fara verificari si fara siguranta
```

## 3. Probleme in modelul gitc actual

Scripturile `gitc` sunt utile ca prototip. Ele arata bine intentia: o comanda
scurta care face mai multe lucruri. Problema este ca sunt fragile si periculoase
pentru folosire generala.

### 3.1 `update-repo`

Modelul actual:

```bat
git pull
git add .
git commit -m "%commitMessage%"
git push
```

Probleme:

- `git pull` ruleaza inainte sa fie clar daca working tree-ul este curat.
- `git add .` adauga tot ce nu este ignorat, inclusiv fisiere neintentionate.
- daca `pull` esueaza, scriptul nu opreste clar workflow-ul.
- daca `commit` esueaza pentru ca nu exista schimbari, scriptul continua spre push.
- nu verifica starea Git: merge in progres, rebase in progres, detached HEAD, branch invalid.
- nu stie nimic despre refs speciale `snap`, cum ar fi `refs/snap-metadata/*`.

### 3.2 `setup-repo`

Modelul actual initializeaza Git, creeaza repo pe GitHub si face push. Ideea este
buna, dar implementarea trebuie facuta mai sigur:

- nu trebuie sa existe token hardcodat;
- trebuie validat daca directorul este deja repo Git;
- trebuie validat daca remote `origin` exista deja;
- trebuie validat daca repo-ul GitHub exista deja;
- trebuie explicat ce se va crea inainte de executie;
- trebuie oprit clar la primul pas esuat.

### 3.3 `delete-repo`

Stergerea unui repo remote este foarte riscanta. O comanda profesionista trebuie
sa aiba confirmare puternica:

- afiseaza repo-ul exact;
- afiseaza faptul ca sterge remote-ul GitHub, nu fisiere locale;
- cere tastarea exacta a numelui `owner/repo`;
- refuza sa continue daca confirmarea nu se potriveste;
- nu poate folosi token hardcodat.

### 3.4 `make-public` si `make-private`

Schimbarea vizibilitatii este utila, dar trebuie tratata ca operatie sensibila.
`make-public` este mai riscanta decat `make-private`, pentru ca poate expune cod,
assets, istoric si secrete commitate accidental.

Pentru `make-public`, `snap` trebuie sa:

- afiseze repo-ul vizat;
- recomande `snap doctor` si eventual o verificare de secrete in viitor;
- ceara confirmare explicita;
- explice ca operatia afecteaza GitHub, nu doar repo-ul local.

### 3.5 Token hardcodat

Un token hardcodat intr-un script este un risc critic. Daca tokenul este real si a
fost commitat, trimis intr-un chat sau distribuit cuiva, trebuie considerat compromis.

Regula pentru `snap`:

```text
Niciodata token hardcodat.
Niciodata token salvat in plaintext.
Niciodata token afisat in output.
```

## 4. Principii pentru implementarea profesionista

### 4.1 Git ramane sursa adevarului

`snap` nu trebuie sa inventeze un model paralel de istoric. Git ramane sursa pentru:

- commits;
- branchuri;
- taguri;
- remote-uri;
- object database;
- refs speciale.

`snap` adauga UX, siguranta si workflow-uri.

### 4.2 Comenzile mari sunt orchestrari

O comanda `snap` poate rula mai multe comenzi Git in spate.

Exemplu:

```bash
snap sync
```

Poate executa conceptual:

```bash
git status --porcelain
git pull --rebase
git push origin HEAD
git push origin refs/tags/*:refs/tags/*
git push origin refs/snap-metadata/*:refs/snap-metadata/*
```

Dar numai dupa verificari:

- repo Git valid;
- HEAD atasat la branch;
- nu exista rebase/merge in progres;
- remote `origin` exista;
- branchul curent este cunoscut;
- working tree-ul este intr-o stare acceptata.

### 4.3 Output-ul trebuie sa explice

Comenzile `snap` trebuie sa spuna ce fac, de ce se opresc si ce comanda poate rula
utilizatorul mai departe.

Exemplu:

```text
[snap] Cannot sync because your working tree has uncommitted changes.
[snap] Save them first:
  snap save "describe your changes"

[snap] Or inspect them:
  snap status
```

### 4.4 Operatiile destructive cer confirmare stricta

Pentru delete, make-public, branch delete si overwrite operations, confirmarea
trebuie sa fie mai puternica decat `[y/N]`.

Exemplu:

```text
[snap] This will delete GitHub repository glooring/my-app.
[snap] Local files will not be deleted, but the remote repository will be removed.
Type the repository name to confirm: glooring/my-app
```

## 5. Comenzi propuse

Aceasta sectiune descrie comenzile viitoare. Nu toate trebuie implementate deodata.
Ele formeaza directia produsului.

## 5.1 Daily Git workflow

### `snap status`

Scop: afiseaza o stare prietenoasa a repo-ului.

Comenzi Git posibile in spate:

```bash
git status --short --branch
git remote -v
git rev-parse --abbrev-ref HEAD
git rev-parse --verify HEAD
```

Output propus:

```text
[snap] Project status

  Branch: main
  Remote: origin -> git@github.com:glooring/my-app.git
  Ahead/behind: up to date
  Working tree: 3 modified, 1 untracked
  Snapshots: latest v126, active v126

Next:
  snap save "message"  Save current changes
  snap sync            Pull/push branch, snapshots, and metadata
```

### `snap save "message"`

Scop: commit normal, prietenos, fara snapshot tag.

Comenzi Git posibile in spate:

```bash
git status --porcelain
git add -A
git commit -m "<message>"
```

Reguli:

- daca message lipseste, se cere interactiv;
- daca nu exista schimbari, se opreste elegant;
- daca rebase/merge este in progres, refuza;
- ruleaza fast health preflight;
- nu creeaza tag snapshot;
- nu modifica metadata Snap.

De ce este separat de `snap new`:

- `snap new` creeaza snapshot versionat, cu tag si metadata;
- `snap save` creeaza commit Git obisnuit.

### `snap update-repo "message"`

Alias friendly pentru:

```bash
snap save "message"
snap sync
```

Acest alias pastreaza ideea din `gitc update-repo`, dar o face mai sigura.

Reguli:

- daca `snap save` nu creeaza commit pentru ca nu exista schimbari, `sync` poate rula totusi daca utilizatorul confirma sau daca exista commits locale neimpinse;
- daca `pull --rebase` produce conflict, se opreste si explica pasii urmatori;
- nu ascunde erorile Git.

### `snap history`

Scop: afiseaza commituri recente intr-o forma mai clara decat `git log`.

Comanda Git posibila:

```bash
git log --oneline --decorate --graph -n 20
```

Poate include si snapshot tags:

```text
* a1b2c3d Snapshot: v126
* b4c5d6e fix release scripts
* c7d8e9f Snapshot: v125
```

## 5.2 Pull, push si sync

### `snap push`

Scop: impinge branchul curent, snapshot tags si metadata Snap.

Comenzi Git conceptuale:

```bash
git push origin HEAD
git push origin refs/tags/*:refs/tags/*
git push origin refs/snap-metadata/*:refs/snap-metadata/*
```

De ce nu este suficient `git push`:

- Git impinge branchul curent, dar nu neaparat toate tagurile;
- metadata Snap poate fi stocata in `refs/snap-metadata/*`;
- fara metadata refs, unele snapshoturi pot pierde informatii despre empty dirs, hidden files sau readonly paths dupa clone/pull.

Reguli:

- verifica remote;
- verifica branch curent;
- verifica daca repo-ul este sanatos;
- impinge metadata refs dupa branch si tags;
- daca push-ul de metadata esueaza, comanda trebuie sa raporteze clar ca sync-ul Snap este incomplet.

### `snap pull`

Scop: aduce branchuri, taguri si metadata Snap.

Comenzi Git conceptuale:

```bash
git fetch origin
git fetch origin "refs/tags/*:refs/tags/*"
git fetch origin "refs/snap-metadata/*:refs/snap-metadata/*"
git pull --rebase
```

Reguli:

- daca working tree-ul are schimbari locale, refuza sau cere `snap save`;
- daca exista rebase/merge in progres, refuza;
- daca metadata refs nu exista pe remote, nu este eroare fatala pentru repo-urile vechi;
- dupa pull, poate rula o verificare rapida a snapshoturilor.

### `snap sync`

Scop: workflow complet pentru sincronizare.

Flux propus:

```text
1. verifica repo Git
2. verifica working tree
3. verifica remote origin
4. fetch metadata refs
5. pull --rebase
6. push branch curent
7. push snapshot tags
8. push snap metadata refs
9. afiseaza sumar final
```

Output propus:

```text
[snap] Syncing project with origin
  OK Repository health preflight passed
  OK Pulled latest branch changes
  OK Pushed branch main
  OK Pushed 126 snapshot tag(s)
  OK Pushed 18 metadata ref(s)

[snap] Sync complete.
```

## 5.3 Remote si GitHub workflow

### `snap remote status`

Scop: afiseaza remote-ul curent si starea autentificarii.

Comenzi posibile:

```bash
git remote -v
gh auth status
gh repo view --json nameWithOwner,visibility,url
```

Output propus:

```text
[snap] Remote status

  origin: git@github.com:glooring/my-app.git
  GitHub: authenticated as glooring
  Repository: glooring/my-app
  Visibility: private
```

### `snap remote create owner/repo --private`

Scop: creeaza repo GitHub si conecteaza repo-ul local.

Comenzi conceptuale:

```bash
gh auth status
git rev-parse --is-inside-work-tree
gh repo create owner/repo --private --source=. --remote=origin
git push -u origin HEAD
git push origin refs/tags/*:refs/tags/*
git push origin refs/snap-metadata/*:refs/snap-metadata/*
```

Reguli:

- daca nu exista repo Git local, sugereaza `snap init`;
- daca remote `origin` exista, refuza sau cere `--replace-origin`;
- daca repo-ul GitHub exista deja, sugereaza `snap remote set-url`;
- default recomandat: `--private`;
- `--public` trebuie sa fie explicit.

### `snap setup-repo owner/repo --private`

Alias friendly pentru:

```bash
snap remote create owner/repo --private
```

Acest alias pastreaza ideea din `gitc setup-repo`, dar fara token hardcodat si cu
verificari reale.

### `snap remote set-url <url>`

Scop: seteaza remote-ul `origin`.

Comenzi conceptuale:

```bash
git remote add origin <url>
```

sau:

```bash
git remote set-url origin <url>
```

Reguli:

- daca `origin` nu exista, il adauga;
- daca exista, cere confirmare sau necesita `--replace`;
- afiseaza remote-ul final.

### `snap remote visibility public owner/repo`

Scop: face repo-ul public.

Comanda GitHub CLI:

```bash
gh repo edit owner/repo --visibility public
```

Reguli:

- cere confirmare stricta;
- afiseaza avertisment despre expunerea istoricului;
- poate sugera un viitor `snap secrets scan`;
- nu ruleaza daca `gh auth status` esueaza.

### `snap make-public owner/repo`

Alias friendly pentru:

```bash
snap remote visibility public owner/repo
```

Este acceptabil ca top-level alias, pentru ca este usor de tinut minte. Intern,
implementarea trebuie sa refoloseasca aceeasi logica sigura.

### `snap make-private owner/repo`

Alias friendly pentru:

```bash
snap remote visibility private owner/repo
```

Mai putin riscant decat `make-public`, dar tot trebuie confirmare si output clar.

### `snap remote delete owner/repo`

Scop: sterge repo-ul GitHub remote.

Comanda GitHub CLI:

```bash
gh repo delete owner/repo --yes
```

Reguli stricte:

- nu sterge fisiere locale;
- nu sterge `.git` local;
- cere tastarea exacta `owner/repo`;
- nu accepta confirmare simpla `y`;
- ar trebui sa afiseze URL-ul repo-ului inainte.

### `snap delete-repo owner/repo`

Alias friendly pentru:

```bash
snap remote delete owner/repo
```

Trebuie tratat ca una dintre cele mai sensibile comenzi din aplicatie.

## 5.4 Branch workflow

Branchurile sunt un pas important pentru ca `snap` sa devina mai apropiat de Git.

### `snap branch list`

Comenzi Git posibile:

```bash
git branch --list
git branch -vv
```

Output propus:

```text
[snap] Branches

  Branch       Upstream       Status
  -----------  ------------  ----------------
* main         origin/main   up to date
  feature-ui   origin/feature-ui ahead 2
```

### `snap branch new <name>`

Comanda Git:

```bash
git switch -c <name>
```

Reguli:

- refuza daca working tree-ul are schimbari care pot fi pierdute;
- valideaza numele branchului;
- afiseaza comanda de sync recomandata.

### `snap branch switch <name>`

Comanda Git:

```bash
git switch <name>
```

Reguli:

- daca exista schimbari locale, explica optiunile:
  - `snap save "message"`;
  - `git stash` in viitor poate deveni `snap stash`;
  - cancel.

### `snap branch delete <name>`

Comanda Git:

```bash
git branch -d <name>
```

Pentru force:

```bash
git branch -D <name>
```

Reguli:

- default foloseste delete sigur (`-d`);
- pentru force trebuie flag explicit `--force`;
- nu permite stergerea branchului curent;
- cere confirmare daca branchul nu este impins.

### `snap branch merge <name>`

Comanda Git:

```bash
git merge <name>
```

Reguli:

- ruleaza doar cu working tree curat;
- explica daca apar conflicte;
- nu incearca sa rezolve conflicte automat.

## 6. `snap list` cu branchuri

In prezent, snapshoturile sunt taguri Git. Tagurile sunt globale in repo, nu apartin
natural unui branch. Totusi, commiturile catre care pointeaza tagurile pot fi
reachable din unul sau mai multe branchuri.

De aceea, `snap list` branch-aware trebuie sa fie definit prin reachability.

### 6.1 Comportament default

Recomandare:

```bash
snap list
```

Afiseaza snapshoturile relevante pentru branchul curent. Asta inseamna snapshoturi
ale caror commituri sunt reachable din branchul curent.

Exemplu:

```text
[snap] Snapshots for "my-app"
[snap] Branch: main

  Label  Branch  Description       Timestamp         State
  -----  ------  ----------------  ----------------  --------
  v126   main    release scripts   2026-05-13 18:20  active
  v125   main    resynced          2026-04-02 22:24
```

### 6.2 Toate branchurile

```bash
snap list --all-branches
```

Afiseaza toate snapshoturile, cu o coloana `Branch`.

Exemplu:

```text
[snap] Snapshots for "my-app"

  Label     Branch       Description       Timestamp         State
  --------  -----------  ----------------  ----------------  --------
  v126      main         release scripts   2026-05-13 18:20  active
  ui-v3     feature-ui   button cleanup    2026-05-12 11:10
  shared-1  shared       base refactor     2026-05-10 09:00
```

### 6.3 Filtru explicit

```bash
snap list --branch feature-ui
```

Afiseaza snapshoturile reachable din branchul cerut.

### 6.4 Snapshoturi reachable din mai multe branchuri

Daca un snapshot este reachable din mai multe branchuri, exista doua variante:

1. afisam branchul curent daca este printre ele;
2. altfel afisam `shared`.

Recomandare pentru v1:

```text
shared
```

Motiv: evita o lista lunga de branchuri in tabel. In viitor se poate adauga:

```bash
snap info v125
```

care sa arate toate branchurile relevante.

## 7. Token si autentificare

Autentificarea este una dintre cele mai importante parti ale acestui upgrade.

### 7.1 Regula principala

```text
Tokenul nu se hardcodeaza.
Tokenul nu se salveaza in repo.
Tokenul nu se afiseaza in terminal.
Tokenul nu se cere la fiecare comanda daca exista un credential manager sigur.
```

### 7.2 Daca tokenul vechi a fost real

Daca tokenul din scripturile batch a fost un token real, trebuie considerat compromis.
Actiuni recomandate:

1. mergi in GitHub;
2. deschide Developer settings;
3. revoca tokenul;
4. creeaza alt token doar daca este nevoie;
5. prefera `gh auth login` in loc de token manual.

### 7.3 Abordarea v1 recomandata: GitHub CLI

Pentru prima versiune, cea mai buna abordare este sa folosim GitHub CLI.

Utilizatorul ruleaza:

```bash
gh auth login
```

`snap` verifica:

```bash
gh auth status
```

Avantaje:

- `gh` gestioneaza tokenul deja;
- nu implementam noi stocare de secrete;
- merge pe Windows, Linux si WSL;
- GitHub CLI are comenzi stabile pentru repo create/edit/delete;
- reduce riscul de securitate.

Comenzi `gh` folosite:

```bash
gh repo create owner/repo --private --source=. --remote=origin
gh repo create owner/repo --public --source=. --remote=origin
gh repo edit owner/repo --visibility public
gh repo edit owner/repo --visibility private
gh repo delete owner/repo
gh repo view owner/repo
```

### 7.4 Abordarea v2: `snap auth`

Daca vrem ca `snap` sa nu depinda complet de `gh`, putem adauga:

```bash
snap auth login
snap auth status
snap auth logout
```

Stocarea tokenului trebuie facuta prin OS keychain:

- Windows Credential Manager;
- macOS Keychain;
- Linux Secret Service/libsecret;
- fallback controlat doar daca utilizatorul accepta explicit.

In Rust, o optiune probabila este un crate de tip `keyring`. Trebuie evaluat
separat pentru compatibilitate Windows/WSL/Linux.

### 7.5 De ce nu criptare manuala simpla

Criptarea manuala intr-un fisier local pare atractiva, dar de obicei muta problema:

- unde tinem cheia?
- cum o protejam?
- cum functioneaza pe Windows si Linux?
- ce facem in WSL?
- cum evitam ca tokenul sa ajunga in loguri?

De aceea, pentru v1, `gh auth login` este abordarea mai robusta.

## 8. Help si UX educational

Un obiectiv central este ca `snap` sa explice comenzile.

### 8.1 `snap --help`

Output conceptual:

```text
Snap - friendly Git snapshots and workflow helper

Daily workflow:
  snap status                 Show project status
  snap save "message"         Stage and commit changes safely
  snap sync                   Pull/push branch, snapshots, and metadata

Snapshots:
  snap new <label> [desc]     Create a named snapshot
  snap list                   List snapshots for current branch
  snap restore <label>        Restore a snapshot

Branches:
  snap branch list
  snap branch new <name>
  snap branch switch <name>

GitHub:
  snap remote create owner/repo --private
  snap make-public owner/repo
  snap make-private owner/repo

Learn:
  snap examples
  snap remote --help
  snap branch --help
```

### 8.2 `snap examples`

Comanda `snap examples` ar putea afisa workflow-uri reale.

Exemplu:

```text
Create a project and push it to GitHub:
  snap init
  snap new v1 "initial version"
  snap remote create glooring/my-app --private
  snap sync

Save normal work:
  snap status
  snap save "fix layout"
  snap sync

Work on a branch:
  snap branch new feature-login
  snap save "start login form"
  snap sync
```

### 8.3 Help pentru comenzi riscante

`snap delete-repo --help` trebuie sa fie explicit:

```text
Deletes the GitHub repository, not your local files.
Requires exact repository-name confirmation.

Example:
  snap delete-repo glooring/my-old-project
```

## 9. Release workflow

Scripturile de release deja introduse pot deveni in viitor comenzi `snap`.

Comenzi propuse:

```bash
snap release windows
snap release linux
snap release all
snap release list
```

Pentru inceput, aceste comenzi pot apela scripturile existente:

```text
scripts/release-windows.ps1
scripts/release-linux.sh
```

Dar pe termen lung, ar fi mai curat ca logica sa fie implementata in Rust sau
organizata ca taskuri de build cu output consistent.

Reguli:

- release ruleaza testele;
- release citeste versiunea din Cargo;
- release produce artefacte in `release-github/vX.Y.Z`;
- release nu commituieste artefactele;
- release explica ce fisiere trebuie puse in GitHub Release.

## 10. Arhitectura interna recomandata

Pentru implementare, ar fi bine sa nu punem toata logica in `main.rs`.

Module posibile:

```text
src/commands/status.rs
src/commands/save.rs
src/commands/sync.rs
src/commands/remote.rs
src/commands/branch.rs
src/commands/examples.rs
src/git.rs
src/github.rs
src/auth.rs
```

### 10.1 `src/git.rs`

Wrapper sigur peste comenzi Git comune:

- run git command;
- parse status;
- current branch;
- has remote;
- detect merge/rebase in progress;
- push refs;
- fetch refs;
- branch operations.

### 10.2 `src/github.rs`

Wrapper peste GitHub CLI in v1:

- check `gh` installed;
- check auth status;
- create repo;
- edit visibility;
- delete repo;
- view repo.

### 10.3 `src/auth.rs`

Pentru v1:

- doar verifica `gh auth status`;
- afiseaza instructiuni pentru `gh auth login`.

Pentru v2:

- keychain integration;
- token storage;
- token removal.

## 11. Erori si failure modes

### 11.1 Repo fara Git

Mesaj:

```text
[snap] This folder is not a Git repository.
Run:
  snap init
```

### 11.2 Remote lipsa

Mesaj:

```text
[snap] No remote named origin is configured.
Create one:
  snap remote create owner/repo --private

Or connect an existing one:
  snap remote set-url git@github.com:owner/repo.git
```

### 11.3 GitHub CLI lipseste

Mesaj:

```text
[snap] GitHub CLI is required for this command.
Install it from:
  https://cli.github.com/

Then run:
  gh auth login
```

### 11.4 Nu exista autentificare GitHub

Mesaj:

```text
[snap] GitHub CLI is installed, but you are not authenticated.
Run:
  gh auth login
```

### 11.5 Working tree dirty la pull/sync

Mesaj:

```text
[snap] Your working tree has local changes.
Save them first:
  snap save "message"

Then retry:
  snap sync
```

### 11.6 Conflict in rebase/merge

Mesaj:

```text
[snap] Git has a rebase or merge in progress.
Resolve the conflict with Git, then run:
  snap status
  snap sync
```

## 12. Implementare in faze

### Faza 1: Design CLI si documentatie

Livrabile:

- acest roadmap;
- decizie asupra comenzilor finale;
- exemple de help;
- decizie v1 pentru autentificare prin `gh`.

### Faza 2: Daily workflow local

Comenzi:

```bash
snap status
snap save "message"
snap history
```

De ce intai:

- nu depind de GitHub;
- sunt usor de testat in repo-uri temporare;
- aduc valoare imediata.

### Faza 3: Sync Snap-aware

Comenzi:

```bash
snap push
snap pull
snap sync
```

Focus:

- branch curent;
- tags;
- `refs/snap-metadata/*`;
- mesaje bune pentru remote lipsa si upstream lipsa.

### Faza 4: Remote GitHub prin `gh`

Comenzi:

```bash
snap remote status
snap remote create owner/repo --private
snap remote create owner/repo --public
snap remote set-url <url>
```

Focus:

- verificare `gh`;
- verificare auth;
- remote `origin`;
- push initial Snap-aware.

### Faza 5: Aliasuri friendly

Comenzi:

```bash
snap setup-repo owner/repo --private
snap update-repo "message"
snap make-public owner/repo
snap make-private owner/repo
snap delete-repo owner/repo
```

Regula:

Aliasurile trebuie sa refoloseasca aceeasi logica interna ca subcomenzile
structurate, nu sa dubleze cod.

### Faza 6: Branch workflow

Comenzi:

```bash
snap branch list
snap branch new <name>
snap branch switch <name>
snap branch delete <name>
snap branch merge <name>
```

Plus:

```bash
snap list --branch <name>
snap list --all-branches
```

### Faza 7: Auth nativ optional

Comenzi:

```bash
snap auth login
snap auth status
snap auth logout
```

Aceasta faza trebuie facuta doar dupa ce modelul cu `gh` este stabil.

## 13. Test plan pentru viitoarea implementare

### 13.1 CLI parsing

Teste pentru:

- `snap status`;
- `snap save "message"`;
- `snap sync`;
- `snap remote create owner/repo --private`;
- `snap make-public owner/repo`;
- `snap branch new feature-x`;
- invalid arguments;
- help text pentru subcomenzi.

### 13.2 Repo temporar local

Teste cu `assert_fs`:

- repo nou;
- repo cu schimbari;
- repo fara schimbari;
- repo cu branchuri;
- repo cu taguri snapshot;
- repo cu `refs/snap-metadata/*`.

### 13.3 Sync refs

Teste specifice:

- `snap push` include taguri;
- `snap push` include metadata refs;
- `snap pull` fetch-uieste metadata refs;
- eroare clara cand remote lipseste.

### 13.4 Failure modes

Teste pentru:

- detached HEAD;
- merge in progres;
- rebase in progres;
- remote invalid;
- branch fara upstream;
- `gh` lipsa;
- `gh auth status` esuat.

### 13.5 GitHub CLI mock

In CI nu trebuie create repo-uri reale. Pentru comenzile GitHub, testele trebuie sa
foloseasca un `gh` fake in PATH care:

- captureaza argumentele;
- returneaza success/failure controlat;
- permite verificarea comenzilor generate.

### 13.6 Teste manuale

Inainte de folosirea pe proiecte mari:

1. teste pe repo gol;
2. teste pe repo mic privat;
3. teste cu snapshoturi si metadata;
4. teste cu branchuri;
5. teste cu WSL;
6. teste de make-public/make-private pe repo de test;
7. test de delete repo doar pe repo disposable.

## 14. Decizii recomandate pentru v1

Pentru prima implementare reala, recomandarile sunt:

- autentificare prin GitHub CLI, nu token stocat de `snap`;
- `snap save`, `snap status`, `snap sync` ca primul set util;
- `snap remote create` dupa ce sync local este stabil;
- aliasuri friendly doar dupa ce comenzile structurate exista;
- branch-aware `snap list` dupa ce branch commands sunt stabile;
- delete repo implementat ultimul dintre comenzile GitHub, din cauza riscului.

## 15. Exemplu de workflow final

### Proiect nou privat

```bash
snap init
snap new v1 "initial version"
snap remote create glooring/my-app --private
snap sync
```

### Lucru zilnic

```bash
snap status
snap save "fix toolbar layout"
snap sync
```

### Snapshot important

```bash
snap new v2 "stable toolbar release"
snap sync
```

### Branch nou

```bash
snap branch new feature-login
snap save "start login screen"
snap new login-v1 "first login milestone"
snap sync
```

### Publicare repo

```bash
snap make-public glooring/my-app
```

Comanda trebuie sa ceara confirmare stricta.

### Stergere repo remote

```bash
snap delete-repo glooring/old-test-repo
```

Comanda trebuie sa ceara tastarea exacta:

```text
glooring/old-test-repo
```

## 16. Concluzie

Directia este fezabila si utila. `snap` poate evolua natural dintr-un snapshot tool
intr-un workflow tool peste Git si GitHub.

Cheia este sa nu copiem fragilitatea scripturilor batch. Trebuie sa pastram ideea
buna: comenzi scurte care fac mai multe lucruri. Dar implementarea trebuie sa fie
sigura:

- fara token hardcodat;
- fara pasi ignorati dupa erori;
- fara delete/public fara confirmare serioasa;
- cu suport pentru snapshot tags si metadata refs;
- cu help bun si exemple concrete.

Aceasta abordare ar face `snap` mai profesionist, mai util zilnic si mai potrivit
pentru proiecte reale pe Windows, Linux si WSL.
