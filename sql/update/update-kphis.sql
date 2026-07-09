-- Add pre_admit_master table
CREATE TABLE IF NOT EXISTS `ipd_pre_admit_master` (
  `pre_admit_master_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `vn` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `prev_an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`pre_admit_master_id`) USING BTREE,
  UNIQUE INDEX `vn` (`vn`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;
-- Add opd_er_med_reconciliation table
CREATE TABLE IF NOT EXISTS `opd_er_med_reconciliation` (
  `med_reconciliation_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,
  `pharmacist` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `note` TEXT DEFAULT NULL,
  `doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `med_reconciliation_datetime` DATETIME DEFAULT NULL,
  `phamacist_confirm_datetime` DATETIME DEFAULT NULL,
  `doctor_confirm_datetime` DATETIME DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`med_reconciliation_id`) USING BTREE,
  INDEX `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;
-- Add opd_er_med_reconciliation_item table
CREATE TABLE IF NOT EXISTS `opd_er_med_reconciliation_item` (
  `med_reconciliation_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `med_reconciliation_id` INT(11) UNSIGNED DEFAULT NULL,
  `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,
  `icode` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `med_name` VARCHAR(255) DEFAULT NULL,
  `custom_med_name` VARCHAR(255) DEFAULT NULL,
  `receive_from` VARCHAR(255) DEFAULT NULL,
  `receive_date` DATE DEFAULT NULL,
  `old_drugusage` TEXT DEFAULT NULL,
  `changed_drugusage` TEXT DEFAULT NULL,
  `receive_qty` INT(11) DEFAULT NULL,
  `last_dose_taken_time` DATETIME DEFAULT NULL,
  `last_dose_taken_remark` VARCHAR(255) DEFAULT NULL,
  `use` VARCHAR(1) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`med_reconciliation_item_id`) USING BTREE,
  INDEX `med_reconciliation_id` (`med_reconciliation_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;
-- Add ipd_summary_note table
CREATE TABLE IF NOT EXISTS `ipd_summary_note` (
  `summary_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `summary_id` INT(11) UNSIGNED NOT NULL,
  `note` TEXT DEFAULT NULL,
  `doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`summary_note_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;
-- Add ipd_vs_dipstick table
CREATE TABLE IF NOT EXISTS `ipd_vs_dipstick` (
  `dipstick_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `dipstick_name` VARCHAR(50) DEFAULT NULL,
  PRIMARY KEY (`dipstick_id`) USING BTREE
) ENGINE=MyISAM AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;
-- Add ipd_vs_lr_moulding table
CREATE TABLE IF NOT EXISTS `ipd_vs_lr_moulding` (
  `lr_moulding_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `lr_moulding_name` VARCHAR(50) DEFAULT NULL,
  PRIMARY KEY (`lr_moulding_id`) USING BTREE
) ENGINE=MyISAM AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;
-- Add ipd_vs_stage_of_change table
CREATE TABLE IF NOT EXISTS `ipd_vs_stage_of_change` (
  `stage_of_change_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `stage_of_change_name` VARCHAR(50) DEFAULT NULL,
  PRIMARY KEY (`stage_of_change_id`) USING BTREE
) ENGINE=MyISAM AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;
-- Add ipd_xray_read table
CREATE TABLE IF NOT EXISTS `ipd_xray_read` (
  `xn` INT(11) NOT NULL DEFAULT 0,
  `xray_read_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `xray_read_datetime` DATETIME NOT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`xn`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;
-- Add data to ipd_vs_dipstick
INSERT INTO `ipd_vs_dipstick` VALUES (1, '-ve') ON DUPLICATE KEY UPDATE dipstick_id=1, dipstick_name='-ve';
INSERT INTO `ipd_vs_dipstick` VALUES (2, 'Tr') ON DUPLICATE KEY UPDATE dipstick_id=2, dipstick_name='Tr';
INSERT INTO `ipd_vs_dipstick` VALUES (3, '+1') ON DUPLICATE KEY UPDATE dipstick_id=3, dipstick_name='+1';
INSERT INTO `ipd_vs_dipstick` VALUES (4, '+2') ON DUPLICATE KEY UPDATE dipstick_id=4, dipstick_name='+2';
INSERT INTO `ipd_vs_dipstick` VALUES (5, '+3') ON DUPLICATE KEY UPDATE dipstick_id=5, dipstick_name='+3';
-- Add data to ipd_vs_lr_moulding
INSERT INTO `ipd_vs_lr_moulding` VALUES (1, '0') ON DUPLICATE KEY UPDATE lr_moulding_id=1, lr_moulding_name='0';
INSERT INTO `ipd_vs_lr_moulding` VALUES (2, '+1') ON DUPLICATE KEY UPDATE lr_moulding_id=2, lr_moulding_name='+1';
INSERT INTO `ipd_vs_lr_moulding` VALUES (3, '+2') ON DUPLICATE KEY UPDATE lr_moulding_id=3, lr_moulding_name='+2';
INSERT INTO `ipd_vs_lr_moulding` VALUES (4, '+3') ON DUPLICATE KEY UPDATE lr_moulding_id=4, lr_moulding_name='+3';
-- Add data to ipd_vs_stage_of_change
INSERT INTO `ipd_vs_stage_of_change` VALUES (1, 'Pre-contemplation') ON DUPLICATE KEY UPDATE stage_of_change_id=1, stage_of_change_name='Pre-contemplation';
INSERT INTO `ipd_vs_stage_of_change` VALUES (2, 'Contemplation') ON DUPLICATE KEY UPDATE stage_of_change_id=2, stage_of_change_name='Contemplation';
INSERT INTO `ipd_vs_stage_of_change` VALUES (3, 'Determination') ON DUPLICATE KEY UPDATE stage_of_change_id=3, stage_of_change_name='Determination';
INSERT INTO `ipd_vs_stage_of_change` VALUES (4, 'Action') ON DUPLICATE KEY UPDATE stage_of_change_id=4, stage_of_change_name='Action';
INSERT INTO `ipd_vs_stage_of_change` VALUES (5, 'Maintenance') ON DUPLICATE KEY UPDATE stage_of_change_id=5, stage_of_change_name='Maintenance';
INSERT INTO `ipd_vs_stage_of_change` VALUES (6, 'Relapse') ON DUPLICATE KEY UPDATE stage_of_change_id=6, stage_of_change_name='Relapse';
-- Update OPD_ER_DOCUMENT resource
INSERT INTO `system_ac_resource` VALUES ('OPD_ER_DOCUMENT', NULL, 'DATA', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
  ON DUPLICATE KEY UPDATE `resource`='OPD_ER_DOCUMENT',`resource_desc`=NULL, `resource_type`='DATA',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update OPD_ER_DOCUMENT resource
INSERT INTO `system_ac_resource` VALUES ('SYSTEM_AC_REPORT', NULL, 'DATA', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
  ON DUPLICATE KEY UPDATE `resource`='SYSTEM_AC_REPORT',`resource_desc`=NULL, `resource_type`='DATA',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update OPD_ER_DOCUMENT_PRINT
INSERT INTO `system_ac_permission` VALUES ('OPD_ER_DOCUMENT_PRINT', 'OPD_ER_DOCUMENT', 'PRINT', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
  ON DUPLICATE KEY UPDATE `permission`='OPD_ER_DOCUMENT_PRINT',`resource`='OPD_ER_DOCUMENT',`operation`='PRINT',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update permission for confirm-order-as
INSERT INTO `system_ac_permission` VALUES ('IPD_ORDER_CHECK', 'IPD_ORDER', 'CHECK', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0)
  ON DUPLICATE KEY UPDATE `permission`='IPD_ORDER_CHECK',`resource`='IPD_ORDER',`operation`='CHECK',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;
INSERT INTO `system_ac_permission` VALUES ('OPD_ER_ORDER_CHECK', 'OPD_ER_ORDER', 'CHECK', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0)
  ON DUPLICATE KEY UPDATE `permission`='OPD_ER_ORDER_CHECK',`resource`='OPD_ER_ORDER',`operation`='CHECK',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;
-- Update SYSTEM_AC_REPORT_ADD
INSERT INTO `system_ac_permission` VALUES ('SYSTEM_AC_REPORT_ADD', 'SYSTEM_AC_REPORT', 'ADD', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
  ON DUPLICATE KEY UPDATE `permission`='SYSTEM_AC_REPORT_ADD',`resource`='SYSTEM_AC_REPORT',`operation`='ADD',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update SYSTEM_AC_REPORT_EDIT
INSERT INTO `system_ac_permission` VALUES ('SYSTEM_AC_REPORT_EDIT', 'SYSTEM_AC_REPORT', 'EDIT', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
  ON DUPLICATE KEY UPDATE `permission`='SYSTEM_AC_REPORT_EDIT',`resource`='SYSTEM_AC_REPORT',`operation`='EDIT',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update SYSTEM_AC_REPORT_REMOVE
INSERT INTO `system_ac_permission` VALUES ('SYSTEM_AC_REPORT_REMOVE', 'SYSTEM_AC_REPORT', 'REMOVE', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
  ON DUPLICATE KEY UPDATE `permission`='SYSTEM_AC_REPORT_REMOVE',`resource`='SYSTEM_AC_REPORT',`operation`='REMOVE',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update SYSTEM_AC_REPORT_VIEW
INSERT INTO `system_ac_permission` VALUES ('SYSTEM_AC_REPORT_VIEW', 'SYSTEM_AC_REPORT', 'VIEW', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
  ON DUPLICATE KEY UPDATE `permission`='SYSTEM_AC_REPORT_VIEW',`resource`='SYSTEM_AC_REPORT',`operation`='VIEW',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update DOCTOR_INTERN role
INSERT INTO `system_ac_role` VALUES ('DOCTOR_INTERN', 'แพทย์ INTERN', 'DOCTOR', 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0)
  ON DUPLICATE KEY UPDATE `role`='DOCTOR_INTERN',`role_desc`='แพทย์ INTERN',`parent_role`='DOCTOR',`create_user`='jommarn',`create_datetime`='2020-01-18 15:54:41',`update_user`='jommarn',`update_datetime`='2020-01-18 15:54:41',`version`=0;
-- Update OPD_ER_DOCUMENT_PRINT
INSERT INTO `system_ac_role_permission` VALUES ('MEDICAL_RECORD', 'OPD_ER_DOCUMENT_PRINT', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
	ON DUPLICATE KEY UPDATE `role`='MEDICAL_RECORD',`permission`='OPD_ER_DOCUMENT_PRINT',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update role-permission for confirm-order-as
INSERT INTO `system_ac_role_permission` VALUES ('MSO', 'IPD_ORDER_CHECK', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0)
	ON DUPLICATE KEY UPDATE `role`='MSO',`permission`='IPD_ORDER_CHECK',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;
INSERT INTO `system_ac_role_permission` VALUES ('MSO', 'OPD_ER_ORDER_CHECK', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0)
	ON DUPLICATE KEY UPDATE `role`='MSO',`permission`='OPD_ER_ORDER_CHECK',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;
-- Update SYSTEM_AC_REPORT_ADD for IT_ADMIN
INSERT INTO `system_ac_role_permission` VALUES ('IT_ADMIN', 'SYSTEM_AC_REPORT_ADD', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
	ON DUPLICATE KEY UPDATE `role`='IT_ADMIN',`permission`='SYSTEM_AC_REPORT_ADD',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update SYSTEM_AC_REPORT_EDIT for IT_ADMIN
INSERT INTO `system_ac_role_permission` VALUES ('IT_ADMIN', 'SYSTEM_AC_REPORT_EDIT', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
	ON DUPLICATE KEY UPDATE `role`='IT_ADMIN',`permission`='SYSTEM_AC_REPORT_EDIT',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update SYSTEM_AC_REPORT_REMOVE for IT_ADMIN
INSERT INTO `system_ac_role_permission` VALUES ('IT_ADMIN', 'SYSTEM_AC_REPORT_REMOVE', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
	ON DUPLICATE KEY UPDATE `role`='IT_ADMIN',`permission`='SYSTEM_AC_REPORT_REMOVE',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update SYSTEM_AC_REPORT_VIEW for IT_ADMIN
INSERT INTO `system_ac_role_permission` VALUES ('IT_ADMIN', 'SYSTEM_AC_REPORT_VIEW', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
	ON DUPLICATE KEY UPDATE `role`='IT_ADMIN',`permission`='SYSTEM_AC_REPORT_VIEW',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update IPD_ORDER_OFF for nurse
INSERT INTO `system_ac_role_permission` VALUES ('NURSE_ER_RN_EMT', 'IPD_ORDER_OFF', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
	ON DUPLICATE KEY UPDATE `role`='NURSE_ER_RN_EMT',`permission`='IPD_ORDER_OFF',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
INSERT INTO `system_ac_role_permission` VALUES ('NURSE_IPD_RN_TN', 'IPD_ORDER_OFF', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
	ON DUPLICATE KEY UPDATE `role`='NURSE_IPD_RN_TN',`permission`='IPD_ORDER_OFF',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
INSERT INTO `system_ac_role_permission` VALUES ('NURSE_OPD_RN_TN', 'IPD_ORDER_OFF', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0)
	ON DUPLICATE KEY UPDATE `role`='NURSE_OPD_RN_TN',`permission`='IPD_ORDER_OFF',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;
-- Update order item types
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'note', 'Note', 1) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='note', order_item_type_name='Note', display_order=1;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'off', 'Off', 2) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='off', order_item_type_name='Off', display_order=2;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'lab', 'Lab', 3) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='lab', order_item_type_name='Lab', display_order=3;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'xray', 'X-Ray', 4) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='xray', order_item_type_name='X-Ray', display_order=4;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'ivfluid', 'IV Fluid', 5) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='ivfluid', order_item_type_name='IV Fluid', display_order=5;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'serial', 'Serial', 6) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='serial', order_item_type_name='Serial', display_order=6;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'record', 'Record', 7) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='record', order_item_type_name='Record', display_order=7;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'injection', 'Injection', 8) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='injection', order_item_type_name='Injection', display_order=8;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'med', 'Med', 9) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='med', order_item_type_name='Med', display_order=9;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'retain', 'Retain', 10) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='retain', order_item_type_name='Retain', display_order=10;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'other', 'Other', 11) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='other', order_item_type_name='Other', display_order=11;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'pharm', 'Pharmacist notify', 12) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='pharm', order_item_type_name='Pharmacist notify', display_order=12;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'discharge', 'Discharge', 13) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='discharge', order_item_type_name='Discharge', display_order=13;
INSERT INTO `ipd_order_item_type` VALUES ('oneday', 'home-medication', 'Home Medication', 14) ON DUPLICATE KEY UPDATE order_type='oneday', order_item_type='home-medication', order_item_type_name='Home Medication', display_order=14;
INSERT INTO `ipd_order_item_type` VALUES ('continuous', 'note', 'Note', 1) ON DUPLICATE KEY UPDATE order_type='continuous', order_item_type='note', order_item_type_name='Note', display_order=1;
INSERT INTO `ipd_order_item_type` VALUES ('continuous', 'off', 'Off', 2) ON DUPLICATE KEY UPDATE order_type='continuous', order_item_type='off', order_item_type_name='Off', display_order=2;
INSERT INTO `ipd_order_item_type` VALUES ('continuous', 'food', 'Food', 3) ON DUPLICATE KEY UPDATE order_type='continuous', order_item_type='food', order_item_type_name='Food', display_order=3;
INSERT INTO `ipd_order_item_type` VALUES ('continuous', 'activity', 'Activity', 4) ON DUPLICATE KEY UPDATE order_type='continuous', order_item_type='activity', order_item_type_name='Activity', display_order=4;
INSERT INTO `ipd_order_item_type` VALUES ('continuous', 'serial', 'Serial', 5) ON DUPLICATE KEY UPDATE order_type='continuous', order_item_type='serial', order_item_type_name='Serial', display_order=5;
INSERT INTO `ipd_order_item_type` VALUES ('continuous', 'record', 'Record', 6) ON DUPLICATE KEY UPDATE order_type='continuous', order_item_type='record', order_item_type_name='Record', display_order=6;
INSERT INTO `ipd_order_item_type` VALUES ('continuous', 'injection', 'Injection', 7) ON DUPLICATE KEY UPDATE order_type='continuous', order_item_type='injection', order_item_type_name='Injecton', display_order=7;
INSERT INTO `ipd_order_item_type` VALUES ('continuous', 'med', 'Med', 8) ON DUPLICATE KEY UPDATE order_type='continuous', order_item_type='med', order_item_type_name='Med', display_order=8;
INSERT INTO `ipd_order_item_type` VALUES ('continuous', 'other', 'Other', 9) ON DUPLICATE KEY UPDATE order_type='continuous', order_item_type='other', order_item_type_name='Other', display_order=9;
-- Update progress note item types
INSERT INTO `ipd_progress_note_item_type` VALUES ('problem-list', 'Problem List', 1) ON DUPLICATE KEY UPDATE progress_note_item_type='problem-list', progress_note_item_type_name='Problem List', display_order=1;
INSERT INTO `ipd_progress_note_item_type` VALUES ('note', 'Note', 2) ON DUPLICATE KEY UPDATE progress_note_item_type='note', progress_note_item_type_name='Note', display_order=2;
INSERT INTO `ipd_progress_note_item_type` VALUES ('subjective', 'Subjective', 3) ON DUPLICATE KEY UPDATE progress_note_item_type='subjective', progress_note_item_type_name='Subjective', display_order=3;
INSERT INTO `ipd_progress_note_item_type` VALUES ('objective', 'Objective', 4) ON DUPLICATE KEY UPDATE progress_note_item_type='objective', progress_note_item_type_name='Objectie', display_order=4;
INSERT INTO `ipd_progress_note_item_type` VALUES ('assessment', 'Assessment', 5) ON DUPLICATE KEY UPDATE progress_note_item_type='assessment', progress_note_item_type_name='Assessment', display_order=5;
INSERT INTO `ipd_progress_note_item_type` VALUES ('plan', 'Plan', 6) ON DUPLICATE KEY UPDATE progress_note_item_type='plan', progress_note_item_type_name='Plan', display_order=6;
-- Update opd_er_patient_status
INSERT INTO `opd_er_patient_status` VALUES (9, 'รอ ATK', 'รอตรวจเขื้อ', 5, 'jommarn', '2021-11-04 12:30:42', 'jommarn', '2021-11-04 12:30:42', 1) ON DUPLICATE KEY UPDATE er_patient_status_id=9, er_patient_status_name='รอ ATK', er_patient_status_name_pt='รอตรวจเขื้อ', display_order=5, create_user='jommarn', create_datetime='2021-11-04 12:30:42', update_user='jommarn', update_datetime='2021-11-04 12:30:42', `version`=1;
INSERT INTO `opd_er_patient_status` VALUES (10, 'Admit แล้ว', 'นอน รพ.แล้ว', 10, 'jommarn', '2021-11-04 12:30:42', 'jommarn', '2021-11-04 12:30:42', 1) ON DUPLICATE KEY UPDATE er_patient_status_id=10, er_patient_status_name='Admit แล้ว', er_patient_status_name_pt='นอน รพ.แล้ว', display_order=10, create_user='jommarn', create_datetime='2021-11-04 12:30:42', update_user='jommarn', update_datetime='2021-11-04 12:30:42', `version`=1;
-- Add Unresponsive to AVPU score
INSERT INTO `ipd_vs_avpu` VALUES (4, 'Unresponsive') ON DUPLICATE KEY UPDATE avpu_id=4, avpu_name='Unresponsive';
-- Add SRM and ARM to ipd_vs_lr_mem
INSERT INTO `ipd_vs_lr_mem` VALUES (4, 'SRM') ON DUPLICATE KEY UPDATE lr_mem_id=4, lr_mem_name='SRM';
INSERT INTO `ipd_vs_lr_mem` VALUES (5, 'ARM') ON DUPLICATE KEY UPDATE lr_mem_id=5, lr_mem_name='ARM';
-- Add crt and band for EWS score (IPD), partograph parameters
ALTER TABLE `ipd_vs_vital_sign`
  ROW_FORMAT = DYNAMIC,
  MODIFY COLUMN `braden` TEXT DEFAULT NULL,
  ADD COLUMN IF NOT EXISTS `action_id` INT(11) UNSIGNED DEFAULT NULL AFTER `vs_id`,
  ADD COLUMN IF NOT EXISTS `sat_room_air` INT(3) UNSIGNED DEFAULT NULL AFTER `sat`,
  ADD COLUMN IF NOT EXISTS `crt` INT(2) DEFAULT NULL AFTER `pleak_flow`,
  ADD COLUMN IF NOT EXISTS `band` INT(3) DEFAULT NULL AFTER `crt`,
  ADD COLUMN IF NOT EXISTS `lr_pos` VARCHAR(3) DEFAULT NULL AFTER `band`,
  ADD COLUMN IF NOT EXISTS `lr_moulding` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_pos`,
  ADD COLUMN IF NOT EXISTS `lr_oxytocin_unit` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_moulding`,
  ADD COLUMN IF NOT EXISTS `lr_oxytocin_rate` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_oxytocin_unit`,
  ADD COLUMN IF NOT EXISTS `lr_urine_vol` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_oxytocin_rate`,
  ADD COLUMN IF NOT EXISTS `urine_protein` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_urine_vol`,
  ADD COLUMN IF NOT EXISTS `urine_sugar` INT(11) UNSIGNED DEFAULT NULL AFTER `urine_protein`,
  ADD COLUMN IF NOT EXISTS `diet` VARCHAR(20) DEFAULT NULL AFTER `urine_sugar`,
  ADD COLUMN IF NOT EXISTS `barthel_index` TEXT DEFAULT NULL AFTER `diet`,
  ADD COLUMN IF NOT EXISTS `aggression_oas` TEXT DEFAULT NULL AFTER `barthel_index`,
  ADD COLUMN IF NOT EXISTS `alcohol_ciwa` TEXT DEFAULT NULL AFTER `aggression_oas`,
  ADD COLUMN IF NOT EXISTS `alcohol_aws` TEXT DEFAULT NULL AFTER `alcohol_ciwa`,
  ADD COLUMN IF NOT EXISTS `amphetamine_awq` TEXT DEFAULT NULL AFTER `alcohol_aws`,
  ADD COLUMN IF NOT EXISTS `motivation_scale` TINYINT(2) UNSIGNED DEFAULT NULL AFTER `amphetamine_awq`,
  ADD COLUMN IF NOT EXISTS `craving_scale` TINYINT(2) UNSIGNED DEFAULT NULL AFTER `motivation_scale`,
  ADD COLUMN IF NOT EXISTS `stage_of_change_id` INT(11) UNSIGNED DEFAULT NULL AFTER `craving_scale`,
  ADD COLUMN IF NOT EXISTS `depress_2q` TEXT DEFAULT NULL AFTER `stage_of_change_id`,
  ADD COLUMN IF NOT EXISTS `depress_9q` TEXT DEFAULT NULL AFTER `depress_2q`,
  ADD COLUMN IF NOT EXISTS `suicide_8q` TEXT DEFAULT NULL AFTER `depress_9q`;
-- Add crt and band for EWS score (OPD-ER), partograph parameters and change type of OPD-ER dtx to the same as IPD
ALTER TABLE `opd_er_vs_vital_sign`
  ROW_FORMAT = DYNAMIC,
  MODIFY COLUMN `dtx` VARCHAR(10) DEFAULT NULL,
  MODIFY COLUMN `braden` TEXT DEFAULT NULL,
  ADD COLUMN IF NOT EXISTS `action_id` INT(11) UNSIGNED DEFAULT NULL AFTER `vs_id`,
  ADD COLUMN IF NOT EXISTS `sat_room_air` INT(3) UNSIGNED DEFAULT NULL AFTER `sat`,
  ADD COLUMN IF NOT EXISTS `crt` INT(2) DEFAULT NULL AFTER `pleak_flow`,
  ADD COLUMN IF NOT EXISTS `band` INT(3) DEFAULT NULL AFTER `crt`,
  ADD COLUMN IF NOT EXISTS `lr_pos` VARCHAR(3) DEFAULT NULL AFTER `band`,
  ADD COLUMN IF NOT EXISTS `lr_moulding` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_pos`,
  ADD COLUMN IF NOT EXISTS `lr_oxytocin_unit` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_moulding`,
  ADD COLUMN IF NOT EXISTS `lr_oxytocin_rate` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_oxytocin_unit`,
  ADD COLUMN IF NOT EXISTS `lr_urine_vol` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_oxytocin_rate`,
  ADD COLUMN IF NOT EXISTS `urine_protein` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_urine_vol`,
  ADD COLUMN IF NOT EXISTS `urine_sugar` INT(11) UNSIGNED DEFAULT NULL AFTER `urine_protein`,
  ADD COLUMN IF NOT EXISTS `diet` VARCHAR(20) DEFAULT NULL AFTER `urine_sugar`,
  ADD COLUMN IF NOT EXISTS `barthel_index` TEXT DEFAULT NULL AFTER `diet`,
  ADD COLUMN IF NOT EXISTS `aggression_oas` TEXT DEFAULT NULL AFTER `barthel_index`,
  ADD COLUMN IF NOT EXISTS `alcohol_ciwa` TEXT DEFAULT NULL AFTER `aggression_oas`,
  ADD COLUMN IF NOT EXISTS `alcohol_aws` TEXT DEFAULT NULL AFTER `alcohol_ciwa`,
  ADD COLUMN IF NOT EXISTS `amphetamine_awq` TEXT DEFAULT NULL AFTER `alcohol_aws`,
  ADD COLUMN IF NOT EXISTS `motivation_scale` TINYINT(2) UNSIGNED DEFAULT NULL AFTER `amphetamine_awq`,
  ADD COLUMN IF NOT EXISTS `craving_scale` TINYINT(2) UNSIGNED DEFAULT NULL AFTER `motivation_scale`,
  ADD COLUMN IF NOT EXISTS `stage_of_change_id` INT(11) UNSIGNED DEFAULT NULL AFTER `craving_scale`,
  ADD COLUMN IF NOT EXISTS `depress_2q` TEXT DEFAULT NULL AFTER `stage_of_change_id`,
  ADD COLUMN IF NOT EXISTS `depress_9q` TEXT DEFAULT NULL AFTER `depress_2q`,
  ADD COLUMN IF NOT EXISTS `suicide_8q` TEXT DEFAULT NULL AFTER `depress_9q`;
-- Add pharmacist_check
ALTER TABLE `ipd_pre_order`
  ADD COLUMN IF NOT EXISTS `pharmacist_check` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL AFTER `pharmacist_accept_time`,
  ADD COLUMN IF NOT EXISTS `pharmacist_check_time` DATETIME DEFAULT NULL AFTER `pharmacist_check`;
-- Add nuser_order_as and doctor_confirm_time (IPD)
ALTER TABLE `ipd_order`
  ADD COLUMN IF NOT EXISTS `nurse_order_as` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL AFTER `order_confirm`,
  ADD COLUMN IF NOT EXISTS `doctor_confirm_time` DATETIME NULL DEFAULT NULL AFTER `nurse_order_as`,
  ADD COLUMN IF NOT EXISTS `pharmacist_check` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL AFTER `pharmacist_accept_time`,
  ADD COLUMN IF NOT EXISTS `pharmacist_check_time` DATETIME DEFAULT NULL AFTER `pharmacist_check`;
-- Add nuser_order_as and doctor_confirm_time (OPD-ER)
ALTER TABLE `opd_er_order`
  ADD COLUMN IF NOT EXISTS `nurse_order_as` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL AFTER `order_confirm`,
  ADD COLUMN IF NOT EXISTS `doctor_confirm_time` DATETIME NULL DEFAULT NULL AFTER `nurse_order_as`,
  ADD COLUMN IF NOT EXISTS `pharmacist_check` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL AFTER `pharmacist_accept_time`,
  ADD COLUMN IF NOT EXISTS `pharmacist_check_time` DATETIME DEFAULT NULL AFTER `pharmacist_check`,
  ADD COLUMN IF NOT EXISTS `pre_order_id` INT(11) UNSIGNED DEFAULT NULL AFTER `pharmacist_order_status`,
  ADD COLUMN IF NOT EXISTS `pre_order_date` DATE DEFAULT NULL AFTER `pre_order_id`,
  ADD COLUMN IF NOT EXISTS `pre_order_time` TIME DEFAULT NULL AFTER `pre_order_date`;
-- Add nurse_assign, first_qty, qty, due_doctor, due_doctor_note, due_pharm, due_pharm_note to ipd_order_item
ALTER TABLE `ipd_order_item`
  ADD COLUMN IF NOT EXISTS `nurse_assign` VARCHAR(20) DEFAULT NULL AFTER `med_reconciliation_item_id`,
  ADD COLUMN IF NOT EXISTS `first_qty` INT(11) NULL DEFAULT NULL AFTER `nurse_assign`,
  ADD COLUMN IF NOT EXISTS `qty` INT(11) NULL DEFAULT NULL AFTER `first_qty`,
  ADD COLUMN IF NOT EXISTS `due_doctor` VARCHAR(1) DEFAULT NULL AFTER `qty`,
  ADD COLUMN IF NOT EXISTS `due_doctor_note` TEXT DEFAULT NULL AFTER `due_doctor`,
  ADD COLUMN IF NOT EXISTS `due_pharm` VARCHAR(1) DEFAULT NULL AFTER `due_doctor_note`,
  ADD COLUMN IF NOT EXISTS `due_pharm_note` TEXT DEFAULT NULL AFTER `due_pharm`;
-- Add med_reconciliation_item_id, nurse_assign, first_qty, qty, due_doctor, due_doctor_note, due_pharm, due_pharm_note to opd_er_order_item
ALTER TABLE `opd_er_order_item`
  ADD COLUMN IF NOT EXISTS `med_reconciliation_item_id` INT(11) UNSIGNED DEFAULT NULL AFTER `icode`,
  ADD COLUMN IF NOT EXISTS `nurse_assign` VARCHAR(20) DEFAULT NULL AFTER `med_reconciliation_item_id`,
  ADD COLUMN IF NOT EXISTS `first_qty` INT(11) NULL DEFAULT NULL AFTER `nurse_assign`,
  ADD COLUMN IF NOT EXISTS `qty` INT(11) NULL DEFAULT NULL AFTER `first_qty`,
  ADD COLUMN IF NOT EXISTS `due_doctor` VARCHAR(1) DEFAULT NULL AFTER `qty`,
  ADD COLUMN IF NOT EXISTS `due_doctor_note` TEXT DEFAULT NULL AFTER `due_doctor`,
  ADD COLUMN IF NOT EXISTS `due_pharm` VARCHAR(1) DEFAULT NULL AFTER `due_doctor_note`,
  ADD COLUMN IF NOT EXISTS `due_pharm_note` TEXT DEFAULT NULL AFTER `due_pharm`;
-- Add progress_note_enter_datetime to opd_er_order_progress_note
ALTER TABLE `opd_er_order_progress_note`
  ADD COLUMN IF NOT EXISTS `progress_note_enter_datetime` DATETIME DEFAULT NULL AFTER `progress_note_doctor`,
  ADD COLUMN IF NOT EXISTS `pre_order_progress_note_id` INT(11) UNSIGNED DEFAULT NULL AFTER `progress_note_enter_datetime`,
  ADD COLUMN IF NOT EXISTS `pre_order_progress_note_date` DATE DEFAULT NULL AFTER `pre_order_progress_note_id`,
  ADD COLUMN IF NOT EXISTS `pre_order_progress_note_time` TIME DEFAULT NULL AFTER `pre_order_progress_note_date`;
-- Add progress_note_item_detail_2 to opd_er_order_progress_note_item
ALTER TABLE `opd_er_order_progress_note_item` ADD COLUMN IF NOT EXISTS `progress_note_item_detail_2` TEXT DEFAULT NULL AFTER `progress_note_item_detail`;
-- Change size of fcnote_patient_type to 2 (IPD)
ALTER TABLE `ipd_focus_note` MODIFY COLUMN `fcnote_patient_type` VARCHAR(2) DEFAULT NULL;
-- Change size of fcnote_patient_type to 2 (OPD-ER)
ALTER TABLE `opd_er_focus_note` MODIFY COLUMN `fcnote_patient_type` VARCHAR(2) DEFAULT NULL;
-- Add coder to ipd_summary_2
ALTER TABLE `ipd_summary_2`
  MODIFY COLUMN `hospital_refer` varchar(9) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL AFTER `discharge_type`,
  ADD COLUMN IF NOT EXISTS `special_other` VARCHAR(1) DEFAULT NULL AFTER `non_or_other_text`,
  ADD COLUMN IF NOT EXISTS `special_other_text` TEXT DEFAULT NULL AFTER `special_other`,
  ADD COLUMN IF NOT EXISTS `coder_name` VARCHAR(50) DEFAULT NULL AFTER `hospital_refer`,
  ADD COLUMN IF NOT EXISTS `principal_diagnosis_code` VARCHAR(7) DEFAULT NULL AFTER `coder_name`,
  ADD COLUMN IF NOT EXISTS `pre_admission_comorbidity_codes` TEXT DEFAULT NULL AFTER `principal_diagnosis_code`,
  ADD COLUMN IF NOT EXISTS `post_admission_comorbidity_codes` TEXT DEFAULT NULL AFTER `pre_admission_comorbidity_codes`,
  ADD COLUMN IF NOT EXISTS `other_diagnosis_codes` TEXT DEFAULT NULL AFTER `post_admission_comorbidity_codes`,
  ADD COLUMN IF NOT EXISTS `external_cause_codes` TEXT DEFAULT NULL AFTER `other_diagnosis_codes`,
  ADD COLUMN IF NOT EXISTS `main_procedure_code` VARCHAR(7) DEFAULT NULL AFTER `external_cause_codes`,
  ADD COLUMN IF NOT EXISTS `other_procedure_codes` TEXT DEFAULT NULL AFTER `main_procedure_code`,
  ADD COLUMN IF NOT EXISTS `status` VARCHAR(7) DEFAULT NULL AFTER `other_procedure_codes`,
  ADD INDEX IF NOT EXISTS `status` (`status`) USING BTREE;
-- Change ipd_dr_admission_note to be able to create with pre-admit
-- Added LR related, Review of System
ALTER TABLE `ipd_dr_admission_note`
  ROW_FORMAT = DYNAMIC,
  MODIFY COLUMN `receiver_medication_date` DATE DEFAULT NULL,
  MODIFY COLUMN `receiver_medication_time` TIME DEFAULT NULL,
  MODIFY COLUMN `anc` VARCHAR(200) DEFAULT NULL,
  MODIFY COLUMN `braden_scale` TEXT DEFAULT NULL,
  ADD COLUMN IF NOT EXISTS `nurse_licenseno` VARCHAR(150) DEFAULT NULL AFTER `nurse_pos`,
  ADD COLUMN IF NOT EXISTS `mem_ruptured_hours` SMALLINT(5) UNSIGNED DEFAULT NULL AFTER `doc_pos`,
  ADD COLUMN IF NOT EXISTS `lr_back_fetus` VARCHAR(20) DEFAULT NULL AFTER `mem_ruptured_hours`,
  ADD COLUMN IF NOT EXISTS `lr_presentation` VARCHAR(20) DEFAULT NULL AFTER `lr_back_fetus`,
  ADD COLUMN IF NOT EXISTS `lr_engagement` VARCHAR(1) DEFAULT NULL AFTER `lr_presentation`,
  ADD COLUMN IF NOT EXISTS `lr_prominence` VARCHAR(20) DEFAULT NULL AFTER `lr_engagement`,
  ADD COLUMN IF NOT EXISTS `lr_attitude` VARCHAR(20) DEFAULT NULL AFTER `lr_prominence`,
  ADD COLUMN IF NOT EXISTS `lr_fhr` SMALLINT(5) UNSIGNED DEFAULT NULL AFTER `lr_attitude`,
  ADD COLUMN IF NOT EXISTS `lr_fhr_irrigular` VARCHAR(1) DEFAULT NULL AFTER `lr_fhr`,
  ADD COLUMN IF NOT EXISTS `lr_efw` SMALLINT(5) UNSIGNED DEFAULT NULL AFTER `lr_fhr_irrigular`,
  ADD COLUMN IF NOT EXISTS `lr_interval` VARCHAR(20) DEFAULT NULL AFTER `lr_efw`,
  ADD COLUMN IF NOT EXISTS `lr_duration` TINYINT(3) UNSIGNED DEFAULT NULL AFTER `lr_interval`,
  ADD COLUMN IF NOT EXISTS `lr_intensity` VARCHAR(20) DEFAULT NULL AFTER `lr_duration`,
  ADD COLUMN IF NOT EXISTS `lr_pelvic_diagonal` Decimal(3,1) DEFAULT NULL AFTER `lr_intensity`,
  ADD COLUMN IF NOT EXISTS `lr_pelvic_interspinous` Decimal(3,1) DEFAULT NULL AFTER `lr_pelvic_diagonal`,
  ADD COLUMN IF NOT EXISTS `lr_pelvic_sidewall` VARCHAR(20) DEFAULT NULL AFTER `lr_pelvic_interspinous`,
  ADD COLUMN IF NOT EXISTS `lr_ischeal_spine` VARCHAR(20) DEFAULT NULL AFTER `lr_pelvic_sidewall`,
  ADD COLUMN IF NOT EXISTS `lr_sacral_curve` VARCHAR(20) DEFAULT NULL AFTER `lr_ischeal_spine`,
  ADD COLUMN IF NOT EXISTS `lr_pubic_angle` TINYINT(3) UNSIGNED DEFAULT NULL AFTER `lr_sacral_curve`,
  ADD COLUMN IF NOT EXISTS `lr_pelvic_ok` VARCHAR(1) DEFAULT NULL AFTER `lr_pubic_angle`,
  ADD COLUMN IF NOT EXISTS `lr_cx_dilate` TINYINT(3) UNSIGNED DEFAULT NULL AFTER `lr_pelvic_ok`,
  ADD COLUMN IF NOT EXISTS `lr_cx_efface` TINYINT(3) UNSIGNED DEFAULT NULL AFTER `lr_cx_dilate`,
  ADD COLUMN IF NOT EXISTS `lr_cx_station` TINYINT(3) DEFAULT NULL AFTER `lr_cx_efface`,
  ADD COLUMN IF NOT EXISTS `lr_cx_position` VARCHAR(20) DEFAULT NULL AFTER `lr_cx_station`,
  ADD COLUMN IF NOT EXISTS `lr_cx_consistency` VARCHAR(20) DEFAULT NULL AFTER `lr_cx_position`,
  ADD COLUMN IF NOT EXISTS `lr_cx_bishop` TINYINT(3) UNSIGNED DEFAULT NULL AFTER `lr_cx_consistency`,
  ADD COLUMN IF NOT EXISTS `lr_cx_ok` VARCHAR(1) DEFAULT NULL AFTER `lr_cx_bishop`,
  ADD COLUMN IF NOT EXISTS `lr_membrane` VARCHAR(20) DEFAULT NULL AFTER `lr_cx_ok`,
  ADD COLUMN IF NOT EXISTS `lr_amniotic_color` VARCHAR(10) DEFAULT NULL AFTER `lr_membrane`,
  ADD COLUMN IF NOT EXISTS `lr_amniotic_smell` VARCHAR(10) DEFAULT NULL AFTER `lr_amniotic_color`,
  ADD COLUMN IF NOT EXISTS `ros_eent` VARCHAR(200) DEFAULT NULL AFTER `lr_amniotic_smell`,
  ADD COLUMN IF NOT EXISTS `ros_neuro` VARCHAR(200) DEFAULT NULL AFTER `ros_eent`,
  ADD COLUMN IF NOT EXISTS `ros_lung` VARCHAR(200) DEFAULT NULL AFTER `ros_neuro`,
  ADD COLUMN IF NOT EXISTS `ros_tb` VARCHAR(200) DEFAULT NULL AFTER `ros_lung`,
  ADD COLUMN IF NOT EXISTS `ros_ht` VARCHAR(200) DEFAULT NULL AFTER `ros_tb`,
  ADD COLUMN IF NOT EXISTS `ros_heart` VARCHAR(200) DEFAULT NULL AFTER `ros_ht`,
  ADD COLUMN IF NOT EXISTS `ros_liver` VARCHAR(200) DEFAULT NULL AFTER `ros_heart`,
  ADD COLUMN IF NOT EXISTS `ros_gi` VARCHAR(200) DEFAULT NULL AFTER `ros_liver`,
  ADD COLUMN IF NOT EXISTS `ros_endocrine` VARCHAR(200) DEFAULT NULL AFTER `ros_gi`,
  ADD COLUMN IF NOT EXISTS `ros_kidney` VARCHAR(200) DEFAULT NULL AFTER `ros_endocrine`,
  ADD COLUMN IF NOT EXISTS `ros_tumour` VARCHAR(200) DEFAULT NULL AFTER `ros_kidney`,
  ADD COLUMN IF NOT EXISTS `ros_hemato` VARCHAR(200) DEFAULT NULL AFTER `ros_tumour`,
  ADD COLUMN IF NOT EXISTS `ros_rheumato` VARCHAR(200) DEFAULT NULL AFTER `ros_hemato`,
  ADD COLUMN IF NOT EXISTS `ros_psychia` VARCHAR(200) DEFAULT NULL AFTER `ros_rheumato`,
  ADD COLUMN IF NOT EXISTS `ros_other` VARCHAR(200) DEFAULT NULL AFTER `ros_psychia`,
  ADD COLUMN IF NOT EXISTS `addict` VARCHAR(20) DEFAULT NULL AFTER `ros_other`,
  ADD COLUMN IF NOT EXISTS `addict_assist` TEXT DEFAULT NULL AFTER `addict`,
  ADD COLUMN IF NOT EXISTS `addict_inj` VARCHAR(1) DEFAULT NULL AFTER `addict_assist`,
  ADD COLUMN IF NOT EXISTS `addict_inj_often` VARCHAR(1) DEFAULT NULL AFTER `addict_inj`,
  ADD COLUMN IF NOT EXISTS `amphetamine_awq` TEXT DEFAULT NULL AFTER `addict_inj_often`,
  ADD COLUMN IF NOT EXISTS `aggression_oas` TEXT DEFAULT NULL AFTER `amphetamine_awq`,
  ADD COLUMN IF NOT EXISTS `motivation_scale` TINYINT(2) UNSIGNED DEFAULT NULL AFTER `aggression_oas`,
  ADD COLUMN IF NOT EXISTS `craving_scale` TINYINT(2) UNSIGNED DEFAULT NULL AFTER `motivation_scale`,
  ADD COLUMN IF NOT EXISTS `stage_of_change_id` INT(11) UNSIGNED DEFAULT NULL AFTER `craving_scale`,
  ADD COLUMN IF NOT EXISTS `alcohol_audit` TEXT DEFAULT NULL AFTER `stage_of_change_id`,
  ADD COLUMN IF NOT EXISTS `alcohol_aws` TEXT DEFAULT NULL AFTER `alcohol_audit`,
  ADD COLUMN IF NOT EXISTS `alcohol_ciwa` TEXT DEFAULT NULL AFTER `alcohol_aws`,
  ADD COLUMN IF NOT EXISTS `depress_2q` TEXT DEFAULT NULL AFTER `alcohol_ciwa`,
  ADD COLUMN IF NOT EXISTS `depress_9q` TEXT DEFAULT NULL AFTER `depress_2q`,
  ADD COLUMN IF NOT EXISTS `depress_cdi` TEXT DEFAULT NULL AFTER `depress_9q`,
  ADD COLUMN IF NOT EXISTS `depress_cesd` TEXT DEFAULT NULL AFTER `depress_cdi`,
  ADD COLUMN IF NOT EXISTS `depress_phqa` TEXT DEFAULT NULL AFTER `depress_cesd`,
  ADD COLUMN IF NOT EXISTS `nicotin_ftnd` TEXT DEFAULT NULL AFTER `depress_phqa`,
  ADD COLUMN IF NOT EXISTS `ptsd_screen` TEXT DEFAULT NULL AFTER `nicotin_ftnd`,
  ADD COLUMN IF NOT EXISTS `ptsd_pisces` TEXT DEFAULT NULL AFTER `ptsd_screen`,
  ADD COLUMN IF NOT EXISTS `ptsd_cries` TEXT DEFAULT NULL AFTER `ptsd_pisces`,
  ADD COLUMN IF NOT EXISTS `suicide_8q` TEXT DEFAULT NULL AFTER `ptsd_cries`,
  ADD COLUMN IF NOT EXISTS `stress_st5` TEXT DEFAULT NULL AFTER `suicide_8q`;
-- Add cc, hpi, vs, informant to ipd_nurse_admission_note
ALTER TABLE `kphis`.`ipd_nurse_admission_note`
  ADD COLUMN IF NOT EXISTS `info_patient` VARCHAR(1) DEFAULT NULL AFTER `an`,
  ADD COLUMN IF NOT EXISTS `info_parent` VARCHAR(1) DEFAULT NULL AFTER `info_patient`,
  ADD COLUMN IF NOT EXISTS `info_spouse` VARCHAR(1) DEFAULT NULL AFTER `info_parent`,
  ADD COLUMN IF NOT EXISTS `info_child` VARCHAR(1) DEFAULT NULL AFTER `info_spouse`,
  ADD COLUMN IF NOT EXISTS `info_relatives` VARCHAR(1) DEFAULT NULL AFTER `info_child`,
  ADD COLUMN IF NOT EXISTS `info_sender` VARCHAR(1) DEFAULT NULL AFTER `info_relatives`,
  ADD COLUMN IF NOT EXISTS `chief_complaints` TEXT DEFAULT NULL AFTER `info_sender`,
  ADD COLUMN IF NOT EXISTS `medical_history` TEXT DEFAULT NULL AFTER `chief_complaints`,
  ADD COLUMN IF NOT EXISTS `vs_admit` TEXT DEFAULT NULL AFTER `medical_history`;
-- Add check
ALTER TABLE `kphis`.`ipd_nurse_index_action`
  ADD COLUMN IF NOT EXISTS `check_datetime` DATETIME DEFAULT NULL AFTER `an`,
  ADD COLUMN IF NOT EXISTS `check_person` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL AFTER `check_datetime`;
ALTER TABLE `kphis`.`opd_er_nurse_index_action`
  ADD COLUMN IF NOT EXISTS `check_datetime` DATETIME DEFAULT NULL AFTER `opd_er_order_master_id`,
  ADD COLUMN IF NOT EXISTS `check_person` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL AFTER `check_datetime`;
-- Add dosage and status
ALTER TABLE `kphis`.`kphis_drug_use_duration`
  ADD COLUMN IF NOT EXISTS `usage` TEXT DEFAULT NULL AFTER `icode`,
  ADD COLUMN IF NOT EXISTS `status` VARCHAR(1) DEFAULT NULL AFTER `exceed_duration3_color`,
  ADD COLUMN IF NOT EXISTS `monitor` TEXT DEFAULT NULL AFTER `status`,
  ADD COLUMN IF NOT EXISTS `monitor_count` TINYINT(3) UNSIGNED DEFAULT NULL AFTER `monitor`,
  ADD COLUMN IF NOT EXISTS `monitor_duration` INT(11) UNSIGNED DEFAULT NULL AFTER `monitor_count`,
  ADD COLUMN IF NOT EXISTS `monitor_status` VARCHAR(1) DEFAULT NULL AFTER `monitor_duration`,
  ADD COLUMN IF NOT EXISTS `info` TEXT DEFAULT NULL AFTER `monitor_status`,
  ADD COLUMN IF NOT EXISTS `info_status` VARCHAR(1) DEFAULT NULL AFTER `info`,
  ADD PRIMARY KEY IF NOT EXISTS `icode` (`icode`) USING BTREE,
  ADD INDEX IF NOT EXISTS `status` (`status`) USING BTREE,
  ADD INDEX IF NOT EXISTS `monitor_status` (`monitor_status`) USING BTREE,
  ADD INDEX IF NOT EXISTS `info_status` (`info_status`) USING BTREE;
-- Change O2 name
INSERT INTO `ipd_vs_o2` VALUES (2, 'Mask c bag') ON DUPLICATE KEY UPDATE o2_id=2, o2_name='Mask c bag';
INSERT INTO `ipd_vs_o2` VALUES (3, 'Collar') ON DUPLICATE KEY UPDATE o2_id=3, o2_name='Collar';
INSERT INTO `ipd_vs_o2` VALUES (4, 'HFNC') ON DUPLICATE KEY UPDATE o2_id=4, o2_name='HFNC';
INSERT INTO `ipd_vs_o2` VALUES (8, 'Tube') ON DUPLICATE KEY UPDATE o2_id=8, o2_name='Tube';
-- ----------------------------------------
-- Add constraint to prevent duplicate data
-- ----------------------------------------
ALTER TABLE `ipd_focus_list_goal_item` ADD CONSTRAINT `fclist_goal` UNIQUE IF NOT EXISTS (`fclist_id`,`goal_id`) USING BTREE;
ALTER TABLE `opd_er_focus_list_goal_item` ADD CONSTRAINT `fclist_goal` UNIQUE IF NOT EXISTS (`fclist_id`,`goal_id`) USING BTREE;
ALTER TABLE `ipd_focus_note_intvt_item` ADD CONSTRAINT `fcnote_intvt` UNIQUE IF NOT EXISTS (`fcnote_id`,`intvt_id`) USING BTREE;
ALTER TABLE `opd_er_focus_note_intvt_item` ADD CONSTRAINT `fcnote_intvt` UNIQUE IF NOT EXISTS (`fcnote_id`,`intvt_id`) USING BTREE;
ALTER TABLE `ipd_focus_note_dlc_item` ADD CONSTRAINT `fcnote_dlc` UNIQUE IF NOT EXISTS (`fcnote_id`,`dlc_id`) USING BTREE;
ALTER TABLE `opd_er_focus_note_dlc_item` ADD CONSTRAINT `fcnote_dlc` UNIQUE IF NOT EXISTS (`fcnote_id`,`dlc_id`) USING BTREE;
-- ----------
-- Add index
-- ----------
ALTER TABLE `opd_er_focus_note`
  ADD INDEX IF NOT EXISTS `fclist_id` (`fclist_id`) USING BTREE,
  ADD INDEX IF NOT EXISTS `fcnote_date` (`fcnote_date`) USING BTREE;
ALTER TABLE `opd_er_io`
  ADD INDEX IF NOT EXISTS `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE,
  ADD INDEX IF NOT EXISTS `opd_er_io_date` (`opd_er_io_date`) USING BTREE;
ALTER TABLE `opd_er_nurse_index_plan`
  DROP INDEX IF EXISTS `an_plan_date`,
  ADD INDEX IF NOT EXISTS `opd_er_order_master_id_plan_date` (`opd_er_order_master_id`,`plan_date`) USING BTREE,
  ADD INDEX IF NOT EXISTS `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE;
ALTER TABLE `opd_er_nurse_index_action`
  DROP INDEX IF EXISTS `an_action_date`,
  ADD INDEX IF NOT EXISTS `opd_er_order_master_id_action_date` (`opd_er_order_master_id`,`action_date`) USING BTREE,
  ADD INDEX IF NOT EXISTS `action_date_time` (`action_date`,`action_time`) USING BTREE;
ALTER TABLE `opd_er_vs_vital_sign`
  ADD INDEX IF NOT EXISTS `action_id` (`action_id`) USING BTREE,
  ADD INDEX IF NOT EXISTS `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE;
ALTER TABLE `ipd_vs_vital_sign`
  ADD INDEX IF NOT EXISTS `action_id` (`action_id`) USING BTREE;
-- ----------------------------------
-- Remove tis620 on non-HOSxP columns
-- ----------------------------------
ALTER TABLE `ipd_focus_note`
  MODIFY COLUMN `general_symptoms` text DEFAULT NULL,
  MODIFY COLUMN `assessment` text DEFAULT NULL,
  MODIFY COLUMN `intvt_text` text DEFAULT NULL,
  MODIFY COLUMN `evalution` text DEFAULT NULL,
  MODIFY COLUMN `dlc_text` text DEFAULT NULL,
  MODIFY COLUMN `other` text DEFAULT NULL;
ALTER TABLE `opd_er_focus_note`
  MODIFY COLUMN `general_symptoms` text DEFAULT NULL,
  MODIFY COLUMN `assessment` text DEFAULT NULL,
  MODIFY COLUMN `intvt_text` text DEFAULT NULL,
  MODIFY COLUMN `evalution` text DEFAULT NULL,
  MODIFY COLUMN `dlc_text` text DEFAULT NULL,
  MODIFY COLUMN `other` text DEFAULT NULL;
ALTER TABLE `ipd_focus_list`
  MODIFY COLUMN `focus_text` text DEFAULT NULL,
  MODIFY COLUMN `goal_id` text DEFAULT NULL,
  MODIFY COLUMN `goal_text` text DEFAULT NULL,
  MODIFY COLUMN `fclist_status` VARCHAR(1) NOT NULL;
ALTER TABLE `opd_er_focus_list`
  MODIFY COLUMN `focus_text` text DEFAULT NULL,
  MODIFY COLUMN `goal_id` text DEFAULT NULL,
  MODIFY COLUMN `goal_text` text DEFAULT NULL,
  MODIFY COLUMN `fclist_status` VARCHAR(1) NOT NULL;
ALTER TABLE `ipd_tmp_dlc` MODIFY COLUMN `dlc_name` text NOT NULL;
ALTER TABLE `ipd_tmp_intvt` MODIFY COLUMN `intvt_name` text NOT NULL;
ALTER TABLE `ipd_vs_lt_arm` MODIFY COLUMN `lt_arm_name` text DEFAULT NULL;
ALTER TABLE `ipd_vs_lt_leg` MODIFY COLUMN `lt_leg_name` text DEFAULT NULL;
ALTER TABLE `ipd_vs_rt_arm` MODIFY COLUMN `rt_arm_name` text DEFAULT NULL;
ALTER TABLE `ipd_vs_rt_leg` MODIFY COLUMN `rt_leg_name` text DEFAULT NULL;
-- -----------------------------------------
-- an to VARCHAR(13) (for fail-safe with VN)
-- -----------------------------------------
ALTER TABLE `ipd_doctor_in_charge` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_dr_admission_note` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_dr_admission_note_item` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_dr_consult` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_dr_consult_signature_reply` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_dr_consult_signature_request` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_focus_list` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_focus_note` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_io` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_med_reconciliation` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_med_reconciliation_item` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_nurse_admission_note` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_nurse_index_action` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_nurse_index_note` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_nurse_index_plan` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_order` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_order_item` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_progress_note` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_progress_note_item` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_summary` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_summary_2` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `ipd_vs_vital_sign` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';
ALTER TABLE `system_patient_lock` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';
-- -----------------------------------------
-- signed to unsigned id, INT(10) to INT(11)
-- -----------------------------------------
ALTER TABLE `ipd_doctor_in_charge` MODIFY COLUMN `doctor_in_charge_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `ipd_dr_admission_note` MODIFY COLUMN `admission_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `ipd_dr_admission_note_item`
  MODIFY COLUMN `admission_note_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `admission_note_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_dr_consult`
  MODIFY COLUMN `consult_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `consult_type` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_dr_consult_signature_reply`
  MODIFY COLUMN `consult_reply_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `consult_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_dr_consult_signature_request`
  MODIFY COLUMN `consult_signature_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `consult_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_dr_consult_type` MODIFY COLUMN `consult_type_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `ipd_emergency` MODIFY COLUMN `emergency_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `ipd_focus_list`
  MODIFY COLUMN `fclist_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `smp_id` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `focus_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_focus_list_goal_item`
  MODIFY COLUMN `fclist_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `fclist_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `goal_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_focus_note`
  MODIFY COLUMN `fcnote_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `fclist_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_focus_note_dlc_item`
  MODIFY COLUMN `fcnote_dlc_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `fcnote_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `dlc_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_focus_note_intvt_item`
  MODIFY COLUMN `fcnote_intvt_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `fcnote_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `intvt_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_io` MODIFY COLUMN `io_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `ipd_med_reconciliation` MODIFY COLUMN `med_reconciliation_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `ipd_med_reconciliation_item`
  MODIFY COLUMN `med_reconciliation_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `med_reconciliation_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_nurse_admission_note` MODIFY COLUMN `nurse_admission_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `ipd_nurse_index_action`
  MODIFY COLUMN `action_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `plan_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_nurse_index_note` MODIFY COLUMN `nurse_index_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `ipd_nurse_index_plan`
  MODIFY COLUMN `plan_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `order_item_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_order`
  MODIFY COLUMN `order_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `pre_order_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_order_item`
  MODIFY COLUMN `order_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `order_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `off_order_item_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `med_reconciliation_item_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_pre_order`
  MODIFY COLUMN `order_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `pre_order_master_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_pre_order_item`
  MODIFY COLUMN `order_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `pre_order_master_id` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `order_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `off_order_item_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_pre_order_master` MODIFY COLUMN `pre_order_master_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `ipd_pre_order_progress_note`
  MODIFY COLUMN `progress_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `pre_order_master_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_pre_order_progress_note_item`
  MODIFY COLUMN `progress_note_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `progress_note_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `pre_order_master_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_progress_note` MODIFY COLUMN `progress_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `ipd_progress_note_item`
  MODIFY COLUMN `progress_note_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `progress_note_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_summary` MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `ipd_summary_2` MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `ipd_summary_approve_doctor` MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_summary_attending_doctor` MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_summary_dx`
  MODIFY COLUMN `id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `sort_index` INT(11) UNSIGNED NOT NULL DEFAULT 0;
ALTER TABLE `ipd_summary_external_cause`
  MODIFY COLUMN `external_cause_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_summary_other_diagnosis`
  MODIFY COLUMN `other_diagnosis_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_summary_post_admission_comorbidity`
  MODIFY COLUMN `post_admission_comorbidity_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_summary_pre_admission_comorbidity`
  MODIFY COLUMN `pre_admission_comorbidity_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_tmp_dlc`
  MODIFY COLUMN `dlc_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `dlc_order` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_tmp_focus`
  MODIFY COLUMN `focus_id` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `smp_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `subgroup` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `focus_order` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_tmp_goal`
  MODIFY COLUMN `goal_id` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `smp_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `subgroup` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `goal_order` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_tmp_group_smp`
  MODIFY COLUMN `smp_id` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `smp_group` INT(11) UNSIGNED DEFAULT NULL,
  MODIFY COLUMN `smp_order` INT(11) UNSIGNED DEFAULT NULL;
ALTER TABLE `ipd_tmp_intvt`
  MODIFY COLUMN `intvt_id` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `smp_id` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `subgroup` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `intvt_order` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_tmp_subgroup`
  MODIFY COLUMN `smp_id` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `subgroup` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `subgroup_order` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `ipd_vs_lt_arm` MODIFY COLUMN `lt_arm` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_vs_lt_leg` MODIFY COLUMN `lt_leg` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_vs_rt_arm` MODIFY COLUMN `rt_arm` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_vs_rt_leg` MODIFY COLUMN `rt_leg` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `ipd_vs_vital_sign`
  MODIFY COLUMN `vs_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `conscious_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `urine_amount` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `urine_duration` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `line_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `right_cha_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `left_cha_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `va_id` INT(1) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `mass_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `lt_arm` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `lt_leg` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `rt_arm` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `rt_leg` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `o2_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `tube_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `intake_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `output_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `lr_sta` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `lr_mem` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `breathing_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `avpu_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `gut_feeling_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `pops_other_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `kphis_spclty` MODIFY COLUMN `spclty_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `opd_er_allergy_history`
  MODIFY COLUMN `er_allergy_history_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `opd_er_bed` MODIFY COLUMN `opd_er_bed_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `opd_er_consult`
  MODIFY COLUMN `er_consult_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `opd_er_dch_type` MODIFY COLUMN `er_dch_type_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `opd_er_document_scan`
  MODIFY COLUMN `opd_er_document_scan_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `opd_er_dr_pe`
  MODIFY COLUMN `opd_er_pe_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `opd_er_emergency_level` MODIFY COLUMN `emergency_level_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `opd_er_focus_list`
  MODIFY COLUMN `fclist_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `smp_id` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `focus_id` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `opd_er_focus_list_goal_item`
  MODIFY COLUMN `fclist_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `fclist_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `goal_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `opd_er_focus_note`
  MODIFY COLUMN `fcnote_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `fclist_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `opd_er_focus_note_dlc_item`
  MODIFY COLUMN `fcnote_dlc_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `fcnote_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `dlc_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `opd_er_focus_note_intvt_item`
  MODIFY COLUMN `fcnote_intvt_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `fcnote_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `intvt_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `opd_er_io`
  MODIFY COLUMN `opd_er_io_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `opd_er_nurse_index_action`
  MODIFY COLUMN `action_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `opd_er_nurse_index_plan`
  MODIFY COLUMN `plan_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `order_item_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `opd_er_nurse_screening`
  MODIFY COLUMN `opd_er_screening_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `opd_er_order`
  MODIFY COLUMN `order_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `opd_er_order_item`
  MODIFY COLUMN `order_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `order_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `off_order_item_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `opd_er_order_master`
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `bedno` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `er_patient_status_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `er_dch_type_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `opd_er_order_progress_note`
  MODIFY COLUMN `progress_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;
ALTER TABLE `opd_er_order_progress_note_item`
  MODIFY COLUMN `progress_note_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `progress_note_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `opd_er_patient_status` MODIFY COLUMN `er_patient_status_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;
ALTER TABLE `opd_er_set_fast_track`
  MODIFY COLUMN `set_ft_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NULL DEFAULT NULL;
ALTER TABLE `opd_er_vs_vital_sign`
  MODIFY COLUMN `vs_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,
  MODIFY COLUMN `conscious_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `urine_amount` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `urine_duration` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `line_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `right_cha_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `left_cha_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `va_id` INT(1) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `mass_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `lt_arm` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `lt_leg` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `rt_arm` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `rt_leg` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `o2_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `tube_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `intake_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `output_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `lr_sta` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `lr_mem` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `breathing_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `avpu_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `gut_feeling_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  MODIFY COLUMN `pops_other_id` INT(11) UNSIGNED NULL DEFAULT NULL;