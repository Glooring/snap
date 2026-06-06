# Snap Friendly Git Workflow - Implementare Curenta

Acest document rezuma ce a fost adaugat in `snap` pentru transformarea dintr-un
snapshot manager intr-un layer prietenos peste Git si GitHub. Roadmap-ul de design
ramane in `doc/SNAP_GIT_WORKFLOW_ROADMAP.md`; documentul acesta descrie ce exista
acum in CLI.

## 1. Directia noua

`snap` ramane bazat pe Git. Nu inlocuieste motorul Git si nu salveaza un istoric
paralel. In schimb, impacheteaza workflow-uri Git/GitHub in comenzi mai clare,
cu verificari si mesaje mai bune.

Modelul curent:

```text
snap = snapshot manager + friendly Git workflow layer + GitHub release helper
```

Comenzile clasice de snapshot raman compatibile:

```bash
snap init
snap new v1 "initial version"
snap list
snap restore v1
snap update
snap delete v1
snap doctor
snap options
```

Peste ele au fost adaugate workflow-uri pentru lucru zilnic, branchuri, remote
GitHub si release-uri.

## 2. Daily Git workflow

### `snap status`

Afiseaza o vedere prietenoasa asupra repo-ului:

- branch curent;
- remote `origin`;
- upstream;
- ahead/behind cand Git poate determina asta;
- sumar working tree;
- snapshot activ si latest snapshot.

Comanda este read-only.

```bash
snap status
```

### `snap save "message"`

Face un commit Git normal, fara snapshot tag.

Comportament:

- refuza detached HEAD;
- refuza merge/rebase/cherry-pick/revert in progres;
- daca nu exista schimbari, iese cu success si mesaj clar;
- ruleaza staging complet si commit cu mesajul dat;
- nu modifica metadata Snap si nu creeaza snapshot.

```bash
snap save "fixed login flow"
```

Daca mesajul lipseste, `snap` poate intreba interactiv.

### `snap push`

Impinge branchul curent si datele Snap importante:

- branch curent;
- snapshot tags;
- `refs/snap-metadata/*`, daca exista.

Daca branchul nu are upstream, foloseste push cu upstream setat.

```bash
snap push
```

### `snap pull`

Aduce datele importante din remote:

- branch updates;
- tags;
- `refs/snap-metadata/*`, daca exista remote.

Refuza working tree dirty, ca sa nu amestece schimbari locale cu rebase/pull.

```bash
snap pull
```

### `snap sync`

Combina pull + push Snap-aware.

```bash
snap sync
```

Este comanda principala dupa `snap save`, `snap new`, `snap branch new` sau dupa
lucru pe branchuri.

### `snap update-repo "message"`

Alias friendly pentru:

```bash
snap save "message"
snap sync
```

Nu dubleaza logica; refoloseste implementarea interna a comenzilor `save` si
`sync`.

```bash
snap update-repo "implemented release upload"
```

### `snap history`

Afiseaza istoricul Git curent intr-o forma scurta, cu decorations si snapshot tags.

```bash
snap history
snap history 50
snap history all
```

Comportament:

- default: ultimele 20 commituri;
- limita numerica trebuie sa fie pozitiva;
- `all` afiseaza tot istoricul curent;
- merge si cu working tree dirty, fiind read-only;
- repo fara commituri afiseaza mesaj clar.

## 3. Remote si GitHub workflow

Autentificarea GitHub se face prin GitHub CLI. `snap` nu cere si nu stocheaza
tokenuri proprii.

Setup recomandat:

```bash
gh auth login
```

### `snap remote status`

Afiseaza remote-urile Git si statusul GitHub CLI. Daca `origin` este GitHub,
incearca sa afiseze repo-ul, URL-ul si visibility.

```bash
snap remote status
```

### `snap remote create owner/repo --private`

Creeaza un repo GitHub prin `gh repo create`, il conecteaza ca `origin` si ruleaza
un push initial Snap-aware daca exista commituri.

```bash
snap remote create owner/repo --private
snap remote create owner/repo --public
```

Default-ul este private daca nu se alege explicit `--public`.

### `snap setup-repo owner/repo --private`

Alias friendly pentru `snap remote create`.

```bash
snap setup-repo owner/repo --private
snap setup-repo owner/repo --public
```

### `snap remote set-url <url>`

Adauga `origin` cand lipseste.

```bash
snap remote set-url https://github.com/owner/repo.git
```

In implementarea curenta refuza sa inlocuiasca un `origin` existent automat.

### `snap remote visibility public owner/repo`

Schimba visibility pe GitHub prin `gh repo edit`.

```bash
snap remote visibility public owner/repo
snap remote visibility private owner/repo
```

Reguli:

- `public` cere tastarea exacta `owner/repo`;
- `private` cere confirmare explicita;
- mesajele spun clar ca se modifica repo-ul GitHub, nu fisierele locale.

### `snap make-public` si `snap make-private`

Aliasuri friendly:

```bash
snap make-public owner/repo
snap make-private owner/repo
```

Folosesc aceeasi logica interna ca `snap remote visibility`.

### `snap remote delete owner/repo`

Sterge repository-ul GitHub remote prin `gh repo delete owner/repo --yes`.

```bash
snap remote delete owner/repo
```

Reguli:

- cere tastarea exacta `owner/repo`;
- spune clar ca sterge repo-ul GitHub remote, nu fisierele locale;
- nu modifica working tree-ul local si nu sterge folderul proiectului;
- ruleaza `gh auth status` inainte de stergere;
- daca tokenul GitHub CLI nu are scope-ul `delete_repo`, mesajul recomanda
  `gh auth refresh -s delete_repo`.

### `snap delete-repo owner/repo`

Alias friendly pentru `snap remote delete`.

```bash
snap delete-repo owner/repo
```

Este intentionat separat de `snap delete <snapshot>`, ca sa fie clar cand este
vorba despre stergerea unui repository GitHub.

## 4. Branch workflow

Comenzile `snap branch` sunt wrapper-e locale peste Git branch operations.

### `snap branch list`

Afiseaza branchurile locale, branchul curent, upstream si status tracking cand Git
poate determina asta.

```bash
snap branch list
```

### `snap branch new <name>`

Valideaza numele branchului, cere working tree curat si ruleaza `git switch -c`.

```bash
snap branch new feature-login
```

### `snap branch switch <name>`

Valideaza branchul local si cere working tree curat inainte sa comute.

```bash
snap branch switch main
```

Daca esti deja pe branchul cerut, comanda iese cu success.

### `snap branch delete <name>`

Sterge branch local cu safe delete.

```bash
snap branch delete feature-login
```

Pentru force delete:

```bash
snap branch delete old-experiment --force
```

Reguli:

- refuza branchul curent;
- default foloseste delete sigur;
- `--force` cere tastarea exacta a numelui branchului.

### `snap branch merge <name>`

Ruleaza merge local intr-un working tree curat.

```bash
snap branch merge feature-login
```

Daca apar conflicte, acestea raman de rezolvat explicit cu Git.

## 5. `snap list` branch-aware

Comportamentul default al `snap list` a ramas compatibil.

Au fost adaugate optiuni explicite:

```bash
snap list --branch main
snap list --all-branches
```

Reguli:

- filtrarea se face dupa Git reachability;
- un snapshot apartine unui branch daca commitul snapshotului este reachable din branchul local;
- `--branch` si `--all-branches` sunt mutual exclusive;
- pentru `--all-branches`, output-ul include coloana `Branch`;
- snapshoturile reachable din mai multe branchuri pot aparea ca `shared` sau `<current> (shared)`;
- snapshoturile fara branch local reachable apar cu `-`.

## 6. Release workflow local

Scripturile automate de release sunt:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\release-windows.ps1
```

```bash
bash ./scripts/release-linux.sh
```

Ele produc artefactele in:

```text
release-github/vX.Y.Z
```

Artefacte Windows:

```text
snap-vX.Y.Z-windows-x86_64.exe
snap-vX.Y.Z-windows-x86_64-setup.exe
snap-vX.Y.Z-windows-x86_64.msi
```

Artefacte Linux:

```text
snap-vX.Y.Z-linux-x86_64
snap-vX.Y.Z-linux-x86_64.tar.gz
```

Folderul `release-github/` este ignorat de Git, ca artefactele binare sa nu fie
commitate accidental.

## 7. `snap release`

Peste scripturile de release a fost adaugat un wrapper CLI.

### `snap release windows`

Ruleaza scriptul Windows:

```bash
snap release windows
```

Intern foloseste PowerShell si scriptul `scripts/release-windows.ps1`.

### `snap release linux`

Ruleaza scriptul Linux/WSL:

```bash
snap release linux
```

Intern foloseste `bash` si scriptul `scripts/release-linux.sh`.

### `snap release all`

Verifica runtime-urile pentru ambele platforme si ruleaza Windows apoi Linux.

```bash
snap release all
```

Nu face bridge automat Windows/WSL. Daca mediul curent nu poate rula ambele
scripturi, trebuie rulate separat comenzile Windows si Linux in mediile potrivite.

### Override-uri pentru teste si medii custom

Runnerul accepta env vars:

```text
SNAP_RELEASE_WINDOWS_SCRIPT
SNAP_RELEASE_LINUX_SCRIPT
SNAP_RELEASE_POWERSHELL
SNAP_RELEASE_BASH
```

Acestea sunt utile pentru teste si medii custom, nu pentru workflow-ul normal.

## 8. GitHub Release upload

### `snap release upload`

Uploadeaza cele 5 artefacte din `release-github/vX.Y.Z` catre GitHub Release.

```bash
snap release upload
snap release upload --repo owner/repo
snap release upload --publish
snap release upload --clobber
```

Comportament:

- citeste versiunea din Cargo;
- foloseste tagul `vX.Y.Z`;
- cere existenta tuturor celor 5 artefacte;
- infereaza repo-ul GitHub din `origin`, daca `--repo` lipseste;
- verifica `gh auth status`;
- creeaza release draft implicit;
- cu `--publish`, creeaza release public direct;
- daca release-ul exista, refuza fara `--clobber`;
- cu `--clobber`, uploadeaza peste asseturi existente cu acelasi nume;
- nu ruleaza build scripturile si nu face `snap sync`.

Workflow recomandat:

```bash
snap release windows
snap release linux
snap release upload
```

### `snap release list`

Listeaza GitHub Releases pentru repo-ul curent.

```bash
snap release list
snap release list 20
snap release list --repo owner/repo
```

Comportament:

- default: ultimele 10 release-uri;
- limita trebuie sa fie un numar pozitiv;
- repo-ul se infereaza din `origin`, daca `--repo` lipseste;
- output tabelar: `Tag`, `Name`, `State`, `Published`, `Latest`;
- `State` poate fi `draft`, `prerelease` sau `published`;
- daca nu exista release-uri, afiseaza mesaj clar si iese cu success.

## 9. Help si exemple

Au fost imbunatatite:

```bash
snap --help
snap remote --help
snap branch --help
snap release --help
snap examples
```

`snap --help` grupeaza mental comenzile:

- Daily workflow;
- Snapshots;
- Branches;
- Remote/GitHub;
- Release;
- Diagnostics;
- Learn by example.

`snap examples` afiseaza workflow-uri practice pentru:

- proiect cu remote GitHub;
- creare repo privat;
- comenzi structurate in loc de aliasuri;
- snapshot important;
- branch local;
- release assets.

## 10. Arhitectura adaugata

Au fost adaugate sau extinse module interne pentru separarea responsabilitatilor:

```text
src/git.rs
src/github.rs
src/commands/status.rs
src/commands/save.rs
src/commands/push.rs
src/commands/pull.rs
src/commands/sync.rs
src/commands/update_repo.rs
src/commands/setup_repo.rs
src/commands/remote.rs
src/commands/branch.rs
src/commands/history.rs
src/commands/release.rs
src/commands/examples.rs
```

Principii folosite:

- comenzile Git/GitHub sunt rulate cu argumente structurate, nu stringuri shell;
- tokenurile GitHub nu sunt cerute sau stocate de `snap`;
- operatiile sensibile cer confirmari explicite;
- logica aliasurilor refoloseste comenzile structurate;
- workflow-urile Snap-aware includ tags si `refs/snap-metadata/*`.

## 11. Siguranta

Protectii adaugate:

- detached HEAD este refuzat pentru comenzi de write relevante;
- merge/rebase/cherry-pick/revert in progres sunt detectate pentru comenzi care pot muta HEAD sau modifica branchuri;
- branch switch/delete/merge cer working tree curat;
- `make-public` cere tastarea exacta a repo-ului;
- `branch delete --force` cere tastarea exacta a branchului;
- `release upload` refuza release existent fara `--clobber`;
- GitHub auth ramane in GitHub CLI prin `gh auth login`.

## 12. Testare adaugata

Suita de teste integration a fost extinsa pentru:

- parsing/help pentru comenzile noi;
- daily workflow: `status`, `save`, `history`, `push`, `pull`, `sync`;
- remote/GitHub cu `gh` fake;
- visibility public/private si aliasuri;
- branch workflow local;
- `snap list --branch` si `snap list --all-branches`;
- release script wrappers cu runtime fake;
- release upload/list cu `gh` fake;
- failure modes pentru repo lipsa, auth failure, artefacte lipsa, limite invalide, branchuri invalide.

La ultimul batch documentat, testele complete treceau:

```text
cargo fmt -- --check
git diff --check
cargo check
cargo test
```

cu 96 teste integration trecute.

## 13. Ce nu este implementat inca

Comenzi intentionat amanate:

```bash
snap auth login
snap auth status
snap auth logout
```

Motive:

- auth nativ cere OS keychain si design separat pentru Windows/WSL/Linux;
- momentan strategia sigura este GitHub CLI: `gh auth login`.

## 14. Ghid practic dupa instalarea noii versiuni

Aceasta sectiune este un how-to pentru folosirea comenzilor noi pe un proiect
nou sau existent. Output-urile de mai jos sunt exemple aproximative; valorile
reale depind de numele proiectului, branch, remote si snapshoturile existente.

### 14.1 Verificare instalare

Dupa ce instalezi noul `snap`, deschide un terminal nou. Pe Windows:

```powershell
where snap
snap --version
snap --help
```

Pe WSL/Linux:

```bash
which snap
snap --version
snap --help
```

La `snap --help` trebuie sa vezi comenzile noi:

```text
status
save
push
pull
sync
history
branch
remote
release
update-repo
setup-repo
make-public
make-private
delete-repo
examples
```

Daca terminalul gaseste o versiune veche, verifica PATH. Pe Windows, `where snap`
trebuie sa arate executabilul pe care tocmai l-ai instalat sau copiat.

### 14.2 Setup GitHub CLI

Comenzile GitHub din `snap` folosesc GitHub CLI (`gh`). `snap` nu salveaza token
propriu si nu iti cere token in terminal.

Prima data:

```powershell
gh auth login -h github.com -p https -w
gh auth status
gh api user --jq .login
```

Pentru operatii normale, scope-urile uzuale sunt suficiente:

```text
repo
read:org
gist
workflow
```

Pentru stergere de repository GitHub este nevoie si de:

```text
delete_repo
```

Adauga acest scope doar cand vrei sa testezi sau sa folosesti stergerea de repo:

```powershell
gh auth refresh -h github.com -s delete_repo
gh auth status
```

La `gh auth status`, verifica faptul ca userul activ este contul dorit si ca
lista `Token scopes` contine `delete_repo`. Nu copia tokenul in chat sau in
fisiere.

### 14.3 Proiect nou local

Exemplu:

```powershell
mkdir D:\Projects\my-new-project
cd D:\Projects\my-new-project
```

Creeaza cateva fisiere:

```powershell
Set-Content README.md "# My New Project"
mkdir src
Set-Content src\main.txt "initial content"
```

Initializeaza proiectul Snap:

```powershell
snap init
```

Output asteptat:

```text
[snap] Initialized empty snap repository.
```

Verifica starea:

```powershell
snap status
```

Output tipic intr-un proiect nou:

```text
[snap] Project status
  OK Branch: master
  WARN Remote origin: not configured
  WARN Upstream: none
  WARN Working tree: 2 untracked
  WARN Active snapshot: none
  WARN Latest snapshot: none
```

Branchul poate fi `main` sau `master`, in functie de configuratia Git.

### 14.4 Primul snapshot

Un snapshot este un punct important la care vrei sa poti reveni usor.

```powershell
snap new v1 "initial project state"
```

Output tipic:

```text
[snap] Step 1/4: Scanning for metadata (hidden files, empty dirs)...
[snap] Step 2/4: Staging all files...
[snap] Step 3/4: Creating the commit...
[snap] Step 4/4: Creating the annotated snapshot tag...

[snap] New snapshot created: [abc1234] v1
```

Listeaza snapshoturile:

```powershell
snap list
```

Output tipic:

```text
[snap] Snapshots for "my-new-project":

  Label  Description            Timestamp
  -----  ---------------------  ----------------
  v1     initial project state  2026-05-13 18:30   (active)
```

### 14.5 Diferenta dintre `snap save` si `snap new`

Foloseste `snap save` pentru commituri normale de zi cu zi:

```powershell
snap save "implemented login form"
```

`snap save`:

- face commit Git normal;
- nu creeaza snapshot tag;
- nu schimba metadata Snap;
- este bun pentru pasi mici si desi.

Output tipic:

```text
[snap] Staging all changes...
[snap] Creating Git commit...

[snap] Changes saved with commit message: implemented login form
```

Foloseste `snap new` pentru puncte importante:

```powershell
snap new v2 "login form stable"
```

`snap new`:

- face commit;
- creeaza tag de snapshot;
- salveaza metadata pentru empty dirs / hidden files;
- apare in `snap list`;
- poate fi restaurat usor cu `snap restore`.

Regula practica:

```text
snap save  = commit normal
snap new   = versiune importanta / checkpoint
```

### 14.6 Istoric proiect

Vezi ultimele commituri si snapshot tags:

```powershell
snap history
```

Output tipic:

```text
[snap] Project history

* 5b092be (HEAD -> master, tag: v2) Snapshot: v2
* 37a8c10 implemented login form
* b55dadd (tag: v1) Snapshot: v1
```

Mai multe commituri:

```powershell
snap history 50
```

Tot istoricul branchului curent:

```powershell
snap history all
```

Comanda este read-only si merge chiar daca working tree-ul este dirty.

### 14.7 Conectare la GitHub cu repo privat

Pentru un proiect nou, recomandat este:

```powershell
snap setup-repo Dan1579/my-new-project --private
```

Aceasta comanda este alias pentru:

```powershell
snap remote create Dan1579/my-new-project --private
```

Ce face:

- verifica `gh auth status`;
- creeaza repository GitHub privat;
- adauga `origin`;
- impinge branchul curent;
- impinge snapshot tags;
- impinge `refs/snap-metadata/*`.

Output tipic:

```text
[snap] setup-repo is a shortcut for `snap remote create`.
[snap] Creating GitHub repository Dan1579/my-new-project as private...
[snap] Running initial snap-aware push...
[snap] Pushing current branch...
[snap] Pushing snapshot tags...
[snap] Pushing snap metadata refs...
[snap] Remote repository created and branch 'master' pushed.
```

Verifica remote-ul:

```powershell
snap remote status
```

Output tipic:

```text
[snap] Remote status
  OK origin: https://github.com/Dan1579/my-new-project.git
  OK GitHub auth: github.com
  OK GitHub repo: Dan1579/my-new-project
  OK Visibility: PRIVATE
  OK URL: https://github.com/Dan1579/my-new-project
```

Daca repo-ul exista deja pe GitHub si vrei doar sa conectezi proiectul local:

```powershell
snap remote set-url https://github.com/Dan1579/my-existing-project.git
snap sync
```

`snap remote set-url` adauga `origin` doar daca lipseste. In versiunea curenta nu
inlocuieste automat un `origin` existent.

### 14.8 Workflow zilnic recomandat

La inceput:

```powershell
snap status
```

Dupa modificari normale:

```powershell
snap update-repo "implemented settings page"
```

Aceasta comanda face:

```powershell
snap save "implemented settings page"
snap sync
```

Output tipic:

```text
[snap] update-repo is a shortcut for `snap save` + `snap sync`.
[snap] Staging all changes...
[snap] Creating Git commit...
[snap] Changes saved with commit message: implemented settings page
[snap] Syncing project with origin...
[snap] Fetching branch updates...
[snap] Fetching snapshot tags...
[snap] Fetching snap metadata refs...
[snap] Pulling current branch with rebase...
[snap] Pushing current branch...
[snap] Pushing snapshot tags...
[snap] Pushing snap metadata refs...

[snap] Sync complete.
```

Daca nu ai schimbari locale, `snap update-repo` nu creeaza commit nou, dar poate
rula sync pentru commiturile existente.

Pentru un checkpoint important:

```powershell
snap new v3 "before payment refactor"
snap sync
```

### 14.9 `snap push`, `snap pull`, `snap sync`

Foloseste `snap sync` in mod normal. Este varianta prietenoasa pentru proiectele
Snap, pentru ca sincronizeaza si snapshot tags si metadata refs.

```powershell
snap sync
```

Ce sincronizeaza:

- branchul curent;
- snapshot tags;
- `refs/snap-metadata/*`.

`snap push` impinge doar spre remote:

```powershell
snap push
```

Output tipic:

```text
[snap] Pushing current branch...
[snap] Pushing snapshot tags...
[snap] Pushing snap metadata refs...

[snap] Push complete.
  Branch: master
  Snapshot tags: pushed
  Snap metadata refs: 1
```

`snap pull` aduce din remote:

```powershell
snap pull
```

Output tipic:

```text
[snap] Fetching branch updates...
[snap] Fetching snapshot tags...
[snap] Fetching snap metadata refs...
[snap] Pulling current branch with rebase...

[snap] Pull complete.
```

Important: `snap pull` refuza working tree dirty. Daca ai fisiere modificate:

```powershell
snap save "work before pull"
snap pull
```

sau curata/modifica manual schimbarile inainte.

### 14.10 Lucru cu branchuri

Listeaza branchurile:

```powershell
snap branch list
```

Output tipic:

```text
[snap] Branches

     Branch                       Upstream                     Status
     ------                       --------                     ------
  *  master                       origin/master                up to date
     feature-login                -                            local only
```

Creeaza branch nou si comuta pe el:

```powershell
snap branch new feature-login
```

Output tipic:

```text
[snap] Creating and switching to branch 'feature-login'...
[snap] Now on branch 'feature-login'.
  Next: run `snap sync` when you want to publish this branch.
```

Lucreaza normal:

```powershell
snap save "start login feature"
snap new login-v1 "first stable login version"
snap sync
```

Comuta inapoi:

```powershell
snap branch switch master
```

sau:

```powershell
snap branch switch main
```

Merge:

```powershell
snap branch merge feature-login
```

Output tipic:

```text
[snap] Merging branch 'feature-login' into current branch...
[snap] Branch 'feature-login' merged.
```

Sterge branch local dupa merge:

```powershell
snap branch delete feature-login
```

Pentru un branch nemerguit, Git poate refuza safe delete. Daca esti sigur:

```powershell
snap branch delete feature-login --force
```

`--force` cere tastarea exacta a numelui branchului.

Reguli de siguranta:

- `new`, `switch`, `delete`, `merge` cer working tree curat;
- nu poti sterge branchul curent;
- `switch` pentru branchul curent iese cu success;
- conflictele de merge raman de rezolvat explicit cu Git.

### 14.11 `snap list` pe branchuri

Default ramane neschimbat:

```powershell
snap list
```

Filtreaza dupa branch local:

```powershell
snap list --branch master
snap list --branch feature-login
```

Output tipic:

```text
[snap] Snapshots for "my-new-project":
[snap] Branch: feature-login

  Label     Description                  Timestamp
  --------  ---------------------------  ----------------
  login-v1  first stable login version   2026-05-13 18:50   (active)
  v1        initial project state        2026-05-13 18:30
```

Vezi snapshoturile cu branch column:

```powershell
snap list --all-branches
```

Output tipic:

```text
[snap] Snapshots for "my-new-project":

  Label     Branch                  Description                  Timestamp
  --------  ----------------------  ---------------------------  ----------------
  login-v1  feature-login           first stable login version   2026-05-13 18:50
  v2        master (shared)         login form stable            2026-05-13 18:40
  old-test  -                       old detached test            2026-05-13 18:35
```

Semnificatie:

- `feature-login`: snapshot reachable din acel branch;
- `master (shared)`: snapshot reachable din mai multe branchuri, inclusiv branchul curent;
- `shared`: snapshot reachable din mai multe branchuri, dar nu din branchul curent;
- `-`: snapshotul nu este reachable din niciun branch local.

### 14.12 Schimbare visibility GitHub

Fa repo public:

```powershell
snap make-public Dan1579/my-new-project
```

sau forma structurata:

```powershell
snap remote visibility public Dan1579/my-new-project
```

Output tipic:

```text
[snap] Public visibility change
  Repository: Dan1579/my-new-project
  This changes the GitHub repository, not your local files.
  Warning: the repository and its history may become visible to anyone.
Type the repository name to confirm:
```

Trebuie sa tastezi exact:

```text
Dan1579/my-new-project
```

Fa repo privat:

```powershell
snap make-private Dan1579/my-new-project
```

sau:

```powershell
snap remote visibility private Dan1579/my-new-project
```

Output tipic:

```text
[snap] Private visibility change
  Repository: Dan1579/my-new-project
  This changes the GitHub repository, not your local files.
[snap] Continue changing this GitHub repository to private? [y/N]
```

Confirma cu:

```text
y
```

Verifica:

```powershell
gh repo view Dan1579/my-new-project --json visibility --jq .visibility
```

### 14.13 Stergere repository GitHub

Aceasta este cea mai destructiva comanda din workflow-ul GitHub.

Forma recomandata:

```powershell
snap delete-repo Dan1579/my-test-repo
```

Forma structurata:

```powershell
snap remote delete Dan1579/my-test-repo
```

Ce face:

- sterge repository-ul GitHub remote;
- nu sterge folderul local;
- nu sterge fisierele locale;
- cere confirmare stricta prin tastarea exacta `owner/repo`;
- foloseste `gh repo delete owner/repo --yes` dupa confirmarea din `snap`.

Output tipic:

```text
[snap] delete-repo is a shortcut for `snap remote delete`.

[snap] Delete GitHub repository
  Repository: Dan1579/my-test-repo
  This deletes the GitHub repository, not your local files.
  Warning: this can remove remote branches, tags, releases, issues, and settings.
  This action cannot be undone on GitHub.
Type the repository name to confirm deletion:
```

Trebuie sa tastezi exact:

```text
Dan1579/my-test-repo
```

Success:

```text
[snap] Deleting GitHub repository Dan1579/my-test-repo...
[snap] GitHub repository Dan1579/my-test-repo was deleted. Local files were not changed.
```

Daca tokenul nu are `delete_repo`, vei vedea o eroare de tip:

```text
[snap] Error: Failed to delete GitHub repository 'Dan1579/my-test-repo'.
GitHub CLI may require `gh auth refresh -s delete_repo`.
```

Rezolvare:

```powershell
gh auth refresh -h github.com -s delete_repo
gh auth status
```

Dupa stergerea remote-ului, folderul local ramane pe disc. Poti sa il pastrezi,
sa schimbi remote-ul, sau sa il stergi manual daca nu mai ai nevoie de el.

### 14.14 Release local

Pentru release complet ai nevoie de cele 5 artefacte:

```text
snap-vX.Y.Z-windows-x86_64.exe
snap-vX.Y.Z-windows-x86_64-setup.exe
snap-vX.Y.Z-windows-x86_64.msi
snap-vX.Y.Z-linux-x86_64
snap-vX.Y.Z-linux-x86_64.tar.gz
```

Pe Windows:

```powershell
snap release windows
```

Output tipic:

```text
[snap] Release folder: release-github/v7.2.0
[snap] Expected Windows artifacts:
  snap-v7.2.0-windows-x86_64.exe
  snap-v7.2.0-windows-x86_64-setup.exe
  snap-v7.2.0-windows-x86_64.msi
```

Pe WSL/Linux:

```bash
snap release linux
```

Output tipic:

```text
[snap] Release folder: release-github/v7.2.0
[snap] Expected Linux artifacts:
  snap-v7.2.0-linux-x86_64
  snap-v7.2.0-linux-x86_64.tar.gz
```

`snap release all` ruleaza ambele scripturi doar daca mediul curent poate rula
si PowerShell, si Bash:

```powershell
snap release all
```

Pe masina Windows + WSL, de obicei este mai clar sa rulezi separat:

```powershell
snap release windows
```

si in WSL:

```bash
snap release linux
```

### 14.15 Upload GitHub Release

Dupa ce exista toate cele 5 artefacte:

```powershell
snap release upload
```

Ce face:

- citeste versiunea din Cargo;
- cauta folderul `release-github/vX.Y.Z`;
- verifica toate cele 5 artefacte;
- infereaza repo-ul GitHub din `origin`;
- verifica `gh auth status`;
- creeaza draft release implicit.

Output tipic:

```text
[snap] GitHub release upload
  Repository: Dan1579/snap
  Release: v7.2.0
  Folder: release-github\v7.2.0
[snap] Creating draft GitHub Release...
[snap] Uploaded 5 release asset(s) for v7.2.0.
```

Daca vrei publicare directa:

```powershell
snap release upload --publish
```

Daca release-ul exista si vrei sa reincarci asseturile:

```powershell
snap release upload --clobber
```

Daca `origin` nu este GitHub sau vrei alt repo:

```powershell
snap release upload --repo Dan1579/snap
```

Important:

- `snap release upload` nu ruleaza build;
- nu face `snap sync`;
- nu commituieste artefacte;
- daca target commit nu este pe remote, GitHub poate refuza release create;
- ruleaza `snap sync` inainte de upload daca ai commituri locale noi.

### 14.16 Listare GitHub Releases

```powershell
snap release list
```

Output tipic:

```text
[snap] GitHub releases
  Repository: Dan1579/snap
  Limit: 10

  Tag     Name         State      Published             Latest
  ------  -----------  ---------  --------------------  ------
  v7.2.0  snap v7.2.0  draft      0001-01-01T00:00:00Z  -
  v7.1.0  snap v7.1.0  published  2026-05-10T10:00:00Z  yes
```

Mai multe release-uri:

```powershell
snap release list 20
```

Alt repo:

```powershell
snap release list --repo Dan1579/snap
```

### 14.17 Comenzi de help utile

Cand nu mai stii exact sintaxa:

```powershell
snap --help
snap examples
snap remote --help
snap remote visibility --help
snap branch --help
snap branch delete --help
snap release --help
snap release upload --help
snap release list --help
```

`snap examples` este cel mai bun punct de pornire pentru workflow-uri complete.

### 14.18 Erori comune si ce faci

#### `Remote origin is already configured`

Inseamna ca proiectul are deja `origin`.

Verifica:

```powershell
git remote -v
snap remote status
```

In V1, `snap remote set-url` nu inlocuieste automat origin existent. Daca vrei sa
il schimbi intentionat, foloseste Git manual:

```powershell
git remote set-url origin https://github.com/owner/repo.git
snap remote status
```

#### `GitHub CLI is installed, but you are not authenticated`

Ruleaza:

```powershell
gh auth login -h github.com -p https -w
gh auth status
```

#### `This API operation needs the "delete_repo" scope`

Se intampla la `snap delete-repo`.

Ruleaza:

```powershell
gh auth refresh -h github.com -s delete_repo
gh auth status
```

#### `Working tree has local changes`

Comenzile care pot muta HEAD sau face rebase cer working tree curat.

Optiuni:

```powershell
snap save "work before operation"
```

sau anulezi/rezolvi manual schimbarile cu Git/editorul tau.

#### `Git HEAD is detached`

Esti pe un commit/tag direct, nu pe un branch. Comenzile noi de write refuza
aceasta stare.

Verifica:

```powershell
git status --short --branch
snap doctor
```

In multe cazuri poti reveni pe branch:

```powershell
git switch master
```

sau:

```powershell
git switch main
```

Daca nu esti sigur, ruleaza:

```powershell
snap doctor
```

#### `A merge/rebase/cherry-pick/revert is in progress`

Finalizeaza sau anuleaza operatia Git existenta inainte de comenzi Snap de write.

Exemple Git:

```powershell
git status
git rebase --continue
git rebase --abort
git merge --abort
```

#### `Release artifact missing`

`snap release upload` cere toate cele 5 artefacte. Ruleaza buildurile:

```powershell
snap release windows
```

in WSL:

```bash
snap release linux
```

#### `a Cargo.lock must exist for this command`

`snap release upload` citeste versiunea prin Cargo. Daca proiectul Rust nu are
`Cargo.lock`, genereaza-l:

```powershell
cargo generate-lockfile
```

Pentru repo-ul `snap`, `Cargo.lock` trebuie sa existe deja.

#### `Existing release found`

`snap release upload` nu suprascrie asseturi implicit.

Pentru retry intentionat:

```powershell
snap release upload --clobber
```

### 14.19 Workflow complet recomandat pentru proiect nou

```powershell
cd D:\Projects\my-new-project
snap init
snap status
snap new v1 "initial project state"
snap setup-repo Dan1579/my-new-project --private
snap remote status
```

Lucru zilnic:

```powershell
snap status
snap update-repo "implemented first feature"
snap history
```

Checkpoint important:

```powershell
snap new v2 "first stable version"
snap sync
snap list
```

Branch:

```powershell
snap branch new feature-x
snap save "start feature x"
snap new feature-x-v1 "feature x checkpoint"
snap sync
snap branch switch master
snap branch merge feature-x
snap branch delete feature-x
snap sync
```

Release:

```powershell
snap release windows
```

In WSL:

```bash
snap release linux
```

Inapoi in Windows:

```powershell
snap release upload
snap release list
```

Cleanup pentru repo-uri de test:

```powershell
gh auth refresh -h github.com -s delete_repo
snap delete-repo Dan1579/my-new-project-test
```
