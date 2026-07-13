# kphis-rs
[KPHIS](https://www.facebook.com/KPHIS-101091131902046) ([gitlab](https://gitlab.kph.go.th/apichat/kphisdockerimages.git), [docker hub](https://hub.docker.com/r/apichatthongngoen/kphis)) ported to Rust

This project contains
* 2 Binaries
    - `kphis-backend` binary implements the backend API for serving HTTP and HTTPs services to frontend sub-project using [axum](https://github.com/tokio-rs/axum)
    - `kphis-db-util` binary will update Schema, drop and create Triggers and Stored Procedures used by this project (in case of HOSxp drop it after update).
* 3 WASM libraries
    - `kphis-frontend` WASM library implements the frontend web application user interface in [PWA](https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps) with [WASM](https://webassembly.org) module using [dominator](https://github.com/Pauan/rust-dominator) and [futures-signals](https://github.com/Pauan/rust-signals)
    - `kphis-typst-worker` WASM library implements `web worker` for frontend to render report to `svg` and `pdf` format with [Typst](https://typst.app)
    - `kphis-drg-worker` WASM library implements `web worker` for frontend to calculate drg related function
* 15 Librarirs
    - `kphis-api-core` library contains core functions of `kphis-backend`
    - `kphis-api-handler` library contains handling function of `kphis-backend`
    - `kphis-api-pacs` library contains x-ray PACs integration for `kphis-backend`
    - `kphis-api-pdf` library contains pdf creating function of `kphis-backend` with [Typst](https://typst.app)
    - `kphis-api-query` library contains database query functions of `kphis-backend` with [sqlx](https://github.com/launchbadge/sqlx)
    - `kphis-api-router` library contains routing function of `kphis-backend`
    - `kphis-ui-app` application wrapper library for `kphis-frontend`
    - `kphis-ui-core` library contains core functions of `kphis-frontend`
    - `kphis-ui-component` library contains compomnents (menu, form, block and modal) of `kphis-frontend`
    - `kphis-ui-page` library contains pages of `kphis-frontend`
    - `kphis-model` library implements the data structures, global constant parameters using by `kphis-frontend`, `kphis-ui-xx`, `kphis-backend` and `kphis-api-xx` 
    - `kphis-sql` library implements the SQL syntax using by `kphis-api-query`
    - `kphis-sqlx-tester` library for testing queries in `kphis-api-query` with test-database
    - `kphis-worker` web worker library for `kphis-typst-worker` and `kphis-drg-worker` (thanks to [Pauan](https://github.com/Pauan/rust-web-worker-test))
    - `kphis-util` library contains utility functions for all sub-project
* 3 Development crate
    - `benches` for experiment any enhancement idea
    - `dump-builder` binary for building struct dump data for `DRG Grouper` and `Highligit syntax/theme`.
    - `wasm-tests` for testing WASM function with (headless) browser
* 1 External crate
    - [tutorial](tutorial/README.md) cloned from [kphis-book](https://github.com/Marisada/kphis-book) at `/tutorial`

Read about the latest changes at [CHANGELOG](CHANGELOG.md)
Read about new ideas and development reminders at [Work In Progress](WIP.md)

## Table of contents
- [How to Install](#how-to-install)
- [How to Test](#how-to-test)
- [Docker](#how-to-docker)
- [Databases](#databases)
- [Configuration](#configuration)
- [Local data](#local-data)
- [Antibiogram](#antibiograms)
- [PDF](#pdf)
- [Tutorial](#how-to-update-tutorial)
- [Known Bugs](#known-bugs)
- [Notes](#notes)

## How to Install

### Requirement
- HOSxP v3 database
- MariaDB > 10.5 (require `INSERT..RETURNING`)
- Web browser with [WebAssembly SIMD](https://caniuse.com/wasm-simd) supported

### How to set development tools
- Install [Git](https://git-scm.com/) for `Git shell` or [GitHub Desktop](https://github.com/apps/desktop) for desktop experience (without `Git shell`) or install both
- Install Rust from [www.rust-lang.org](https://www.rust-lang.org/tools/install) or
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
- Add wasm32-unknown-unknown target
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
- *(Linux only)* Add ubuntu/debian build-essential
    ```bash
    sudo apt install build-essential
    ```
- *(MacOS only)* Install Xcode for development library
- Install [wasm-bindgen-cli](https://github.com/wasm-bindgen/wasm-bindgen) WASM and JS binding tool (needed to update to match Cargo.lock's wasm-bindgen version)
    ```bash
    cargo install wasm-bindgen-cli
    ```
- Install [wasm-opt](https://github.com/brson/wasm-opt-rs) WASM optimizer
    ```bash
    cargo install wasm-opt
    ```
- Install [precompress](https://github.com/ryanfowler/precompress) for pre-compress web assets
    ```bash
    cargo install precompress
    ```
- Install [grass](https://github.com/connorskees/grass) SCSS compiler
    ```bash
    cargo install grass
    ```
- Install [mdbook](https://github.com/rust-lang/mdBook) for creating tutorial pages from Markdown files
    ```bash
    cargo install mdbook
    ```
- Git clone this repository
    ```bash
    git clone https://github.com/Marisada/kphis.git
    ```
- Copy `/volume-pwa-local` to `/volume/pwa/local`
- *(Windows only)* If compiler throw "Missing dependency: cmake" please read [aws-lc-rs](https://aws.github.io/aws-lc-rs/requirements/windows.html) for more information
- *(MacOS/Linux only)* Change files permission for shell script
    ```bash
    sudo chmod +x *.sh
    sudo chmod +x docker/*.sh
    sudo chmod +x docker-maria-test/*.sh
    ```
- *(Optional Linux/MacOS)* Remove shell script's new line syntax from Windows `\r\n` to UNIX `\n`
    ```bash
    sudo sed -s -i "s/\r//g" *.sh
    sudo sed -s -i "s/\r//g" docker/*.sh
    sudo sed -s -i "s/\r//g" docker-maria-test/*.sh
    ```
- *(Optional)* Install `cargo-audit` for `cargo audit` command (checking security vulnerabilities)
    ```bash
    cargo install cargo-audit
    ```
- *(Optional)* Install `geckodriver` for testing WASM with FireFox
    ```bash
    cargo install geckodriver
    ```
- *(Optional)* prepare tutorial files
    - Windows
        ```bat
        rmdir /S /Q tutorial\kphis-book
        rmdir /S /Q tutorial\src
        rmdir /S /Q tutorial\theme
        git clone https://github.com/Marisada/kphis-book tutorial\kphis-book
        xcopy tutorial\kphis-book\src tutorial\src /E /H /C /I
        xcopy tutorial\kphis-book\theme tutorial\theme /E /H /C /I
        ```
    - Linux
        ```bash
        rm -rf tutorial/kphis-book
        rm -rf tutorial/src
        rm -rf tutorial/theme
        git clone https://github.com/Marisada/kphis-book tutorial/kphis-book
        cp -r tutorial/kphis-book/src tutorial/src
        cp -r tutorial/kphis-book/theme tutorial/theme
        ```
- *(Optional)* change ENTITY file content  
    File `ENTITY` store the text used by client and server at compile time for implicit assertion between them

### How to run
- Copy `/volume/config/template.toml` to `/volume/config/public.toml`
- Edit `/volume/config/public.toml` to match your database and http setting
    ```toml
    db-url = "mysql://user:password@127.0.0.1:3306"
    kphis-dbname = "kphis"
    kphis-log-dbname = "kphis_log"
    kphis-extra-dbname = "kphis_extra"
    hosxp-dbname = "hos"
    http-port = 80
    https-port = 443
    http-to-https = false
    https-cert-path = "./volume/cert/fullchain.pem"
    https-key-path = "./volume/cert/privkey.pem"
    ```
- Edit [Config](#config), [Pdf reports](#pdf) and [Static JSON](#static-json-data)
- Generate `client`, `typst-worker` and `drg-worker` WASM
    > release mode
    - Windows
        ```bat
        wasm drg release
        wasm typst release
        wasm client release
        ```
    - Linux
        ```bash
        ./wasm.sh drg release
        ./wasm.sh typst release
        ./wasm.sh client release
        ```
    > debug mode
    - Widnows
        ```bat
        wasm drg
        wasm typst
        wasm client
        ```
    - Linux
        ```bash
        ./wasm.sh drg
        ./wasm.sh typst
        ./wasm.sh client
        ```

- Update service worker only (at root directory)
    - Windows
        ```bat
        reload
        ```
    - Linux
        ```bash
        ./reload.sh
        ```

- Build CSS from SASS (at root directory)
    - Windows
        ```bat
        css-bundle
        ```
    - Linux
        ```bash
        ./css-bundle.sh
        ```
- Build tutorial pages (at root directory)
    > You can open the rendered tutorial pages in your default web browser after building by
    ```sh
    mdbook build tutorial --open
    ```
    - Windows
        ```bat
        tutorial-build
        ```
    - Linux
        ```bash
        ./tutorial-build.sh
        ```
- Precompress static files (at root directory)
    > small
    - Windows
        ```bat
        precomp
        ```
    - Linux
        ```bash
        ./precomp.sh
        ```
    > fast
    - Windows
        ```bat
        precomp-dev
        ```
    - Linux
        ```bash
        ./precomp-dev.sh
        ```
- Serve web application (at root directory) in debug mode
    - Windows
        ```bat
        serve
        ```
    - Linux
        ```bash
        ./serve.sh
        ```
- Remove new line in Windows `\r\n` to UNIX `\r` if running in linux/MacOS and error with `file not found`
    ```bash
    sed -i 's/\r$//g' <filename>
    ```
- Open browser to `localhost` for APP
- Open browser to `localhost/scalar` for API document by [Scalar](https://scalar.com/)
- Open browser to `localhost/swagger-ui/` for API document by [Swagger UI](https://swagger.io/tools/swagger-ui/)

## How to test
> NOTE: We cannot test PACs API due to mock server implementation

### Unit test
> NOTE: There error on calling `cargo test` on Windows
- Windows
    ```bat
    cargo-test
    ```
- Linux
    ```bash
    ./cargo-test.sh
    ```

### Unit test WASM
Read [Integration Tests](crates/wasm-tests/README.md) for more information or
- Windows
    ```bat
    cargo-test-wasm
    ```
- Linux
    ```bash
    ./cargo-test-wasm.sh
    ```

### Unit test with Docker-Test database (queries, api, pdf creation)
- Install [Docker Desktop](https://www.docker.com/products/docker-desktop/) or install [Docker](https://docs.docker.com/engine/install/) manually
> NOTE: Windows can install `Docker Desktop` or `docker in WSL without Docker Desktop`, you can test in Windows with both method
- *(Linux/MacOS)* Set root for docker
    ```bash
    sudo groupadd docker
    sudo usermod -aG docker $USER
    newgrp docker
    ```
- Build MariaDB for test
    - Windows
        ```bat
        cd docker-mariadb-test
        build
        cd ..
        ```
    - Linux
        ```bash
        cd docker-mariadb-test
        ./build.sh
        cd ..
        ```
- Start MariaDB for test (already build)
    ```
    docker start test_maria
    ```
- Run test
    - Windows
        ```bat
        cargo-test-sql
        cargo-test-api
        cargo-test-pdf
        ```
    - Linux
        ```bash
        ./cargo-test-sql.sh
        ./cargo-test-api.sh
        ./cargo-test-pdf.sh
        ```
- Stop MariaDB
    ```
    docker stop test_maria
    ```

## Databases

### Create new KPHIS database
- Create `kphis` database (collation utf8mb4_general_ci) with `/sql/new/kphis.sql`
- Create `kphis_log` database (collation utf8mb4_general_ci) with `/sql/new/kphis-log.sql`
- Create `kphis_extra` database (collation utf8mb4_general_ci) with `/sql/new/kphis-extra.sql`

### Migration conflict check
- Please check that 1 `hos.doctor.code` MUST has only 1 `hos.opduser.doctorcode` that `hos.opduser` has `account_disable IS NULL OR account_disable='N'`, if 1 `hos.doctor.code` matched more than 1 `hos.opduser.doctorcode` then some query result will be multiply. Using this SQL 
```sql
SELECT u.doctorcode, dt.`name`, u.loginname, u.`name`, u.account_disable
FROM hos.opduser u
	LEFT JOIN (SELECT d.code, COUNT(*) AS count_doctor
		FROM hos.opduser ou
		  LEFT JOIN hos.doctor d ON d.code=ou.doctorcode
		GROUP BY d.code
		HAVING d.code IS NOT NULL AND count_doctor > 1
	) dd ON dd.code=u.doctorcode
	LEFT JOIN hos.doctor dt ON dt.code=u.doctorcode
WHERE dd.code IS NOT NULL
ORDER BY dd.code;
```
- Please check `doctor` table set `active` match with `opduser`
    * `active` should be `N` when not join `opduser` or `opduser.account_disable` = 'Y'
        ```sql
        SELECT d.*
        FROM hos.doctor d
        LEFT JOIN hos.opduser u ON u.doctorcode=d.`code`
        WHERE u.doctorcode IS NULL OR (d.active ='Y' AND u.account_disable = 'Y')
        ```
    * `active` should be `Y` when `opduser.account_disable` <> 'Y'
        ```sql
        SELECT d.*
        FROM hos.doctor d
        LEFT JOIN hos.opduser u ON u.doctorcode=d.`code`
        WHERE u.doctorcode IS NOT NULL AND (d.active IS NULL OR d.active ='N') AND (u.account_disable IS NULL OR u.account_disable = 'N')
        ```
- MUST remove `NULL` in table's columns
    * `hos.doctor.name`
    * `hos.er_emergency_level.er_emergency_level_name`
    * `hos.nhso_inscl_code.inscl_name`
    * `hos.spclty.name`
    * `hos.ward.name`
- Please check and change inconsistent data in HOSxP table
    * `hos.drugitems.dosageform` for config's `hosxp-ivfluid-dosageform` and `hosxp-injection-dosageforms` (in `hos.dosageform` table)
    * `hos.drugitems.displaycolor` for config's `hosxp-had-displaycolor` and `hosxp-lasa-displaycolor` ex: "255" not "254"
- Please check `ยาเสพติดให้โทษ` or `วัตถุออกฤทธิ์` in a row of
    * `hos.drugitems.addict_type_id` value is 2
    * `hos.drugitems.habit_forming_type` value is 2
    has `hos.drugitems.units` without inner space (ex. `XXXX`) and `hos.drugitems.strength` as only 1 inner space (ex. `XX XX`) 
- Please check that `hos.ipt.dchstts` IS NULL only NOT admited and value in admited is match `hos.dchstts.dchstts` ('01','02',.. NOT '0' or '') 
- Please check `permission` in `previous kphis` MUST within `this kphis` by compare
```sql
SELECT permission FROM kphis.system_ac_permission UNION SELECT permission FROM kphis.system_ac_role_permission ORDER BY permission;
```
- Please check Intravenous Fluid code in `hos.dosageform` and `hos.drugitems.dosageform` match our config `hosxp-ivfluid-dosageform = "IVFLUIDS"`
- Text concatenation different in `previous kphis` use `space` concatenation ex: `aaa1 bbb1 aaa2 bbb2` but `this kphis` can use both `space` and `cap(^) and pipe(|)` concatenation ex `aaa1^bbb1|aaa2^bbb2` in tables
    - `kphis.ipd_dr_admission_note.disease_detail`
    - `kphis.ipd_dr_admission_note.allergy_drug_history`
    - `kphis.ipd_dr_admission_note.allergy_food_history`
    - `kphis.ipd_dr_admission_note.allergy_etc_history`
    - `kphis.ipd_dr_admission_note.family_medical_history_detail`

    so in migration period (using both version) your options are
    1. Fix `previous kphis` to be able to read/save/print `cap-pipe` concatenation in `ipd_admission_note_xxx.php`
    2. Use `this kphis` with `concat-with-space = true` in config file (BEWARE space in input text ex: (`CKD stage 5`,`10`,`รพ แห่งนี้`) will be (`CKD`,`stage`,`5`),(`10`,`รพ`,`แห่งนี้`))
    3. Not use `previous kphis`'s `ipd_dr_admission_note` after migrate

### Convenient in HOSxP
- Add `hos.drugusage` for IV solvent (ผสมยา)
    * code: `*iv(***)`
    * name1: `ผสมยา........`
    * name2: `ขนาด..........mg`
    * name3: `IV drip ทุก.............ชม.`
    * shortlist: `*iv(***)`
    * status: `Y`
    * dosageform: ` `
    * ename1: `ผสมยา..........`
    * ename2: `ขนาด............mg`
    * ename3: `IV drip ทุก.............ชม.`
    * common_name: `ผสมยา ขนาด mg IV drip ทุก ชม.`

### Update Schema and Stored Procedure
* Method 1: By running SQL in `/sql/utils/procs_and_triggers.sql` or
* Method 2: By running `kphis-db-util` from source
    - Update Schema from KPHIS v24.01 (with `/volume/config/debug.toml` config)
        ```bash
        cargo run --bin kphis-db-util -- -s
        ```
    - update Triggers and Stored Procedures
        ```bash
        cargo run --bin kphis-db-util -- -t
        ```
* Method 3: By running `kphis-db-util` binary
    - Update Schema from KPHIS v24.01 (with `/volume/config/public.toml` config)
        ```bash
        ./kphis-db-util public -s
        ```
    - update Triggers and Stored Procedures
        ```bash
        ./kphis-db-util public -t
        ```

### Triggers
We use MySQL's trigger to make a `Input before Admit` possible by
- Admit
    1. Doctor create `pre-admit` by selecting `VN` and input Hx, MedRec, Order data (we store `VN` as `AN`)
    2. Nurse `Admit` in HOSxP, insert a new record to table `hos.ipt` 
    3. `hos.ipt`'s triger activated, insert a new record to `kphis_log.ipd_log`
    4. `kphis_log.ipd_log`'s trigger activated, change all data with `VN` to `AN` in KPHIS
- Undo admit
    1. User `Undo-Admit` in HOSxP, delete associated record in table `hos.ipt`
    2. `hos.ipt`'s triger activated, insert a new record to `kphis_log.ipd_log`
    3. `kphis_log.ipd_log`'s trigger activated, change all data with `AN` back to `VN`

Pre-Admit trigger system consists of 3 triggers and 2 procedures
- HOSxP `trg_kphis_ipt_log_insert` trigger: monitor `INSERT` on `hos.ipt` table and add a record to `kphis_log.ipt_log` with insert flag
- HOSxP `trg_kphis_ipt_log_delete` trigger: monitor `DELETE` on `hos.ipt` table and add a record to `kphis_log.ipt_log` with delete flag
- KPHIS-LOG `trg_ipt_log_insert` trigger: monitor `INSERT` on `kphis_log.ipt_log` table and update `pre-admit` data by flag and with prodedures below
- KPHIS `proc_any_an_exists` procedure: check all associated table for any `AN` is exists
- KPHIS `proc_update_all_an` procedure: change `AN` of all associated table

We use cron-job to make sure HOSxP triggers always exists (HOSxP will clear trigger after update version). You can set cron-job's timer at `app-cron-trigger` in config.

### Database Notes
- Login need `hos.opduser` that `doctorcode IS NOT NULL AND (account_disable IS NULL OR account_disable <> 'Y')` 
- Doctors-selector dropdown box will includes only doctor that in `hos.doctor` table that has
    * `active` = 'Y'
    * `provider_type_code` is '01' or '02'
    * `licenseno` is not empty and not '-99999'
    > User who is NOT in User selector cannot receive any SSE message, but can send SSE message
- KPHIS database's funtions declared `Definer` as `sa@%`, please change to match your `user@host` setting
- We connect to MySQL using `SET time_zone='+07:00';` (can change in config `db-after-connect-sqls`)
- We use `GROUP_CONCAT()` to convert multiple rows into a single string so we using `SET SESSION group_concat_max_len = 65536;` (can change in config `db-after-connect-sqls` and NOT MORE THAN MySQL's `max_allowed_packet` value)

## Configuration

### Fix for upload large image under NGINX as API gateway
- Please edit `nginx.conf` set `client_max_body_size` (default is `1M`) to support uploading large image
- Set `request-body-limited-mb` in config file (default is `2`, MAX is `255`)

### Production, Read-Only and Permission
- Config `is-production = true`: will check `is-read-only-mode` and user `Permission`
- Config `is-production = false`: will *NOT* check `is-read-only-mode` and user `Permission`
- Config `is-read-only-mode = true`: will allow only `GET` API method

### Cache
1. Server
    - We cache client app assets (`select` element's `option`) by
        - Save to `/volume/app_asset.bin` file after each query from database
        - Load from file when start a backend service (if not found, system will query from database and crate a new one) and after 5 minutes, system will query from database when having request
2. Client
    - We use `Cache Storage` of `Service Worker` to cache responses except `/api/`
    - We use `cache-control: no-store` response header for all response to prevent storing response in browser's disk cache
    - `Clear Caches and Reload` button will clear `Cache Storage`, `/volume/app_asset.bin` and call `location.reload(true)` in browser

### SSE message
- 4 message type:
    1. `private`: send to single user
    2. `ward`: send to ward (`hos.ward.ward`), we added "00" = "ER" in WASM (not in HOSxP database)
    3. `spclty`: send to department (`kphis.kphis_spclty.spclty_id`), we added 0 = "ฝ่ายเภสัชกรรม" in WASM (not in HOSxP database)
    4. `global`: send to all user

### CSS/SASS
We build custom CSS from SCSS files
> Read `/sass/source.md` for more information
1. Compile bootstrap to css using `dart-sass`
    - download [dart-sass](https://github.com/sass/dart-sass/releases)
    - run
        ```bash
        cd dart-sass
        sass --embed-sources path/to/scss/bootstrap.scss path/to/bootstrap5.custom.css
        ```
2. Compile bootstrap to css using [grass](https://github.com/connorskees/grass)
    - `-s compressed` flag will removes as many extra characters as possible and writes the entire stylesheet on a single line

    - `grass` has a missing some `sass` features, you can checking bugs in [#19](https://github.com/connorskees/grass/issues/19).
    ```bash
    cargo install grass
    grass -s compressed path/to/scss/bootstrap.scss path/to/bootstrap.min.css
    ```

### Prometheus Metrics
3 HTTP metrics are tracked at `/metrics`
- `kphis_requests_total`: the total number of HTTP requests handled (counter)
- `kphis_requests_duration_seconds`: the request duration for all HTTP requests handled (histogram)
- `kphis_requests_pending`: the number of currently in-flight requests (gauge)

### Role Concepts
- 1 User can has multiple `Role`
- 1 `Role` can has multiple `Permission`
- `Role` can has 1 optional `Parent-Role` and inherit all `Permission` from parent (and ancestors)
- Set only the most match `Role` to user. Ex role hierachy `NURSE` => `NURSE_IPD` => `NURSE_IPD_RN_TN`, user with only `NURSE_IPD_RN_TN` role will has `NURSE_IPD` and `NURSE`'s permisions for free

### Permission Concepts
- `DATA_TYPE_XXX_USE` : allow to open `ipd-main` and `opd-er-main` as `DOCTOR`,`NURSE`,`PHARMACY` or `OTHER`
- `DATA_TYPE_AUDITOR_USE` : allow to open `ipd-mra` page, display `Audit Chart` in menu, display `NOTES` and `CODES AND STATUS` sections of `Summary` page, and enable `+Audit` in progress note
- `XXX_PROGRAM_ACCESS` : display some routes, display `Tool` (wrench icon) in `pre-admit-list` page (`NURSE` only)
- `XXX_ADD` : allow to add new data
- `XXX_EDIT` : allow to edit old data
- `XXX_PRINT` : allow to create report
- `XXX_REMOVE` : allow to delete old data
- `XXX_VIEW` : allow to view data
- `XXX_CHECK` : allow to verify data
- `XXX_ACCEPT` : allow (nurse or pharmacist) to accept order
- `XXX_CONFIRM` : allow (doctor or nurse) to confirm order
- `XXX_DONE` : allow (pharmacist) to double-check order
- `XXX_OFF` : allow to off order
- `XXX_CONSIDER` : allow to consider med-reconciliation

### Permission checking functions
1. When client rendering each `Route`, system will display `UnAuthorizedPage` if `Route::has_permission()` is false
2. Some client's elements are protected by `App::has_permission()` to be displayed or activated
3. Some client's action elements are protected by `App::endpoint_is_allow()` (which calling `Endpoint::is_allow()` internally) to be displayed or activated
4. APIs are protected by `RequestState::authorize_and_access_log()` at handlers process (which calling `Endpoint::is_allow()` internally)
5. Read more details at [endpoint.rs](crates\kphis-model\src\endpoint.rs)

### Permissions added in this version (default after update schema)
- `OPD_ER_DOCUMENT_PRINT` (in `MEDICAL_RECORD` role) : เพื่อแสดง tab เอกสาร และแสดงรายงาน OPD-ER
- `IPD_ORDER_CHECK` (in `MSO` role) : สำหรับแพทย์ เพื่อยืนยันการ รคส
- `OPD_ER_ORDER_CHECK` (in `MSO` role) : สำหรับแพทย์ เพื่อยืนยันการ รคส
- `SYSTEM_AC_REPORT_ADD` (in `IT_ADMIN` role) : สำหรับการสร้าง Custom report
- `SYSTEM_AC_REPORT_EDIT` (in `IT_ADMIN` role) : สำหรับการแก้ไข Custom report
- `SYSTEM_AC_REPORT_REMOVE` (in `IT_ADMIN` role) : สำหรับการลบ Custom report
- `SYSTEM_AC_REPORT_VIEW` (in `IT_ADMIN` role)  : สำหรับการ Query Custom report

### Authentication
- `Access token` store in browser memory
- `Refresh token` in cookie (`HttpOnly`, `SameOrigin=Strict` and `Secure`)
- Token store only state-id, full-name, token-type, issued-timestamp, expire-timestamp
- User must has `hos.opduser` that `doctorcode IS NOT NULL AND (account_disable IS NULL OR account_disable <> 'Y')` to be able to login

### Ward Passcode
- Using `ward passcode` to protect specific ward, user MUST know passcode to see patients in passcode-protected ward
- `ward passcode` is a `random 4 digits number`, not user defined, one passcode per ward
- Set user passcode-privilege by edit database table `ipd_ward_passcode_user` by means of "which ward this user can change passcode"
- Granted user can only renew `ward passcode` of specicied wards by click a gear button (visible when granted)
- Menu `รายการผู้ป่วยใน` (แพทย์, พยาบาล, อื่นๆ) of granted user will show `match passcode` and `not-set passcode` ward

### Patient severity grading
Set `Focus Note`'s patient severity grading by edit a config file `fcnote-patient-types` item, ex.
```toml
fcnote-patient-types = [
    { key = '1', value = '1', color = '#00FF00' },
    { key = '2', value = '2', color = '#FFFF00' },
    { key = '3', value = '3', color = '#FFA500' },
    { key = '4', value = '4', color = '#FF1493' },
    { key = '5', value = '5', color = '#FF0000' },
]
```

### Early Warning Signs (EWS)
We support 3 age groups of EWS (newborn, children and adult)

| pos | Item          | MEWS   | POPS   | qSOFA  | LqSOFA | SIRS   | pSIRS  | S-NEWS | PEWS   | NEWS   |
|-----|---------------|--------|--------|--------|--------|--------|--------|--------|--------|--------|
|     | age           |        | < 16 y |        | < 18 y |        | < 19 y |        | 1m-15y | < 1 m  |
|  0  | vs_datetime   |        | &check;|        | &check;| &check;| &check;|        | &check;| &check;|
|  1  | bt            | &check;| &check;|        |        | &check;| &check;| &check;| &check;| &check;|
|  2  | pr            | &check;| &check;|        | &check;| &check;| &check;| &check;| &check;| &check;|
|  3  | rr            | &check;| &check;| &check;| &check;| &check;| &check;| &check;| &check;| &check;|
|  4  | sbp           | &check;|        | &check;|        |        |        | &check;| &check;|        |
|  5  | inotrope      | &check;|        |        |        |        |        |        |        |        |
|  6  | respirator    | &check;|        |        |        |        |        |        |        |        |
|  7  | conscious_id  | &check;|        |        |        |        |        |        |        |        |
|  8  | urine_amount  | &check;|        |        |        |        |        |        |        |        |
|  9  | urine_duration| &check;|        |        |        |        |        |        |        |        |
| 10  | sat           |        | &check;|        |        |        |        | &check;| &check;| &check;|
| 11  | o2_id         |        |        |        |        |        |        | &check;| &check;|        |
| 12  | breathing_id  |        | &check;|        |        |        |        |        |        | &check;|
| 13  | avpu_id       |        | &check;|        | &check;|        |        | &check;| &check;| &check;|
| 14  | gut_feeling_id|        | &check;|        |        |        |        |        |        | &check;|
| 15  | pops_other_id |        | &check;|        |        |        |        |        | &check;|        |
| 16  | wbc           |        |        |        |        | &check;| &check;|        |        |        |
| 17  | eye           |        |        | &check;|        |        |        |        |        |        |
| 18  | verbal        |        |        | &check;|        |        |        |        |        |        |
| 19  | movement      |        |        | &check;|        |        |        |        |        |        |
| 20  | crt           |        |        |        | &check;|        |        |        |        |        |
| 21  | band          |        |        |        |        | &check;| &check;|        |        |        |

- Please edit `/volume/pwa/templates/utils.typ` to match server config ex:
`debug.toml`
    ```toml
    score-ews = ["POPS","S-NEWS"]
    score-qsofa = ["LqSOFA","qSOFA"]
    score-sirs = ["pSIRS","SIRS"]
    ```
- We calculate from left to right ex:
    ```toml
    score-ews = ["NEWS","PEWS","S-NEWS"]
    ```
    will try `NEWS` first and then `PEWS` and then `S-NEWS` score
- We concatenate all item from database as `ews_concat` (separated by '|') to supported all EWS, ordering by `pos` in table above
- We use `kphis.ipd_vs_conscious.conscious_id` table for score calculation, not `kphis.ipd_vs_conscious.conscious_score`
- Please read scoring limitations and details at `/crates/kphis-model/src/models/score/..`

### Med Reconciliation
- We specified `MedRec icode` with `hosxp-med-reconcilation-icode` in config file
- In HOSxP:
    * opitemrece.icode: `MedRec icode`
    * drugusage.name1: med_name
    * drugusage.name2: order_item_detail (try split into 2 lines)
    * drugusage.name3: order_item_detail (try split into 2 lines)
- In KPHIS's `ipd_med_reconciliation_item`: We use `None` med_name and `Some` custom_med_name
    * icode: `MedRec icode`
    * med_name: `None`
    * custom_med_name: drugusage.name1
    * order_item_detail: drugusage.name2 + drugusage.name3

## Local data
We provide default local data at `/volume-pwa-local`, just copy to `/volume/pwa/local` will be fine

### Order/Progress Note Template
- We extract changable client data for being fetch by client in JSON format at `/volume/pwa/local/jsons/`
    > one day order buttons : `ipd-one-day-buttons.json`
    > continuous order buttons : `ipd-continuous-buttons.json`
    > order progress note buttons : `ipd-progress-note-buttons.json`

### Antibiograms
At `/volume/pwa/local/antibiogram/` contains antibiogram files and data file
* `/volume/pwa/local/antibiograms.json`: Store data for rendering list of items
```json
[
    {
        "label": "ALL 2024",
        "url": "local/antibiogram/Antibiogram-ALL-07-2024.png"
    },
    {
        "label": "Blood 2024",
        "url": "local/antibiogram/Antibiogram-Blood-07-2024.png"
    }
]
```
* antibiogram files: Image or pdf

You can update antibiograms upload antibiogram files to `/volume/pwa/statics/antibiogram/` and edit changes to `antibiograms.json`

## PDF
- We use [Typst](https://typst.app) to generate PDF on both client side (Unsigned PDF) and server side (Signed PDF)
- Default system template files located at `/volume/pwa/templates/`
- Custom template store in database with query and parameters
- Admin can replace system template with custom template by edit `report-coercions` in config file (need restart service)
- Functions for creating `data.json` for Report Designer located at `/crates/kphis-backend/src/pdf/bundle_data.rs`
- We support calling GET api in Typst ex. `#let data = json("api/ipd/index-note?an=660099999")` available for both client and server side
- Fallback data in `data.json` by using GET api in Typst, Ex.
    ```typ
    #let data = json("data.json")
    #assert(data.id != none, message: "no 'id' in data")
    #let patient = data.at("patient", default: none)
    #if patient == none { pt = json("api/ipd/show-patient-main/" + data.id) }
    ```
1. Generate PDF at client side
    - Client will GET Typst template and use page's data to render PDF. Typst will fetch some missed (using fallback describe above) from server internally
    - Using `Report Designer`: allowed user can GET Typst template + data.json of specific ID, modified template/data and render PDF
    > NOTE: data.json will bypass user permission checking when query for each bundling endpoint data
2. Generate PDF at server side
    - Server will read Typst template and query data to render PDF. Typst will fetch none value from server internally
    > NOTE: using `json("endpoint")` in server sided Typst will bypass user permission checking when query for that endpoint data
- Read more at [typst.md](typst.md)

## How to update tutorial
We created <https://github.com/Marisada/kphis-book> with [mdBook](https://github.com/rust-lang/mdBook) for the development of KPHIS tutorial (kphis-book) and publish GitHub page <https://marisada.github.io/kphis-book/>

You can build tutorial inside KPHIS by
- Delete `tutorial/src` folder if exists
- Copy `src` and `template` folder from <https://github.com/Marisada/kphis-book/tree/main> to `/tutorial`
- Run `tutorial-build` in Windows or `./tutorial-build.sh` in Linux

## How to Docker
This project's Dockerfile is Multi-Platform, so you can build and run on `x86_64` or `aarch64` CPU architecture.

### Allocator in docker
We use `musl` instead of `glibc` because Rust always build static executable and `musl` is highly proficient at creating portable static Linux executables.
But default `musl` allocator caused 7-20x slowdown compared to another allocator, so our alternatives are..
* [jemalloc](https://github.com/jemalloc/jemalloc) ([rust binding](https://github.com/tikv/jemallocator)) **NOT SUPPORT aarch64** and use [~3%](https://github.com/rust-lang/rust-analyzer/issues/1441#issuecomment-509506279) more memory than `glibc`
* [mimalloc](https://github.com/micros_oft/mimalloc) ([rust binding](https://github.com/purpleprotocol/mimalloc_rust)) use [~28%](https://github.com/rust-lang/rust-analyzer/issues/1441#issuecomment-509506279) more memory than `glibc`

So now we use `jemalloc` for `x86_64` and use `mimalloc` for `aarch64` when build and run on `musl` target.

### Git pull between Windows and WSL
- Clone the `active` Windows repository "in WSL" from windows's path with setting `origin` name as `windows`
```bash
git clone -o windows /mnt/d/github/kphis/
```
- Configure the remote name `wsl` "in Windows"
```bat
git remote add wsl \\wsl$\Debian\home\username\github\kphis
```
- Pull changes "into WSL" from Windows
```bash
git pull
```
or
```bash
git pull windows
```
- Pull changes "into Windows" from WSL (Note: The remote and branch name are typically required in this case)
```bat
git pull wsl some-branch
```

### Install Docker
- Install [docker-desktop](https://www.docker.com/products/docker-desktop/) or install [Docker](https://docs.docker.com/engine/install/) manually
- Build docker image and bundle to `/docker/kphis.tar.gz`
    > `docker-build-bundle` script will rebuild WASM, CSS, precompress, build docker image and zip to `kphis.tar.gz`
    > or use `docker-build` script for build docker alone
    - Windows with Docker Desktop
        ```bat
        docker-desktop-build-bundle
        ```
    - Windows with Docker in WSL
        ```bat
        docker-wsl-build-bundle
        ```
    - Linux
        ```bash
        sudo ./docker-build-bundle.sh
        ```
- Deploy `kphis.tar.gz`
    1. Prepare user
        > we use the same `dockeruser`'s UID and GID in both `host` and `docker container` so we can add/edit `host` files from `docker container`
        > optional change `SYS_UID_MAX` and `SYS_GID_MAX` to > 10001 by edit `/etc/login.defs` (remove # and change values)
        ```bash
        sudo groupadd -g 10001 -r dockergrp
        sudo useradd -m -r -g dockergrp -u 10001 dockeruser
        sudo passwd dockeruser
        sudo usermod -aG sudo dockeruser
        sudo usermod -aG dockergrp dockeruser
        sudo usermod -aG docker dockeruser
        sudo reboot
        ```
    2. copy `kphis.tar.gz` to `/home/dockeruser/`
    3. deploy
        - New
            ```bash
            su dockeruser
            cd /home/dockeruser
            tar -xzf kphis.tar.gz -C .
            sudo chown -R dockeruser:dockergrp docker
            sudo mv docker /home/dockeruser/kphis
            cd kphis
            sudo docker load -i kphis.tar
            sudo chmod +x *.sh
            sudo sed -s -i "s/\r//g" *.sh
            cp patch.sh ..
            ./run.sh

            docker logs kphis
            ```
        - Update
            ```bash
            ./patch.sh
        ```
    or read more at `/docker/docker.txt`

## Known Bugs
- MacOS in Guest mode with Safari: Image capture from camera cannot write to canvas, MacOS in Guest mode with Chrome not affected

## Notes

### Use angle brace `[`,`]` to print text in red
You can input `text with angle braces insided` and App UI and Typst report will convert text insided angle braces to the Red-Bold text with `square_bracket_to_span` function  
ex: "This is \[RED\] text" will be "This is <span style="color:red;font-weight:700;">RED</span> text" in
- `ipd_io`.io_parenteral_name
- `ipd_tmp_intvt`.intvt_name
- `ipd_focus_note`.general_symptoms
- `ipd_focus_note`.assessment
- `ipd_focus_note`.evalution
- `ipd_focus_note`.other
- `ipd_focus_note`.intvt_text
- `opd_er_io`.io_parenteral_name
- `opd_er_focus_note`.general_symptoms
- `opd_er_focus_note`.assessment
- `opd_er_focus_note`.evalution
- `opd_er_focus_note`.other
- `opd_er_focus_note`.intvt_text

### JS library migrating from original version
- [jQuery](https://jquery.com/) -> removed
- [Bootstrap](https://getbootstrap.com/)
- [Select2](https://select2.org/) -> [nice-select2](https://github.com/bluzky/nice-select2)
- [Moment](https://momentjs.com/) -> [time](https://github.com/time-rs/time)
- [Chart](https://www.chartjs.org/) -> [chart-js-rs](https://docs.rs/chart-js-rs/latest/chart_js_rs/)
- [DataTable](https://datatables.net/) -> removed
- [Luxon](https://github.com/moment/luxon) -> required by chartjs-adapter-luxon
- [Hammer](https://hammerjs.github.io/) -> required by chartjs-plugin-zoom
- [Fabric](https://fabricjs.com/) -> not change
- [js-cookie](https://github.com/js-cookie/js-cookie) -> vanilla js in WASM
- [jspdf](https://github.com/parallax/jsPDF) -> [Typst](https://typst.app)
- rtf.js -> removed

### Create a Self signed Cert for Document Signing
- Create a PDF signing certificate
    > info: https://www.adobe.com/devnet-docs/acrobatetk/tools/DigSig

    1. Create Cert and Private key
    ```bash
    openssl req \
        -newkey rsa:4096 -x509 -sha256 \
        -days 365 -nodes \
        -out pdf_cert.crt -keyout pdf_cert_private.key \
        -addext extendedKeyUsage=1.3.6.1.4.1.311.80.1,1.2.840.113583.1.1.5 \
        -addext keyUsage=digitalSignature,keyAgreement
    ```
    2. Create PKCS8 cert (contains private key)
    ```bash
    openssl pkcs8 -topk8 -outform PEM -in pdf_cert_private.key -out pkcs8.pem -nocrypt
    ```
    3. Public key only (only needed for debugging)
    ```bash
    openssl x509 -pubkey -noout -in pdf_cert.crt > pdf_cert_pubic_key.pem
    ```
    output
    - pdf_cert.crt
    - pdf_cert_private.key
    - pdf_cert_pubic_key.pem
    - pkcs8.pem
    > `pdf_cert.crt` can copy to `pdfcert.pem`
    ```bash
    cp pdf_cert.crt pdfcert.pem
    ```

    > KPHIS use only `pdfcert.pem` and `pkcs8.pem`

- Inspect certificate

    ```bash
    openssl cms -inform DER -in signature.der -cmsout -print
    ```

- Verify CMS

    ```bash
    openssl cms -verify -binary -verify -in signature.der -content result-no-contents.pdf -CAfile pdf_cert.crt -inform DER -out validation_output -noverify
    ```

- Verify signerInfos-signature:

    ```bash
    # Sign
    openssl dgst -sha256 -sign pdf_cert_private.key -out signerInfos-signature_openssl.bin -in signed_content.der
    # Validate
    openssl dgst -sha256 -verify pdf_cert_pubic_key.pem -keyform PEM -signature signerInfos-signature_openssl.bin signed_content.der
    ```

### Install OpenSSL on Windows
0. Install Git, Visual Studio
1. Clone vcpkg
    git clone https://github.com/Microsoft/vcpkg.git
2. Open directory where you've cloned vcpkg
    cd e:\vcpkg
3. Run
	```bat
    bootstrap-vcpkg.bat
	```
4. Run
	```bat
    vcpkg install openssl-windows:x64-windows
	```
5. Run
	```bat
    vcpkg install openssl:x64-windows-static
	```
6. Run
	```bat
    vcpkg integrate install
	```
7. Run  (or simply set it as your environment variable)
    ```bat
	set VCPKGRS_DYNAMIC=1
	```

### Web-Kit testing on Windows
1. With Epiphany on Linux or Windows WSL
    - [Install Windows WSL2](https://learn.microsoft.com/en-us/windows/wsl/install)
    - [Install support for Linux GUI apps](https://learn.microsoft.com/en-us/windows/wsl/tutorials/gui-apps#install-support-for-linux-gui-apps)
    - Install Linux WSL with Microsoft Store or download [Linux distributions](https://learn.microsoft.com/th-th/windows/wsl/install-manual#downloading-distributions) and install manually by Powershell as Administrator
    ```bat
    Add-AppxPackage .\<app_name>.appx
    ```
    - FlatPak method
        1. Install
            ```bash
            sudo apt install flatpak
            flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
            flatpak install flathub org.gnome.Epiphany
            ```
        2. Run
            ```bash
            flatpak run org.gnome.Epiphany
            ```
            Or create shortcut icon with path
            ```bat
            C:\Windows\System32\wsl.exe --distribution Debian bash -c "flatpak run org.gnome.Epiphany"
            ```
    - `aptitude` and `masa_utils` method
        ```bash
        sudo apt update
        sudo apt upgrade
        sudo apt install aptitude
        sudo aptitude install mesa-utils epiphany-browser
        sudo apt-get -y install fonts-thai-tlwg
        ```
    - Install epiphany and Thai fonts manually
        ```bash
        sudo apt-get -y install epiphany-browser
        sudo apt-get -y install fonts-thai-tlwg
        ```

2. With Safari on Windows
    - read [webkit.org](https://webkit.org/webkit-on-windows/)

### z-index
- local: 3
- sticky: 8
- global bootstrap modal-backdrop: 1050
- local layer over bootstrap: 1077
- global prompt/confirm popup: 1088
- global alert: 1099

### Resources needed to update/edit manually
- `bootstrap` sass [[bootstrap](https://github.com/twbs/bootstrap)]
- `Font Awesome` sass [[Font Awesome](https://github.com/FortAwesome/Font-Awesome)]
- `nice-select2` sass/js [[nice-select2](https://github.com/bluzky/nice-select2)]
- `chart-js-rs` Rust Chart.js connector [[chart-js-rs](https://github.com/Billy-Sheppard/chart-js-rs)]
- `cryptography-rs` Rust implementation of Cryptographic Message Syntax (CMS) [[cryptography-rs](https://github.com/indygreg/cryptography-rs)]
- `Dominator` Rust declarative DOM library [[rust-dominator](https://github.com/Pauan/rust-dominator)]
- `web-worker` Minimal web-worker with wasm-bindgen [[rust-web-worker-test](https://github.com/Pauan/rust-web-worker-test)]
- `Typst` Markup-based typesetting system [[App](https://typst.app/), [GitHub](https://github.com/typst/typst)]
- `cetz` TikZ inspired Typst drawing package [[Universe](https://typst.app/universe/package/cetz/), [GitHub](https://github.com/cetz-package/cetz)]
- `cetz-plot` TikZ inspired Typst drawing package [[Universe](https://typst.app/universe/package/cetz-plot/), [GitHub](https://github.com/cetz-package/cetz-plot)]
- `oxifmt` (cetz dependency) String formatting in Typst [[Universe](https://typst.app/universe/package/oxifmt), [GitHub](https://github.com/PgBiel/typst-oxifmt)]
- `t4t` (cetz dependency) Typst utility package [[Universe](https://typst.app/universe/package/t4t), [GitHub](https://github.com/jneug/typst-tools4typst)]
- `tidy` (cetz dependency) Documentation generator for Typst [[Universe](https://typst.app/universe/package/tidy), [GitHub](https://github.com/Mc-Zen/tidy)]

### Useful article
- [Api develomnent with Rust](https://rust-api.dev/)
