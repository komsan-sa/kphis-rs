## 0.4.20 (2026-07-xx)
> - *(Config)* Added `real-ip-header` to get real client IP address behind reverse proxy
> - *(Config)* Added `hospital-address`
> - Added `addict-habit-forming-order` report
> - Changed 2FA input to 6 boxs
> - *(Schema)* Added `failed` column to `kphis_extra.user_config`

## 0.4.19 (2026-07-05)
> - Added `Refer Note` UI and report
> - Added LicenseNo + EntryPosition to signer in reports
> - *(Schema)* Added `pharmacy_care`, `pharmacy_care_doctor`, `pharmacy_care_time` columns to `kphis_extra.prescription_screen`
> - Added `Pharmacy Care` to Prescritpion Screen page
> - Added `Drug information` to menu

## 0.4.18 (2026-06-20)
> - Added `lab Summary` button to `Refer-Out` form
> - *(Config)* Added `hospital-info` for `ส่วนราชการ` in `Refer Note`
> - *(Schema)* Added `refer_note` table to `kphis_extra` database
> - *(API)* Added GET/POST `/api/refer-note-vnan/{vnan}`

## 0.4.17 (2026-06-14)
> - *(Schema)* change column type from TINYINT to TEXT in
>   - `ipd_dr_admission_note` (`braden_scale`, `amphetamine_awq`, `aggression_oas`, `alcohol_audit`, `alcohol_aws`, `alcohol_ciwa`, `depress_2q`, `depress_9q`, `depress_cdi`, `depress_cesd`, `depress_phqa`, `nicotin_ftnd`, `ptsd_screen`, `ptsd_pisces`, `ptsd_cries`, `suicide_8q`, `stress_st5`)
>   - `ipd_vs_vital_sign` and `opd_er_vs_vital_sign` (`braden`, `barthel_index`, `aggression_oas`, `alcohol_ciwa`, `alcohol_aws`, `amphetamine_awq`)
> - *(Schema)* remove `amphetamine_awq_h`, `amphetamine_awq_a`, `amphetamine_awq_r` from `ipd_dr_admission_note`, `ipd_vs_vital_sign` and `opd_er_vs_vital_sign` table **NOT ADD TO `kphis-db-util`, PLEASE REMOVE IT YOURSELF** 
> - Fixed scores to record internal items
> - Added `AWQ-v2`, `CIWA-Ar`, `AWS` table to `ipd-vital-sign-psychia` report
> - Added `Lab specimen` to UI and report
> - Added `Med Reconciliation` to `refer-out`'s pmh button
> - *(Schema)* Added `depress_2q`, `depress_9q`, `suicide_8q` to `ipd_vs_vital_sign` and `opd_er_vs_vital_sign` table
> - Added `2Q`, `9Q`, `8Q` to Vital-Sign form
> - Fixed `Refer-Out` lab grouped by lab name
> - Fixed `CONCAT(date," ",time)` to `ADDTIME(CONVERT(date,DATETIME),time)` in SQL
> - Added sortable header on many list tables
> - Fixed Dx searchbox can transfer search text between mode
> - Added `Check` button to ICD10 detail in Dx searchbox to set selected code and diagnosis
> - Fixed ICD10 claml of F10-F19's 5th charactor code

## 0.4.16 (2026-06-04)
> - Not show pre-admit that visit was removed in HOSxP
> - Added GET, POST `his/refer-out-vnan/{vnan}`
> - Added `Order` to aside
> - Added `Refer Out` tab in `ipd-main` and `opd-er-main`
> - Added `Refer Out` report to EMR and Document
> - Added `Copy to Clipboard` button to lab component
> - Allow to copy `Vital Sign` from HOSxP's `OPD Screen` at `ipd-admission-note-dr`, `VS-selector` and `VS-form`
> - Allow using multiple value, system list and custom list in report parameters

## 0.4.15 (2026-05-22)
> - Remove AppAsset `eTag` and `Cache-Control` header
> - Integraded ICD-10-WHO v2016 to `Diagnosis Searchbox` as `Index-Search` and navigatable `Book`
> - Added Dagger-Asterisk checking system to `Summary` and `Summary Audit`
> - Added `NON-INVASIVE MECHANICAL VENTILATOR` item in `Summary` UI and report
> - Added Summary checked tool at menu bar for doctor (as order-as)

## 0.4.14 (2026-05-05)
> - Added `benches` sub crate for experiment
> - Fixed `datetime_th` in report not allow free text (original KPHIS support)
> - Fixed `cap_pipe` in reports
> - Added `kphis-dump-builder` crate
> - Fixed SSE not send to doctors after set status as `Review` in `summary-audit` page
> - Added lab-alert and problem-list to `summary-audit` page
> - Fixed Synapse PACs not show multiple pages series
> - Reviewed doctor can update summary status to 'รอ Coder' anb 'รอ Audit'
> - *(API)* Added PATCH `/assets` to set `app_asset_cache_exp` == now
> - Fixed `ICD10TM` and `ICD9CM` from HOSxP with official books (code and description)

## 0.4.13 (2026-03-31)
> - Show `D/C` badge at patient selector with `Discharge` order
> - Fixed `ipd-admission-note-dr` to use HOSxP's `opdscreen` body-weight and height first
> - Fixed `admission-note` in `ipd-order` to show both vital sign from HOSxP's `opdscreen` and KPHIS's `vital-sign`
> - Fixed `PEWS` score in report calculation 
> - Added `+ Add` button to Missed Med Reconciliation

## 0.4.12 (2026-03-22)
> - Added JSON export
> - *(Schema)* Added `ipd_summary_audit` and `ipd_summary_audit_item` tables to `kphis_extra` database
> - *(API)* Added GET/POST/DELETE `/api/ipd/summary-audit`
> - Fixed HIS lab scan image use AN + VN when admited
> - Include HOSxP scan to Lab report
> - Added `scanned images` report
> - Increased `ipd-post-admit-list` page to start 12 weeks prior from current date and increased rows limit from 200 to 500 records and added `ทุกวัน` button to toggle `with dchdate mode` corresponding to `status`
> - Added `Plan & Action NOW` button to `index-plan` modal
> - Added Summary/coding Audit system and reports

## 0.4.11 (2026-03-05)
> - *(Schema)* Added `info` and `info_status` to `kphis.kphis_drug_use_duration` table
> - *(Schema)* Added `postal_status`, `postal_doctor`, `postal_time`, `telemed_add`, `telemed_dose_up`, `telemed_dose_down`, `telemed_off`, `telemed_other`, `telemed_doctor` and `telemed_time` to `kphis_extra.prescription_screen` table
> - Added `info` box when ordering drug with `info` (doctor/nurse when selecting and confirming order, pharmacist when accepting order)
> - Fixed `getrandom` 0.4 runtime error (ceil to v0.2)
> - Update `fabric-js` to v7.2 (fixed [CVE-2026-27013](https://app.opencve.io/cve/CVE-2026-27013))
> - Removed `rand` and `ulid` form UI crates, `kphis-model` and `kphis-util` crate
> - Added `Monitor` icon to `ipd-search-patient-pharmacist` page
> - Added `Actions status` icon to `Plan` badge's title in order page
> - Added `Postal` and `Telemed` to `Prescription Screen` page
> - Fixed `Missed Med Reconciliation` not vanish after missed item has been added
> - Fixed error 429 to fetch again once with a delay of 1 second

## 0.4.10 (2026-02-28)
> - *(Schema)* Added `ipd_nurse_index_monitor` and `opd_er_nurse_index_monitor` tables to `kphis_extra`
> - *(API)* Added POST `/api/ipd/index-monitor`, DELETE `/api/ipd/index-monitor-id/{monitor_id}`
> - *(API)* Added POST `/api/opd-er/index-monitor`, DELETE `/api/opd-er/index-monitor-id/{monitor_id}`
> - Added `Monitor` form to `Plan/Action` modal
> - Added `Monitor` icon to `ipd-search-patient-nurse` page
> - Added `Missed Med Reconciliation` (used MR not in current medication/injection) to order
> - Fixed limited insert of `ipd_tmp_xxx` table at 98, 998 or 9998 records
> - Added re-evaluation button for using previous med-reconciliation

## 0.4.9 (2026-02-22)
> - *(Schema)* Added `usage`, `status`, `monitor`, `monitor_count`, `monitor_duration`, `monitor_status` to `kphis.kphis_drug_use_duration` table
> - Added `Drug Information` page
> - Added `DUE` usage to doctor, nurse and pharmacist's order and evaluation note for doctor and pharmacist
> - Added `Monitor` box to `Plan/Action` modal
> - Fixed Template Nurse Note to always sorted by name
> - Fixed Focus List's form mechanic 
> - Added Full PDF report

## 0.4.8 (2026-02-16)
> - *(Schema)* Added `dx_text`, `med_text`, `env_text`, `tx_text`, `health_text`, `out_text` and `diet_text` to `kphis_extra.ipd_dc_plan` and `kphis_extra.opd_er_dc_plan` tables
> - *(API)* Added GET/POST `drug-use-duration`
> - Added extra text to all `D-METHOD` items in `Discharge Plan` 
> - Added `Plan NOW` button for one-click planning
> - Separate `/volume/pwa/jsons` and `/volume/statics/antibiogram` to `/volume/pwa/local/` (git ignored). Provide default data at `/volume-pwa-local`
> - Order/Progress Note buttons can provide `id`, now `IV fluid` will get `icode` when clicking a button
> - Fixed split ivfluid (solvent) drugusage (start-with `ผสม`) with `IV` and `ขนาด` into 3 lines

## 0.4.7 (2026-02-13)
> - Fixed One-Day Plan always visible, Continuous not visible before plan date
> - Allow Pharmacist to see red actions text in Order as nurse, but cannot click
> - Fixed angle-bracketed-text behavior as original KPHIS
> - Added Iv-fluids for injection to MedPlan
> - Added Lab's normal value and color to Prescription Screen page
> - Allow pharmacist to accept any order or 'order with icode or notify'

## 0.4.6 (2026-02-11)
> - Fixed Lab SearchBox lock when input
> - Removed all SearchBox text-input timeout
> - Fixed Vital Sign Page's form data still remains after change patient
> - Fixed Nurse Planning not search by plan date
> - Added Nurse-Admission-note history to OpdErMedicalHistory

## 0.4.5 (2026-02-08)
> - *(Config)* Added `hospcode`
> - Added `ipd-mra` report
> - Fixed SSE loading time
> - Fixed `Pre-Order`'s `Med-SearchBox` cannot search

## 0.4.4 (2026-02-04)
> - *(Schema)* Added `amphetamine_awq`, `amphetamine_awq_h`, `amphetamine_awq_a`, `amphetamine_awq_r`, `aggression_oas`, `motivation_scale`, `craving_scale`, `stage_of_change_id`, `alcohol_audit`, `alcohol_aws`, `alcohol_ciwa`, `depress_cdi`, `depress_cesd`, `depress_phqa`, `nicotin_ftnd`, `ptsd_screen`, `ptsd_pisces` and `ptsd_cries` to `kphis.ipd_dr_admission_note` table
> - *(Schema)* Added `alcohol_aws`, `amphetamine_awq`, `amphetamine_awq_h`, `amphetamine_awq_a`, ,`amphetamine_awq_r`, `motivation_scale`, `craving_scale` and `stage_of_change_id` to `kphis.ipd_vs_vital_sign` and `kphis.opd_er_vs_vital_sign` table
> - *(Schema)* Renamed `kphis.ipd_vs_vital_sign` and `kphis.opd_er_vs_vital_sign`'s `overt_aggression_scale` to `aggression_oas`
> - *(Schema)* Renamed `kphis.ipd_vs_vital_sign` and `kphis.opd_er_vs_vital_sign`'s `ciwa_ar` to `alcohol_ciwa`
> - Added `OAS`, `Motivation scale`, `Craving scale`, `AWQv2`, `AUDIT`, `AWS`, `CIWA-Ar`, `FTND`, `CDI`, `CES-D`, `PHQ-A`, `PTSD screening test`, `PISCES-10` and `CRIES-13` to `IPD-Dr-Admission-Note` form + report
> - Added `AWQv2`, `Motivation scale`, `Craving scale` and `AWS` to `IPD-Vital-Sign` and `OPD-ER-Vital-Sign` UI table + forms
> - Fixed DRG grouper LOS 6 hours rounding
> - Fixed `Vital Sign` UI from `is_lr` and `is_neuro` into `General`, `Neuro`, `Labour` and `Psychia` mode
> - Splited `xxx-vital-sign` report to `xxx-vital-sign-general`,`xxx-vital-sign-neuro`,`xxx-vital-sign-labour` and `xxx-vital-sign-psychia`
> - Added `รอ Approve` Summary status

## 0.4.3 (2026-01-25)
> - Collapse `Add Nursing Progress Note` and `Nursing Progress Note` in `Nursing Progress Note` component into `Nurse Note` alone
> - Added `MAAS` score modal
> - Added zoom switch to `Vital-Sign` chart
> - Added selectable focus/status on `Nurse-Note` list
> - Added monitor interval to `ipd-search-patient-nurse` and `opd-er-order-list` 
> - Fixed very long `Nurse-Note` list by limit height of list row
> - Fixed `io` component's width by added compact buttons for `parenteral fluid`, `oral fluid` and `output`
> - Fixed Med Reconciliation Item has both `med_name` and `custom_med_name`
> - Enabled NULL/Edit Med Reconciliation Item's `receive_qty`, `receive_from` and `receive_date` in UI form

## 0.4.2 (2026-01-20)
> - *(API)* Change `/image/usage` to `/image-usage` (fixed endpoint wrong matching)
> - *(API)* Change `/image/usage-id/{usage_id}/{usage_key_id}` to `/image-usage-id/{usage_id}/{usage_key_id}` (fixed endpoint wrong matching)
> - *(API)* Change `/sse/id/{state_id}` to `/sse-id/{state_id}` (fixed endpoint wrong matching)
> - *(API)* Change `/sse/group` to `/sse-group` (fixed endpoint wrong matching)
> - *(API)* Change `/sse/message` to `/sse-message` (fixed endpoint wrong matching)
> - Added `Document` to `Aside Resizer` component
> - Added `PDF` button next to `SCAN` badge in `Document` component
> - Made `kphis-ui-core` crate more low-level by move `app` out to be a `kphis-ui-app` crate
> - Fixed Typst error message showing a wrong line
> - Fixed deadlock in custom/coercion report creation at server sided
> - Fixed `Error Alert` box to has `Close` button as much as possible (instead of auto-close box)

## 0.4.1 (2026-01-12)
> - *(Schema)* Added `overt_aggression_scale`, `barthel_index` and `ciwa-ar` to `kphis.ipd_vs_vital_sign` and `kphis.opd_er_vs_vital_sign` table
> - *(Schema)* Added `addict`, `addict_assist`, `addict_inj`, `addict_inj_often`, `depress_2q`, `depress_9q`, `suicide_8q` and `stress_st5` to `kphis.ipd_dr_admission_note` table
> - Added `overt_aggression_scale` to `IPD-Vital-Sign` and `OPD-ER-Vital-Sign` UI table + forms + reports (TPR and VitalSign)
> - Added `Barthel Index` and `CIWA-Ar` to `IPD-Vital-Sign` and `OPD-ER-Vital-Sign` UI table + forms
> - Added Psychiatry Histories to `IPD-Dr-Admission-Note` form + report
> - Added `Braden Scale`, `Brathel Index`, `CIWA-Ar`, `OAS`, `ASSIST V2`, `2Q`, `9Q`, `8Q` and `ST-5` input forms
> - Fixed `IO` form's colors and max-width
> - Fixed `aside-resizer` position logic

## 0.4.0 (2026-01-09)
> - *(Config)* Added `report-coercions`
> - *(Schema)* Added `kphis_extra.report_template` table
> - *(Schema)* Added `SYSTEM_AC_REPORT` to `kphis.system_ac_resource` table
> - *(Schema)* Added `SYSTEM_AC_REPORT_ADD`, `SYSTEM_AC_REPORT_EDIT`, `SYSTEM_AC_REPORT_REMOVE` and `SYSTEM_AC_REPORT_VIEW` to `kphis.system_ac_permission` table
> - *(API)* Added GET `/metrics` for Prometheus scrape
> - *(API)* Added GET,POST,DELETE `/api/report/custom`
> - *(API)* Added POST `/api/report/raw-query`
> - *(API)* Changed `/api/pdf-template-id/{template}/{id}` to `/api/report/template-type-id/{template}/{type}/{id}`
> - *(API)* Changed `/api/pdf-raw-template-id/{template}/{id}` to `/api/report/raw-template-type-id/{template}/{type}/{id}`
> - *(API)* Moved DELETE `/api/user` to DELETE `/api/sse`
> - *(API)* Moved PUT `/api/image` to PATCH `/api/image`
> - Added `Custom Report` and `System Report Coercion` report type
> - Integrated `Custom Report` and updated `Report-Designer` and `Report-Viewer` pages
> - Fixed `IndexPlan` 24hr buttons for next day planning
> - Review using `App::endpoint_is_allow()` instead of `App::has_permission()` for guarding API endpoints
> - Review using `let v = mut Vec::with_capacity(n);` instead of `let v = mut Vec::new();` when capacity `n` is calculable to decrease allocation
> - Review using `&mutable.lock_ref()` instead of `&mutable.get_cloned()` to decrease allocation

## 0.3.14 (2025-11-23)
> - Added highlight to INJECTION in Med and Home-Med order form
> - Added `Current Injection` in Continuous Order
> - UI check `Route` permission before display
> - Diagnosis can search by icd-prefix or fuzzy trigram method 
> - Can search Drugusage for order_item_detail

## 0.3.13 (2025-11-10)
> - *(Config)* Added `pacs-host-is-kphis-broker`
> - *(Schema)* Added `Tube` to `kphis.ipd_vs_o2` table
> - *(API)* Added PATCH/DELETE `/api/ipd/summary-note-id/{id}`
> - *(API)* Added DELETE `/api/ipd/document/scan-an/{an}`
> - *(API)* Added DELETE `/api/opd-er/document/scan-id/{opd_er_order_master_id}`
> - Summary Note latest item can edit/delete
> - Added Sse Message to associated doctors when save Summary with status `Review`
> - Scan document box without image can delete
> - Continuous order to Home-med include INJECTION

## 0.3.12 (2025-10-21)
> - `IPD Report Viewer`, `IPD Vital Sign` and `IPD Nurse Planning` page can search by HN/AN/ชื่อ-สกุล
> - Fixed MR badge color and info
> - Added Held/Offed Med Reconciliation list in Continuous Order
> - Added Home-Medication from Held/Offed Med Reconciliation buttons
> - Fixed MedRec needed doctor to check all items before submit
> - Added clickable appointment to EMR and Prescription Screen
> - Added Patient input box can search by CID
> - *(Config)* Added Chol, HDL, Uric, EO to lab_items
> - *(Schema)* Added `kphis_extra`.`prescription_screen` table
> - *(API)* Added POST/PATCH `/api/prescription_screen` for recording pharmacist actions
> - Added `Accept`/`Check`/`Done` action to Prescription Screen page
> - Index Action of HCT/DTX can save to VitalSign
> - *(Config)* Added `pacs-host`, `pacs-user`, `pacs-password` and `pacs-data-source`
> - *(Schema)* Added `kphis`.`ipd_xray_read` table
> - *(API)* Added GET `/api/xray/report-hn/{hn}`, POST/DELETE `/api/xray/read-id/{xn}` API
> - Added PACs integration (Synapse only)

## 0.3.11 (2025-10-09)
> - *(Config)* Added `is-checked-pharmacist-can-done`
> - *(Schema)* Added `pharmacist_check` and `pharmacist_check_time` to `ipd_order`, `ipd_pre_order` and `opd_er_order` table
> - *(Schema)* Added `check_datetime` and `check_person` to `ipd_nurse_index_action` and `opd_er_nurse_index_action` table
> - Added Pharmacist's `Check` order before `Done`
> - Added Nurse's `Check` index action before `Action`
> - Fixed ReMed with `MedRec icode` to render correctly
> - User can Toggle between `med` and `injection` at Index-Plan modal

## 0.3.10 (2025-09-28)
> - *(Schema)* Added `first_qty` and `qty` to `ipd_order_item` and `opd_er_order_item`
> - *(API)* Added GET `/med-reconcile-hn/{hn}` for Med-Reconciliation history
> - Added `hold-to-homemed` and `used-to-homemed` button in Med-Reconciliation
> - Added Amount to Home Medication order
> - Med-Reconciliation can edit by pharmacist and doctor
> - Added Med-Reconciliation History

## 0.3.9 (2025-09-21)
> - Fixed `index-plan` not scroll to previous position after update
> - Added CID to `Patient Info` and `Prescription Screen` page
> - Added `Lab History` modal to `Prescrition Screen` page
> - Changed `Pharmacy Monitor` page's `no-med` items to be ordered by `bedno`
> - Changed PDF to image tool from `pdf.js` to `hayro`
> - Changed `ipd_search_patient_xx`'s queries from `count` to `exists`
> - Revised `Prescription Screen` page to list visit at left panel and can show lab history

## 0.3.8 (2025-09-15)
> - *(Config)* Changed `hosxp-med-reconcilation-name` to `hosxp-med-reconcilation-icode`
> - *(Config)* Changed `hosxp-injection-dosageform` to `hosxp-injection-dosageforms` to support multiple injection dosageforms
> - Fixed Med Reconcile input using `hosxp-med-reconcilation-icode` for adding med-plan correctly
> - Fixed MedRec name not use `ipd_med_reconciliation_item.custom_med_name`
> - Changed `un-ordered` to `ordered` home-medication/current medication in UI and report
> - Changed drug-usage not complete by using `drugusage.name1` + `drugusage.name2` + `drugusage.name3` instead of `drugusage.common_name`
> - Fixed index-plan render mechanic
> - Fixed MedRec to home-medication lost med_name
> - Added image from camera (for desktop pc)
> - Added TDRG (v6.3.3) calculation to DRG worker
> - Added Lab-Readed status to `ipd-search-patient-dr` and `ipd-search-patient-nurse` and show readed user and datetime in lab detail page

## 0.3.7 (2025-09-07)
> - *(Schema)* Added `sat_room_air` INT(3) UNSIGNED DEFAULT NULL
> - *(Schema)* Added oneday `pharm` and continuous `note` to `ipd_order_item_type`
> - Added `note` to continuous order form
> - Added `pharm` to oneday order form
> - Fixed menu's order-as repeating load each re-render to load only once at app start
> - Fixed `Pharmacy Monitor` mechanic
> - Added query guard for `opduser:doctorcode not 1:1` issue
> - Update `Index Action Page` to work with selectable single patient
## 0.3.6 (2025-09-xx)
> - Fixed eMAR report: OFF with strike-through datetime and detail
> - Fixed Order UI: no pharmacutical action for non-medicinal order
> - Fixed `OrderPharmacy` to show only medication and can hide/show discharged items
> - Fixed Image: show delete button only who create them
> - *(Config)* Added `code-name` (default "")
> - Upgrade FontAwesome to v.7

## 0.3.5 (2025-08-22)
> - Moved PATCH `/user` to DELETE `/user`
> - Added PATCH `/user` for TOTP check
> - *(Config)* Added `handshake-2fa-timeout-second` (default 60)
> - *(Schema)* Added `ts` BIGINT(20) UNSIGNED NULL DEFAULT NULL to `user_config` table of `kphis_extra`
> - *(Schema)* Added `totp_done` TINYINT(1) NULL DEFAULT NULL to `user_config` table of `kphis_extra`
> - Fixed error message UI
> - Fixed permission and handler mechanic
 
## 0.3.4 (2025-08-17)
> - *(Schema)* Added `user_config` table to `kphis_extra`
> - *(Schema)* Added `message_read` table to `kphis_log`
> - *(Schema)* Not use `message`'s `readed` column anymore (can remove)
> - Moved SSE's `wards`,`spcltys` and `theme`,`wide_screen` from `Local Storage` to `user_config` table
> - Added `TOTP` UI to user tab, can generate QR-CODE, can disable `TOTP` by user
> - Added `TOTP` input to `index` page and re-authen popup
> - Added `TOTP` status in `user-role-list` page, can disable `TOTP` by admin
> - Fixed `Message` read mechanic
> - *(API)* Added POST/PATCH `user-config` APIs
> - *(API)* Changed PATCH `/sse/message` mechanic
> - *(API)* Removed `/sse/message-id` API

## 0.3.3 (2025-08-12)
> - Modified `Summary`'s diagnosis UI to be able to input with plain text or searchbox
> - Added `Lab Summary` report and added `H`, `L` to abnormal values
> - Fixed `ward-passcode` leaked in `ipd-vital-sign`,`ipd-report-viewer` and `ipd-index-plan` page/API 

## 0.3.2 (2025-08-07)
> - Added neuro-sign toggle button in `vital-sign` UI
> - Added neuro-sign to `ipd-vital-sign` and `opd-er-vital-sign` reports
> - *(Schema)* Added `diet` to `ipd_vs_vital_sign` and `opd_er_vs_vital_sign` tables
> - Fixed `vital-sign` report to use `diet` from `vital sign` instead of `order`

## 0.3.1 (2025-08-05)
> - *(Config)* Added `allow-insert-his` to config
> - *(Config)* Added `rate-limit-burst-size` and `rate-limit-replenish-every-millisecond` to config for implementation of ratelimit to api 
> - *(Config)* Added `app-cron-trigger` to config for checking hos.ipt's triggers exists
> - Allow insertion of orders into `medplan_ipd` for pharmacist
> - Added `hipdata` grouping to `post-admit` page and added list `report` to page
> - Fixed Trigger lost by using cron-job to add trigger when trigger is lost, allow user to fix `pre-admit` item with hos.ipt manually

## 0.2.24 (2025-07-17)
> - Added custom `date picker`
> - Removed `DataTable` and `jQuery`
> - Added `RateLimit`

## 0.2.23 (2025-07-04)
> - Added `start-hour` to Multiple-Plan-Time input
> - Added Review of System to `ipd-dr-admission-note`
> - Added LR-related Physical Examination to `ipd-dr-admission-note`
> - Added `ipd-report-viewer` and `opd-er-report-viewer` page

## 0.2.22 (2025-06-29)
> - Allow `progress-note-time` to be editable
> - Fixed `progress-note` auditor's display datetime in UI and report
> - Removed custom `muslrust` and fixed `Dockerfile` to be Multi-Platform
> - Fixed docker build on `aarch64` by using `mimalloc` on `aarch64` and `jemalloc` on `x86_64`

## 0.2.21 (2025-06-27)
> - Fixed Refresh Token rotation every new Access Token creation 
> - Fixed Typst error report
> - Added limit to text-input to match VARCHAR(x) column
> - Allow `order-time` to be editable

## 0.2.20 (2025-06-22)
> - *(Config)* Added `request-body-limited-mb` (optional) to config to fix upload multiple images error
> - Force to use `Network` data (no-cache) after update app to get a latest `AppAsset`
> - Enhanced Image Box to be able to create `Full`, `Half` and `2-Cols` types PDF
> - Added Menu's `App Data` toggle botton for using `Network` or `Local` data
> - Added images to `IPD Admission Note (Doctor)`, `OPD-ER Medical History`, `Order`, `Focus Note` and `IPD Consult` reports
> - Fixed `AppError` to show Status Code `418: I'm a teapot` as `Error from Client` (not from server)
> - Fixed `Off` multiple order item
> - Fixed nurse cannot `Off` order item

## 0.2.19 (2025-06-18)
> - *(Schema)* Added `kphis`.`ipd_admission_note_dr`.`mem_ruptured_hours` SMALLINT(5) UNSIGNED DEFAULT NULL,
> - *(Schema)* Added `kphis`.`*_vs_vital_sign`.`lr_pos` VARCHAR(3) DEFAULT NULL
> - *(Schema)* Added `kphis`.`*_vs_vital_sign`.`lr_moulding` INT(11) UNSIGNED DEFAULT NULL
> - *(Schema)* Added `kphis`.`*_vs_vital_sign`.`lr_oxytocin_unit` INT(11) UNSIGNED DEFAULT NULL
> - *(Schema)* Added `kphis`.`*_vs_vital_sign`.`lr_oxytocin_rate` INT(11) UNSIGNED DEFAULT NULL
> - *(Schema)* Added `kphis`.`*_vs_vital_sign`.`lr_urine_vol` INT(11) UNSIGNED DEFAULT NULL
> - *(Schema)* Added `kphis`.`*_vs_vital_sign`.`urine_protein` INT(11) UNSIGNED DEFAULT NULL
> - *(Schema)* Added `kphis`.`*_vs_vital_sign`.`urine_sugar` INT(11) UNSIGNED DEFAULT NULL
> - *(Schema)* Added `kphis`.`ipd_vs_lr_moulding` table
> - *(Schema)* Added `kphis`.`ipd_vs_dipstick` table
> - Added `Position`, `Moulding`, `Oxytocin (U/L,Rate)`, `urine (protein,sugar,volume)` to `LR/PP` vital-sign UI form
> - Added `g`,`p`,`last_child`,`lmp`,`edc`,`gestation_age`,`gestation_day` and `mem_ruptured_hours` to `PatientInfo` for convenient
> - Added `Partograph` report
> - Fixed `app_asset` caching mechanic by using Etag

## 0.2.18 (2025-06-08)
> - *(Config)* Added `doctor-intern-roles` to config and apply `(Intern)` to Order and PreOrder's UI + Report
> - Added `Copy to Clipboard` when click at `Drug Name` in Order (only view by pharmacist)
> - Upgraded `json-syntax` with forked GitHub (`syntect` crate cannot provide `pretty json` for us)
> - Fixed `eMAR` report to be grouped by `icode`

## 0.2.17 (2025-05-29)
> - Fixed Multi-page Reports header + footer
> - Enhanced Image Box to be able to create PDF report and can input with PDF file (render 1st page to image)
> - Included `PDF.js` to UI dependencies for rendering PDF file
> - Added `ipd_dc_plan*`,`opd_er_dc_plan*`,`ipd_dc_plan_tmp_*` tables to `kphis_extra`
> - Added GET/POST/DELETE `/api/ipd/dc-plan-an/{an}`
> - Added GET/POST/DELETE `/api/opd-er/dc-plan-id/{opd_er_order_master_id}`
> - Added GET/POST/DELETE `/api/ipd/ipd-dc-plan-tmp/dx`
> - Added GET/POST/DELETE `/api/ipd/ipd-dc-plan-tmp/med`
> - Added GET/POST/DELETE `/api/ipd/ipd-dc-plan-tmp/env`
> - Added GET/POST/DELETE `/api/ipd/ipd-dc-plan-tmp/tx`
> - Added GET/POST/DELETE `/api/ipd/ipd-dc-plan-tmp/diet`
> - Added `setting-template-dc-plan` UI page
> - Added `nurse_note/dc_plan` UI component
> - Added `ipd-discharge-plan` and `opd-discharge-plan` report

## 0.2.16 (2025-05-20)
> - Fixed UI error message popup
> - Added `ipd_document` and `opd_er_document` tables to `kphis_extra`
> - Added GET + POST `/ipd/document/scan-an/{an}`, GET + POST  `/opd-er/document/scan-id/{opd_er_order_master_id}`
> - Added DocumentScan UI (IPD and OPD-ER)

## 0.2.15 (2025-05-17)
> - Fixed `ipd-summary` report to match 2556 version
> - Fixed `cont-to-homemed` order sending a wrong `order-item-type`
> - Removed unnescessary AN check in handler
> - Added CC, PI, informants and VS to `ipd-admission-nurse` form and report

## 0.2.14 (2025-05-14)
> - Changed `ipd-mra` from modal to page
> - Fixed Date-Input mechanic (Press SpaceBar to open Date-Picker, can input and change with Enter key)

## 0.2.13 (2025-05-12)
> - Fixed different between card and table
> - Seperated `kphis-router` from `kphis-backend` and added `api-tests`

## 0.2.12 (2025-05-07)
> - Added `ipd_mra` table to `kphis_extra`
> - Added GET, POST, PUT, DELETE `ipd/mra` API
> - Added `ipd-mra` modal, can be called from `post-admit` page

## 0.2.11 (2025-04-26)
> - Edited `index-plan` modal, nurse can select multiple plan-time
> - Changed `app_asset` state in local-storage to CACHE_CONTROL + service-worker cache
> - Fixed `summary` report's operation-date, start-time, and end-time
> - Added `summary` coder section and `status`
> - Added coding-related columns and `status` to `ipd_symmary_2` table
> - Added PATCH `/ipd/summary` API
> - Added `ipd_summary_note` table
> - Added GET, POST `/ipd/summary-note-id/{summary_id}` API

## 0.2.10 (2025-04-18)
> - Removed `kphis-version` from config
> - Added `nurse-assign-groups` to config
> - Added `nurse-assign` column to `ipd_order_item` and `opd_er_order_item` table
> - Edited `nurse-note` form's `daily-care` to be collapsable
> - Added PATCH `/ipd/order/item` and PATCH `/opd-er/order/item` API
> - Added `Assign` to `index-plan` modal, component and page

## 0.2.9 (2025-04-15)
> - Edited `OPD-ER medical history`, `vital-sign form`, and `prescription-screen` UI
> - Added `injection` order type and selectable in `IndexPlan`
> - Added `med_reconciliation_item_id` to `opd_er_order_item` table
> - Added `opd-er-med-reconcile` tab to OPD-ER main
> - Merged `ipd_order`, `ipd_pre_order` and `opd_er_order` form
> - Merged `ipd_order` and `opd_er_order` component
> - Fixed `pre-order-into` execute_many queries

## 0.2.8 (2025-04-07)
> - Added `kphis-db-util` crate for update database `Schema`, `Stored Procedures` and `Triggers`
> - Changed DataTable to Card in `ipd-search-patient`,`opd-er-order-list`,`ipd-pre-admit-list`,`ipd-post-admit-list`,`ipd-consult-list`
> - Removed `STAT` badge in previous order
> - Added confirm selected/aLL `nurse-order-as`

## 0.2.7 (2025-03-30)
> - Seperated `PatientImage` from `JSON` to `Binary` with `cache-control` header
> - Rebased `PatientInfo`
> - Added VisitTypeId::Visit
> - Added `AsideResizer` to IpdMain, OpdErMain, PreOrderMain, IpdAdmissionNoteDr, IpdSummary
> - Added report button to report detail
> - Added `PostAdmitList` Page
> - Added `/api/ipd/post-admit` GET
> - Added selectable lab button for inserting progress note's 'Evaluation','Objective' and nurse note's 'Assessment'

## 0.2.6 (2025-03-20)
> - Added `continuous` to `opd-er-order` and report
> - Added `/api/ipd/pre-admit` GET and POST
> - Added PreAdmitList page
> - Added Stored Procedures and Triggers for `PreAdmit` at the start of backend service
> - Added screening for `admited` and `revoked` AN at handler level
> - fixed : remove `Action` in new index-plan
> - fixed : index-action must show short result inlined
> - fixed : interventions will render in list (not comma seperated)
> - fixed : nurse-note without focus cannot edit
> - fixed : enable editing nurse_order_as before doctor-confirm

## 0.2.5 (2025-02-26)
> - Splited `api-handler` and `api-pdf` from `backend` crate
> - Rename crates with `kphis-` prefix
> - Added pdf creation test in `kphis-api-pdf` crate
> - Added more unit tests
> - Bumped Rust 2024 edition
> - Added tutorial

## 0.2.4 (2025-02-09)
> - Splited crates for shortening compliation time (`backend` split `api-core` and `api-query`, `frontend` split `ui-core`, `ui-component` and `ui-page`)
> - Changed backend allocator for better memory efficiency in MUSL docker image
> - Rebased all `BASH` and `BAT` script

## 0.2.3 (2025-02-02)
> - Added report preview modal
> - Added report to EMR (IPD only)

## 0.2.2 (2025-01-29)
> - Removed `clone!` without `move`
> - Removed `&mut` from `.children(&mut [..])` to `.children([..])`
> - Use `cargo fmt --all` and add `#[rustfmt::skip]` to maintain some readability
> - Checked all api call to run inside `async_load` and disable button when `loader_is_loading` to prevent duplicate fetch
> - Added internal image server
> - Added images module to menu Setting -> Images Cache
> - Added images module to `progress_note` objective (IPD, OPD-ER)
> - Added images module to `ipd_dr_admission_note`
> - Added images module to `opd_er_medical_history`
> - Added images module to `focus-note` Assessment and Evaluation (IPD, OPD-ER)
> - Added images module to `ipd consult` Data and Finding

## 0.2.1 (2025-01-01)
> - Fixed : ipd_admission_note_nurse form mechanics
> - Fixed : SSE across multiple browser tab
> - Revised kphis `tis620_thai_ci` collation column
> - Changed all database PrimaryKey type to u32 (unsigned int(11)) + update associated key type
> - Added `backend` cron for log and Message clearing
> - Added `backend` cron for reload tls certificate
> - Added focused-path for rendering Reply Modal of `consult` component
> - Added `derive-demo` crate for `Demo` trait
> - Added `Demo` trait to all `ToSchema` models
> - Added `sqlx-tester` crate for testing sql queries
> - Added test to all `backend`'s queries
> - Removed MysqlBinder insert/update boilerplate by Adding `insert` and `update` method to MySqlBinder
> - Edited Admission-Note-Dr report to show PE image only when having canvas data
> - Added previous 1,3,5,7 Lab report

## 0.2.0 (2024-11-27)
> - Seperated log to `kphis_log` database
> - Added `message` table to `kphis_log` database
> - Added GET/POST `/sse/message` and PATCH `/sse/message/id/{id}` API
> - Added PATCH `/user` API to remove user state in server
> - Added wasm_test on client's Route
> - Removed user from localstorage
> - Added User Menu Image
> - Added SSE menu and alert badges
> - Added SSE online checking and server's shutdown alert
> - Added SSE message flow in `order` and `consult` component
> - Added highlight to focused-path of `order` component
> - Added auto-check Service Worker update after online
> - Moved `Update` button to menu after login
> - Added `EventLog` Typst report

## 0.1.38 (2024-11-06)
> - Merged ShowPatientMain component and model
> - Merged IndexPlanPlus model
> - Merged IndexPlan page and component
> - Rebased IndexPlan and IndexAction relation
> - Removed IndexOrderMaster
> - Added selectable vital-sign to insert nurse note's 'Evaluation'
> - Added Ipd Scan to EMR
> - Added eMAR report

## 0.1.37 (2024-10-16)
> - Changed 'IvFluid' order to only select item (cannot input IvFluid name, for MAR report)
> - Added 'Serial' order for implementing MAR report
> - Allowed nurse to add 'Continuous' order
> - HAD/LASA config code + App
> - Grouped and removed unused element's classes
> - Added 'nurse_order_as' and 'doctor_confirm' to order

## 0.1.36 (2024-10-10)
> - Fixed : action-plan not show in not-ordered-date date
> - Fixed : Typst TRP report not show urine/feces
> - Fixed : Typst index-plan not show med_name
> - Fixed : Typst order not show some order (range bug)
> - Fixed : Typst place() MUST in container (Box, Block, Rect)
> - Fixed : order one-day-previous to get more than one order-item-type (hard coded with 'discharge,home-medication')
> - Fixed : Progress-note in template cannot edit/delete
> - Fixed : Med-reconcile Remed to Order throw sqlx error
> - Moved daily-care to intervention area
> - Added 'retain' one-day-previous to frontend
> - Added auto admit Date/Time to new ipd_admission_note_dr
> - Added 'Normal All' to ipd_admission_note_dr
> - Added 'Clone' button for cloning nurse-note
> - Added 'หมายเหตุ' to other of nurse note
> - Added selectable vital-sign to insert order's progress note's 'Objective' and nurse note's 'Assessment'
> - Rebased using ShowPatientMain trait and ShowPatientMainEnum
> - Merged nurse_note, vital_sign components
> - Merged ipd_vital_sign, opd_er_vital_sign page

## 0.1.35 (2024-10-02)
> - Added HosXp Scan to EMR
> - Added custom Alert, Comfirm and Prompt box
> - Added prompt for password 30 minutes before refresh token expired
> - Removed unused element's ID
> - Added fcnote_patient_type ColorSelectOption from config

## 0.1.34 (2024-09-22)
> - Added hosxp-operation-success to config
> - Added OpenAPI support with Scalar UI

## 0.1.33 (2024-08-24)
> - Fixed : input-group width
> - Added EWS config and Typst's EWS setting
> - Added NEWS, PEWS score

## 0.1.32 (2024-08-16)
> - Fixed : inputbox's url-encoding issue
> - Added limit to all searchable page
> - Removed `dominator` and `axum` from `model`
> - Removed `dominator` from `util`
> - Fixed : bootstrap alert, form-inline, table-responsive, searchbox position

## 0.1.31 (2024-08-04)
> - Bumped Bootstrap 4 to Bootstrap 5
> - Implemented `Sass` css preprocessor

## 0.1.30 (2024-07-27)
> - Fixed : WebKit wasm_bindgen error on unwrap_throw/expect_throw
> - Fixed : WebKit specific css

## 0.1.29 (2024-07-25)
> - Added xray, ekg, scan, cart, food external url template

## 0.1.28 (2024-07-22)
> - Added function to all reports buttons
> - Added service worker reload button
> - Fixed : Named downloaded file
> - Added App Asset file cache

## 0.1.27 (2024-07-18)
> - Added all reports
> - Bugs fixed
### Backend
> - Added `/api/lab/head-vn/:vn`
> - Added `/api/opd-er/show-patient-main-vn/:vn`
> - Changed `/api/lab/detail-id/:lab_order_number` to `/api/lab/head-id/:lab_order_number`

## 0.1.26 (2024-06-28)
> - Added ipd-order report
> - Fixed : duplicate endpoint base name Ex: /ipd/order and /ipd/order/:id
### Backend
> - Added `/api/..` prefix to all endpoints
> - Changed `/api/util/operation-admit/:an` to `/api/his/operation-admit-an/:an`
> - Changed all `/api/*/xx/:an` to `/api/*/xx-an/:an` or `/api/*/xx/:key/:id` to `/api/*/xx-key-id/:key/:id`

## 0.1.25 (2024-06-26)
> - Organized endpoints
> - Added api hook for Typst
> - Refactor Typst template to use data.json or fallback to api call

## 0.1.24 (2024-06-16)
> - Fixed Signature error in PDF
> - Added template buttons for PDF/Signed PDF with loader

## 0.1.23 (2024-06-09)
> - Added Graphic (T.P.R.) chart
> - Bump Typst version to 0.11.0

## 0.1.22 (2024-05-27)
> - Added report designer
> - Added Summary report
> - Seperate Typst web worker
> - Typst can fetch files from backend
### Frontend
> - Added page `report_designer`

## 0.1.21 (2024-04-09)
> - Added Document tab content
### Frontend
> - Added component `document`
### Backend
> - Added `/ipd/document/datetime/:an` GET
> - Added `/ipd/document/list/:vn/:an` GET
> - Added `/opd-er/document/list/:vn/:opd_er_order_master_id` GET

## 0.1.20 (2024-04-06)
> - Added EMR tab content
### Frontend
> - Added component `emr`
### Backend
> - Added `/emr/date/:hn` GET
> - Added `/emr/visit/:vn` GET

## 0.1.19 (2024-04-04)
> - Added Lab tab content
### Frontend
> - Added component `lab`
> - Added modal `lab_history`
### Backend
> - Added `/lab/head/:hn` GET
> - Added `/lab/detail/:lab_order_number` GET
> - Added `/lab/item` GET
> - Added `/lab/read/:lab_order_number` POST DELETE

## 0.1.18 (2024-04-01)
> - Added Summary page
### Frontend
> - Added page `summary`
> - Added searchbox `dx`
> - Added searchbox `hosp`
### Backend
> - Added `/ipd/summary` GET POST
> - Added `/search/box/hosp/:search_text` GET

## 0.1.17 (2024-03-19)
> - Added Doctor-In-Charge tab content
### Frontend
> - Added component `doctor_in_charge`
### Backend
> - Added `/ipd/doctor-in-charge` GET POST DELETE

## 0.1.16 (2024-03-18)
> - Added I/O tab content
### Frontend
> - Added component `io`
### Backend
> - Added `/ipd/io-date/:an` GET
> - Added `/ipd/io` GET POST DELETE
> - Added `/opd-er/io-date/:opd_er_order_master_id` GET
> - Added `/opd-er/io` GET POST DELETE

## 0.1.15 (2024-03-14)
> - Added Med Reconcilliation tab content
### Frontend
> - Added component `ipd_med_reconcile`
> - Added component `ipd_med_reconcile_hosxp`
> - Added component `ipd_med_reconcile_last_dose`
> - Added modal `med_reconcile_remed`
### Backend
> - Added `/ipd/med-reconcile` GET POST PATCH DELETE
> - Added `/ipd/med-reconcile-hosxp/:an` GET
> - Added `/ipd/med-reconcile-last-dose/:an` GET
> - Added `/ipd/med-reconcile-note/:med_reconciliation_id` GET POST
> - Added `/ipd/med-reconcile-remed-visit/:hn` GET
> - Added `/ipd/med-reconcile-remed-med` GET

## 0.1.14 (2024-03-06)
> - Add Opd-Er History tab content
### Frontend
- add component `opd_er_medical_history`
### Backend
> - Added `/opd-er/medical-history-trauma` GET POST
> - Added `/opd-er/medical-history-allergy` GET POST
> - Added `/opd-er/medical-history-screen` GET POST
> - Added `/opd-er/medical-history-consult` GET POST
> - Added `/opd-er/medical-history-scan` GET POST
> - Added `/opd-er/medical-history-ft` GET POST

## 0.1.13 (2024-02-29)
> - Added Role User List page
> - Added Role Permission List page
> - Fixed page reloading after token expired
> - Applied Permissions to API and UI
### Frontend
> - Added page `user-list`
> - Added page `permission-list`
### Backend
> - Added `/user-role/user` GET POST
> - Added `/user-role/prelude` GET

## 0.1.12 (2024-02-18) partial KPHIS 24.01.02
> - Added search button
	- search-patient: HN, AN, ชื่อ-สกุล
	- ipd-consult-list: HN, AN, ชื่อ-สกุล
	- ipd-nurse-index-plan-monitor: HN, AN, ชื่อ-สกุล
	- ipd-pharmacy-order-monitor: HN, AN, ชื่อ-สกุล
	- ipd-pre-order-list: ชื่อ Template
	- ipd-index-plan: HN, AN, ชื่อ-สกุล
	- opd-er-nurse-index-plan-monitor: HN, AN, ชื่อ-สกุล
	- opd-er-pharmacy-order-monitor: HN, AN, ชื่อ-สกุล
	- opd-er-order-list: search button
> - Added ค่าใช้จ่าย + โทร : ipd-show-patient-main
> - Added tab Antibiogram : ipd-dr-main
> - Added date select combo box, IPD summary button : tab order
> - Added Yesterday Order (dr only) : ipd order one-day added
> - Added Previous Day Problem (all) : ipd order progressnote added
> - Added Med Reconcile to table column : ipd-nurse-search-patient
> - Added IPD Summary From to table column : ipd-dr-search-patient
> - Added combo box แพทย์เจ้าของใข้ (dr only) : search-patient
> - Added sirs,qsofa to EWS column data : every EWS column data
> - Added Admit history table : ipd-pharmacy-order-monitor
> - Added Last Dose : ipd-dr-admission-note
> - Added redirect to med-reconcillation tab after save with our hospial disease detail : ipd-dr-admission-note
> - Added 'MR' badge to drug name : ipd-continuous-order (pharmacy)
> - Added 'Previous Home Medication' in ipd one-day order
### Frontend
> - Added page `opd-er-vital-sign`
> - Added modal `lab_wbc`
### Backend
> - Added `/ipd/order/one-day-previous/:an` GET
> - Added `/ipd/order/progress-previous` GET
> - Added `/lab/wbc/:key/:value` GET
> - Added `/patient/opd-er` GET
> - Changed `/ipd/order/cont-previous` to `/ipd/order/previous`

## 0.1.11 (2024-02-08)
> - Added Nursing Focus List page -> - tab content
> - Added Nursing Focus Note page -> - tab content
> - Added Nursing Focus Note All page -> - tab content
> - Added Nursing Progress Note tab content
### Frontend
> - Added component `nurse_note`
> - Grouping component `*_vital_sign`,`*_vital_sign_data` and `*_vital_sign_form` to component `vital_sign`
### Backend
> - Added `/ipd/focus-list` GET POST DELETE
> - Added `/ipd/focus-note` GET POST DELETE
> - Added `/opd-er/focus-list` GET POST DELETE
> - Added `/opd-er/focus-note` GET POST DELETE

## 0.1.10 (2024-01-18)
> - Added opd-er-vital-sign tab content
### Frontend
> - Added component `opd_er_vital_sign`
> - Added component `opd_er_vital_sign_data`
> - Added component `opd_er_vital_sign_form`
### Backend
> - Added `/opd-er/vital-sign` GET POST PUT
> - Added `/opd-er/vital-sign/:vs_id` DELETE

## 0.1.9 (2024-01-15)
> - Added Template Nursing page
### Frontend
> - Added page `setting-template-nurse-note`
### Backend
> - Added `/ipd/tmp/group` GET POST DELETE
> - Added `/ipd/tmp/subgroup` GET POST DELETE
> - Added `/ipd/tmp/focus` GET POST DELETE
> - Added `/ipd/tmp/goal` GET POST DELETE
> - Added `/ipd/tmp/intvt` GET POST DELETE

## 0.1.8 (2024-01-11)
> - Added ipd-vital-sign page
> - Added ipd-vital-sign tab content
### Frontend
> - Added page `ipd-vital-sign`
> - Added component `ipd_vital_sign`
> - Added component `ipd_vital_sign_data`
> - Added component `ipd_vital_sign_form`
### Backend
> - Added `/ipd/vital-sign` GET POST PUT
> - Added `/ipd/vital-sign/:vs_id` DELETE
> - Added `/patient/ward/:ward` GET
> - Added `/util/operation-admit/:an` GET

## 0.1.7 (2023-12-28)
> - Added ipd-nurse-admission-note page + pdf
### Frontend
> - Added page `#/ipd-admission-note-nurse/:an`
### Backend
> - Added `/ipd/admission-note-nurse` POST PUT
> - Added `/ipd/admission-note-nurse/:an` GET
> - Added ipd-admission-note-nurse.typ

## 0.1.6 (2023-12-20)
> - Added pharmacy-prescription-screen page and tab link
### Frontend
> - Added `#/prescription-screen/:search-text` page
### Backend
> - Added `/prescription/screen`

## 0.1.5 (2023-12-14)
> - Added ipd-order-pharmacy-monitor
> - Added opd-er-order-pharmacy-monitor
### Frontend
> - Added `#/ipd-order-pharmacy` page
> - Added `#/opd-er-order-pharmacy` page
### Backend
> - Added `/ipd/order/pharmacy` GET
> - Added `/opd-er/order/pharmacy` GET

## 0.1.4 (2023-12-08)
> - Added ipd-index-plan-monitor
> - Added opd-er-index-plan-monitor
> - Added Nurse Planning tab content
### Frontend
> - Added page `#/ipd-index-plan`
> - Added page `#/opd-er-index-plan`
> - Added component `ipd_index_plan`
> - Added component `opd_er_index_plan`

## 0.1.3 (2023-12-05)
> - Added consult
### Frontend
> - Added page `#/ipd-consult-list/:view_by`
> - Added component `ipd_consult`
> - Added modal `consult_form`

### Backend
> - Added `/ipd/consult`
	- GET consult list for `#/ipd/consult-list` page
	- POST insert/update consult for `consult_form` modal
> - Added `/ipd/consult-an/:an`
	- GET consults for `ipd_consult` component
> - Added `/ipd/consult-id/:consult_id`
	- GET consult for `consult_form` modal
	- DELETE consult for `consult_form` modal

## 0.1.2 (2023-11-30)
> > - Added doctor ipd admission note
> > - Added ipd-xxx-search-patient
> > - Added pre-order
> > - Added opd-er-order
> > - Added order tab content
### Frontend
> - Added page `#/ipd-admission-note-dr/:an`
> - Added page `#/ipd-main/:view_by/:an`
> - Added page `#/ipd-pre-order-list`
> - Added page `#/ipd-pre-order/:pre_order_master_id`
> - Added page `#/ipd-search-patient-dr`
> - Added page `#/ipd-search-patient-nurse`
> - Added page `#/ipd-search-patient-pharmacist`
> - Added page `#/ipd-search-patient-other`
> - Added page `#/opd-er-order-list/:view_by`
> - Added page `#/opd-er-order/:view_by/:opd_er_order_master_id"`

> - Added component `common_searchbox`
> - Added component `ipd_order`
> - Added component `ipd_pre_order`
> - Added component `ipd_show_patient_main`
> - Added component `opd_er_order`
> - Added component `opd_er_show_patient_main`

> - Added modal `drug_duplication`
> - Added modal `drug_interaction`
> - Added modal `drug_notify`
> - Added modal `index_note_form`
> - Added modal `index_plan_action_form`
> - Added modal `opd_er_order_new`
> - Added modal `pre_order_new`
> - Added modal `pre_order_select`

### Backend
> - Added `/count/:keyword/:id` GET count of any keyword with id
> - Added `/ipd/admission-note-dr` POST PUT
> - Added `/ipd/admission-note-dr/:an` GET
> - Added `/ipd/admission-note-dr/pharmacy-check/:an` PATCH
> - Added `/ipd/order/cont-previous` GET
> - Added `/ipd/order/cont/to-home-med/:an` GET
> - Added `/ipd/order/order-date/:an` GET
> - Added `/ipd/order/order/:order_id` DELETE
> - Added `/ipd/order/order` GET POST PATCH
> - Added `/ipd/order/progress-note/:progress_note_id` DELETE
> - Added `/ipd/order/progress-note` GET POST
> - Added `/ipd/pre-order/master/:pre_order_master_id` DELETE
> - Added `/ipd/pre-order/master` GET POST
> - Added `/ipd/pre-order/into` POST
> - Added `/ipd/pre-order/order/:order_id` DELETE
> - Added `/ipd/pre-order/order` GET POST
> - Added `/ipd/pre-order/progress-note/:progress_note_id` DELETE
> - Added `/ipd/pre-order/progress-note` GET POST
> - Added `/ipd/index-note/:an` DELETE
> - Added `/ipd/index-note` GET POST
> - Added `/ipd/index-order/master/:an` GET
> - Added `/ipd/index-order/item/:order_item_id` GET
> - Added `/ipd/index-plan/:plan_id` DELETE
> - Added `/ipd/index-plan` GET POST
> - Added `/ipd/show-patient-main/:an` GET
> - Added `/opd-er/medical-history` GET
> - Added `/opd-er/order/master/check/:vn` GET
> - Added `/opd-er/order/master/:opd_er_order_master_id` GET
> - Added `/opd-er/order/master` GET POST
> - Added `/opd-er/order/opd-med/:vn` GET
> - Added `/opd-er/order/order/:order_id` DELETE
> - Added `/opd-er/order/order` GET POST
> - Added `/opd-er/order/progress-note/:progress_note_id` DELETE
> - Added `/opd-er/order/progress-note` GET POST
> - Added `/opd-er/index-order/master/:opd_er_order_master_id` GET
> - Added `/opd-er/index-order/item/:order_item_id` GET
> - Added `/opd-er/index-plan/:plan_id` DELETE
> - Added `/opd-er/index-plan` GET POST
> - Added `/opd-er/show-patient-main/:opd_er_order_master_id` GET
> - Added `/patient/image/:hn` GET
> - Added `/pdf/:template/:id` GET pdf of any template with id
> - Added `/search/box/med/duplicate` GET
> - Added `/search/box/med/interaction` GET
> - Added `/search/box/med/:hn/:search_text` GET
> - Added `/search/box/opd-visit/:mode/:search_text` GET
> - Added `/search/box/ivfluid/:search_text` GET
> - Added `/search/box/lab/:search_text` GET
> - Added `/search/box/patient/:search_text` GET
> - Added `/search/box/xray/:search_text` GET
> - Added `/search/nurse` GET
> - Added `/search/pharmacist` GET
> - Added `/search/other` GET

## 0.1.1 (2023-8-17)
> - Added features
### Frontend
> - Added page `#/ipd_dr_search_patient`
> - Added component `config_ipd_ward_passcode`

### Backend
> - Added `/app` GET
> - Added `/search/dr` GET
> - Added `/ipd/passcode` GET POST
> - Added `/ipd/passcode/:ward` GET

## 0.1.0 (2023-8-11)
> - Initial Commit
### Frontend
> - Added page `#/index`
> - Added page `#/info`
> - Added page `not found`
> - Added component `menu`

### Backend
> - Added `/` GET
> - Added `/user`
    - GET: login
    - POST: refresh token
