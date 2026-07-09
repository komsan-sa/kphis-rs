pub fn update_kphis(kphis: &str) -> Vec<String> {
    vec![
        // Add pre_admit_master table
        ["CREATE TABLE IF NOT EXISTS `",kphis,"`.`ipd_pre_admit_master` (\
            `pre_admit_master_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `vn` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `prev_an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`pre_admit_master_id`) USING BTREE,\
            UNIQUE INDEX `vn` (`vn`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;"].concat(),
        // Add opd_er_med_reconciliation table
        ["CREATE TABLE IF NOT EXISTS `",kphis,"`.`opd_er_med_reconciliation` (\
            `med_reconciliation_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,\
            `pharmacist` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,\
            `note` TEXT DEFAULT NULL,\
            `doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,\
            `med_reconciliation_datetime` DATETIME DEFAULT NULL,\
            `phamacist_confirm_datetime` DATETIME DEFAULT NULL,\
            `doctor_confirm_datetime` DATETIME DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`med_reconciliation_id`) USING BTREE,\
            INDEX `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;"].concat(),
        // Add opd_er_med_reconciliation_item table
        ["CREATE TABLE IF NOT EXISTS `",kphis,"`.`opd_er_med_reconciliation_item` (\
            `med_reconciliation_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `med_reconciliation_id` INT(11) UNSIGNED DEFAULT NULL,\
            `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,\
            `icode` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,\
            `med_name` VARCHAR(255) DEFAULT NULL,\
            `custom_med_name` VARCHAR(255) DEFAULT NULL,\
            `receive_from` VARCHAR(255) DEFAULT NULL,\
            `receive_date` DATE DEFAULT NULL,\
            `old_drugusage` TEXT DEFAULT NULL,\
            `changed_drugusage` TEXT DEFAULT NULL,\
            `receive_qty` INT(11) DEFAULT NULL,\
            `last_dose_taken_time` DATETIME DEFAULT NULL,\
            `last_dose_taken_remark` VARCHAR(255) DEFAULT NULL,\
            `use` VARCHAR(1) DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`med_reconciliation_item_id`) USING BTREE,\
            INDEX `med_reconciliation_id` (`med_reconciliation_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;"].concat(),
        // Add ipd_summary_note table
        ["CREATE TABLE IF NOT EXISTS `",kphis,"`.`ipd_summary_note` (\
            `summary_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `summary_id` INT(11) UNSIGNED NOT NULL,\
            `note` TEXT DEFAULT NULL,\
            `doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`summary_note_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;"].concat(),
        // Add ipd_vs_dipstick table
        ["CREATE TABLE IF NOT EXISTS `",kphis,"`.`ipd_vs_dipstick` (\
            `dipstick_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `dipstick_name` VARCHAR(50) DEFAULT NULL,\
            PRIMARY KEY (`dipstick_id`) USING BTREE\
        ) ENGINE=MyISAM AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        // Add ipd_vs_lr_moulding table
        ["CREATE TABLE IF NOT EXISTS `",kphis,"`.`ipd_vs_lr_moulding` (\
            `lr_moulding_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `lr_moulding_name` VARCHAR(50) DEFAULT NULL,\
            PRIMARY KEY (`lr_moulding_id`) USING BTREE\
        ) ENGINE=MyISAM AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        // Add ipd_vs_stage_of_change table
        ["CREATE TABLE IF NOT EXISTS `",kphis,"`.`ipd_vs_stage_of_change` (\
            `stage_of_change_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `stage_of_change_name` VARCHAR(50) DEFAULT NULL,\
            PRIMARY KEY (`stage_of_change_id`) USING BTREE\
        ) ENGINE=MyISAM AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
         // Add ipd_xray_read table
        ["CREATE TABLE IF NOT EXISTS `",kphis,"`.`ipd_xray_read` (\
            `xn` INT(11) NOT NULL DEFAULT 0,\
            `xray_read_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `xray_read_datetime` DATETIME NOT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`xn`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;"].concat(),
        // Add data to ipd_vs_dipstick
        ["INSERT INTO `",kphis,"`.`ipd_vs_dipstick` VALUES (1, '-ve') ON DUPLICATE KEY UPDATE dipstick_id=1, dipstick_name='-ve';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_dipstick` VALUES (2, 'Tr') ON DUPLICATE KEY UPDATE dipstick_id=2, dipstick_name='Tr';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_dipstick` VALUES (3, '+1') ON DUPLICATE KEY UPDATE dipstick_id=3, dipstick_name='+1';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_dipstick` VALUES (4, '+2') ON DUPLICATE KEY UPDATE dipstick_id=4, dipstick_name='+2';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_dipstick` VALUES (5, '+3') ON DUPLICATE KEY UPDATE dipstick_id=5, dipstick_name='+3';"].concat(),
        // Add data to ipd_vs_lr_moulding
        ["INSERT INTO `",kphis,"`.`ipd_vs_lr_moulding` VALUES (1, '0') ON DUPLICATE KEY UPDATE lr_moulding_id=1, lr_moulding_name='0';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_lr_moulding` VALUES (2, '+1') ON DUPLICATE KEY UPDATE lr_moulding_id=2, lr_moulding_name='+1';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_lr_moulding` VALUES (3, '+2') ON DUPLICATE KEY UPDATE lr_moulding_id=3, lr_moulding_name='+2';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_lr_moulding` VALUES (4, '+3') ON DUPLICATE KEY UPDATE lr_moulding_id=4, lr_moulding_name='+3';"].concat(),
        // Add data to ipd_vs_stage_of_change
        ["INSERT INTO `",kphis,"`.`ipd_vs_stage_of_change` VALUES (1, 'Pre-contemplation') ON DUPLICATE KEY UPDATE stage_of_change_id=1, stage_of_change_name='Pre-contemplation';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_stage_of_change` VALUES (2, 'Contemplation') ON DUPLICATE KEY UPDATE stage_of_change_id=2, stage_of_change_name='Contemplation';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_stage_of_change` VALUES (3, 'Determination') ON DUPLICATE KEY UPDATE stage_of_change_id=3, stage_of_change_name='Determination';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_stage_of_change` VALUES (4, 'Action') ON DUPLICATE KEY UPDATE stage_of_change_id=4, stage_of_change_name='Action';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_stage_of_change` VALUES (5, 'Maintenance') ON DUPLICATE KEY UPDATE stage_of_change_id=5, stage_of_change_name='Maintenance';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_stage_of_change` VALUES (6, 'Relapse') ON DUPLICATE KEY UPDATE stage_of_change_id=6, stage_of_change_name='Relapse';"].concat(),
        // Update OPD_ER_DOCUMENT resource
        ["INSERT INTO `",kphis,"`.`system_ac_resource` VALUES ('OPD_ER_DOCUMENT', NULL, 'DATA', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `resource`='OPD_ER_DOCUMENT',`resource_desc`=NULL, `resource_type`='DATA',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update OPD_ER_DOCUMENT resource
        ["INSERT INTO `",kphis,"`.`system_ac_resource` VALUES ('SYSTEM_AC_REPORT', NULL, 'DATA', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `resource`='SYSTEM_AC_REPORT',`resource_desc`=NULL, `resource_type`='DATA',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update OPD_ER_DOCUMENT_PRINT
        ["INSERT INTO `",kphis,"`.`system_ac_permission` VALUES ('OPD_ER_DOCUMENT_PRINT', 'OPD_ER_DOCUMENT', 'PRINT', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `permission`='OPD_ER_DOCUMENT_PRINT',`resource`='OPD_ER_DOCUMENT',`operation`='PRINT',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update permission for confirm-order-as
        ["INSERT INTO `",kphis,"`.`system_ac_permission` VALUES ('IPD_ORDER_CHECK', 'IPD_ORDER', 'CHECK', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0) \
          ON DUPLICATE KEY UPDATE `permission`='IPD_ORDER_CHECK',`resource`='IPD_ORDER',`operation`='CHECK',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;"].concat(),
        ["INSERT INTO `",kphis,"`.`system_ac_permission` VALUES ('OPD_ER_ORDER_CHECK', 'OPD_ER_ORDER', 'CHECK', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0) \
          ON DUPLICATE KEY UPDATE `permission`='OPD_ER_ORDER_CHECK',`resource`='OPD_ER_ORDER',`operation`='CHECK',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;"].concat(),
        // Update SYSTEM_AC_REPORT_ADD
        ["INSERT INTO `",kphis,"`.`system_ac_permission` VALUES ('SYSTEM_AC_REPORT_ADD', 'SYSTEM_AC_REPORT', 'ADD', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `permission`='SYSTEM_AC_REPORT_ADD',`resource`='SYSTEM_AC_REPORT',`operation`='ADD',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update SYSTEM_AC_REPORT_EDIT
        ["INSERT INTO `",kphis,"`.`system_ac_permission` VALUES ('SYSTEM_AC_REPORT_EDIT', 'SYSTEM_AC_REPORT', 'EDIT', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `permission`='SYSTEM_AC_REPORT_EDIT',`resource`='SYSTEM_AC_REPORT',`operation`='EDIT',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update SYSTEM_AC_REPORT_REMOVE
        ["INSERT INTO `",kphis,"`.`system_ac_permission` VALUES ('SYSTEM_AC_REPORT_REMOVE', 'SYSTEM_AC_REPORT', 'REMOVE', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `permission`='SYSTEM_AC_REPORT_REMOVE',`resource`='SYSTEM_AC_REPORT',`operation`='REMOVE',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update SYSTEM_AC_REPORT_VIEW
        ["INSERT INTO `",kphis,"`.`system_ac_permission` VALUES ('SYSTEM_AC_REPORT_VIEW', 'SYSTEM_AC_REPORT', 'VIEW', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `permission`='SYSTEM_AC_REPORT_VIEW',`resource`='SYSTEM_AC_REPORT',`operation`='VIEW',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update OPD_ER_DOCUMENT_PRINT
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('MEDICAL_RECORD', 'OPD_ER_DOCUMENT_PRINT', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `role`='MEDICAL_RECORD',`permission`='OPD_ER_DOCUMENT_PRINT',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update role-permission for confirm-order-as
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('MSO', 'IPD_ORDER_CHECK', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0) \
          ON DUPLICATE KEY UPDATE `role`='MSO',`permission`='IPD_ORDER_CHECK',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;"].concat(),
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('MSO', 'OPD_ER_ORDER_CHECK', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0) \
          ON DUPLICATE KEY UPDATE `role`='MSO',`permission`='OPD_ER_ORDER_CHECK',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;"].concat(),
        // Update role-permission for view index note for doctor
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('MSO', 'IPD_NURSE_INDEX_NOTE_VIEW', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0) \
          ON DUPLICATE KEY UPDATE `role`='MSO',`permission`='IPD_NURSE_INDEX_NOTE_VIEW',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;"].concat(),
        // Update role-permission for lab-read for doctor
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('MSO', 'IPD_LAB_READ_ADD', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0) \
          ON DUPLICATE KEY UPDATE `role`='MSO',`permission`='IPD_LAB_READ_ADD',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;"].concat(),
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('MSO', 'IPD_LAB_READ_EDIT', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0) \
          ON DUPLICATE KEY UPDATE `role`='MSO',`permission`='IPD_LAB_READ_EDIT',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;"].concat(),
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('MSO', 'IPD_LAB_READ_REMOVE', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0) \
          ON DUPLICATE KEY UPDATE `role`='MSO',`permission`='IPD_LAB_READ_REMOVE',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;"].concat(),
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('MSO', 'IPD_LAB_READ_VIEW', 'jommarn', '2025-04-06 00:00:00', 'jommarn', '2025-04-06 00:00:00', 0) \
          ON DUPLICATE KEY UPDATE `role`='MSO',`permission`='IPD_LAB_READ_VIEW',`create_user`='jommarn',`create_datetime`='2025-04-06 00:00:00',`update_user`='jommarn',`update_datetime`='2025-04-06 00:00:00',`version`=0;"].concat(),
        // Update SYSTEM_AC_REPORT_ADD for IT_ADMIN
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('IT_ADMIN', 'SYSTEM_AC_REPORT_ADD', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `role`='IT_ADMIN',`permission`='SYSTEM_AC_REPORT_ADD',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update SYSTEM_AC_REPORT_EDIT for IT_ADMIN
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('IT_ADMIN', 'SYSTEM_AC_REPORT_EDIT', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `role`='IT_ADMIN',`permission`='SYSTEM_AC_REPORT_EDIT',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update SYSTEM_AC_REPORT_REMOVE for IT_ADMIN
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('IT_ADMIN', 'SYSTEM_AC_REPORT_REMOVE', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `role`='IT_ADMIN',`permission`='SYSTEM_AC_REPORT_REMOVE',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update SYSTEM_AC_REPORT_VIEW for IT_ADMIN
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('IT_ADMIN', 'SYSTEM_AC_REPORT_VIEW', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `role`='IT_ADMIN',`permission`='SYSTEM_AC_REPORT_VIEW',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update SYSTEM_AC_REPORT_VIEW for nurse offing order
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('NURSE_ER_RN_EMT', 'IPD_ORDER_OFF', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `role`='NURSE_ER_RN_EMT',`permission`='IPD_ORDER_OFF',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('NURSE_IPD_RN_TN', 'IPD_ORDER_OFF', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `role`='NURSE_IPD_RN_TN',`permission`='IPD_ORDER_OFF',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        ["INSERT INTO `",kphis,"`.`system_ac_role_permission` VALUES ('NURSE_OPD_RN_TN', 'IPD_ORDER_OFF', 'jommarn', '2021-07-21 16:15:30', 'jommarn', '2021-07-21 16:15:30', 0) \
          ON DUPLICATE KEY UPDATE `role`='NURSE_OPD_RN_TN',`permission`='IPD_ORDER_OFF',`create_user`='jommarn',`create_datetime`='2021-07-21 16:15:30',`update_user`='jommarn',`update_datetime`='2021-07-21 16:15:30',`version`=0;"].concat(),
        // Update DOCTOR_INTERN role
        ["INSERT INTO `",kphis,"`.`system_ac_role` VALUES ('DOCTOR_INTERN', 'แพทย์ INTERN', 'DOCTOR', 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0) \
          ON DUPLICATE KEY UPDATE `role`='DOCTOR_INTERN',`role_desc`='แพทย์ INTERN',`parent_role`='DOCTOR',`create_user`='jommarn',`create_datetime`='2020-01-18 15:54:41',`update_user`='jommarn',`update_datetime`='2020-01-18 15:54:41',`version`=0;"].concat(),
        // Update order item types
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'note', 'Note', 1) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='note',`order_item_type_name`='Note',`display_order`=1;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'off', 'Off', 2) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='off',`order_item_type_name`='Off',`display_order`=2;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'lab', 'Lab', 3) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='lab',`order_item_type_name`='Lab',`display_order`=3;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'xray', 'X-Ray', 4) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='xray',`order_item_type_name`='X-Ray',`display_order`=4;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'ivfluid', 'IV Fluid', 5) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='ivfluid',`order_item_type_name`='IV Fluid',`display_order`=5;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'serial', 'Serial', 6) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='serial',`order_item_type_name`='Serial',`display_order`=6;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'record', 'Record', 7) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='record',`order_item_type_name`='Record',`display_order`=7;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'injection', 'Injection', 8) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='injection',`order_item_type_name`='Injection',`display_order`=8;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'med', 'Med', 9) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='med',`order_item_type_name`='Med',`display_order`=9;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'retain', 'Retain', 10) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='retain',`order_item_type_name`='Retain',`display_order`=10;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'other', 'Other', 11) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='other',`order_item_type_name`='Other',`display_order`=11;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'pharm', 'Pharmacist Notify', 12) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='pharm',`order_item_type_name`='Pharmacist Notify',`display_order`=12;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'discharge', 'Discharge', 13) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='discharge',`order_item_type_name`='Discharge',`display_order`=13;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('oneday', 'home-medication', 'Home Medication', 14) ON DUPLICATE KEY UPDATE `order_type`='oneday',`order_item_type`='home-medication',`order_item_type_name`='Home Medication',`display_order`=14;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('continuous', 'note', 'Note', 1) ON DUPLICATE KEY UPDATE `order_type`='continuous',`order_item_type`='note',`order_item_type_name`='Note',`display_order`=1;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('continuous', 'off', 'Off', 2) ON DUPLICATE KEY UPDATE `order_type`='continuous',`order_item_type`='off',`order_item_type_name`='Off',`display_order`=2;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('continuous', 'food', 'Food', 3) ON DUPLICATE KEY UPDATE `order_type`='continuous',`order_item_type`='food',`order_item_type_name`='Food',`display_order`=3;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('continuous', 'activity', 'Activity', 4) ON DUPLICATE KEY UPDATE `order_type`='continuous',`order_item_type`='activity',`order_item_type_name`='Activity',`display_order`=4;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('continuous', 'serial', 'Serial', 5) ON DUPLICATE KEY UPDATE `order_type`='continuous',`order_item_type`='serial',`order_item_type_name`='Serial',`display_order`=5;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('continuous', 'record', 'Record', 6) ON DUPLICATE KEY UPDATE `order_type`='continuous',`order_item_type`='record',`order_item_type_name`='Record',`display_order`=6;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('continuous', 'injection', 'Injection', 7) ON DUPLICATE KEY UPDATE `order_type`='continuous',`order_item_type`='injection',`order_item_type_name`='Injection',`display_order`=7;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('continuous', 'med', 'Med', 8) ON DUPLICATE KEY UPDATE `order_type`='continuous',`order_item_type`='med',`order_item_type_name`='Med',`display_order`=8;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_order_item_type` VALUES ('continuous', 'other', 'Other', 9) ON DUPLICATE KEY UPDATE `order_type`='continuous',`order_item_type`='other',`order_item_type_name`='Other',`display_order`=9;"].concat(),
        // Update progress note item types
        ["INSERT INTO `",kphis,"`.`ipd_progress_note_item_type` VALUES ('problem-list', 'Problem List', 1) ON DUPLICATE KEY UPDATE `progress_note_item_type`='problem-list',`progress_note_item_type_name`='Problem List',`display_order`=1;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_progress_note_item_type` VALUES ('note', 'Note', 2) ON DUPLICATE KEY UPDATE `progress_note_item_type`='note',`progress_note_item_type_name`='Note',`display_order`=2;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_progress_note_item_type` VALUES ('subjective', 'Subjective', 3) ON DUPLICATE KEY UPDATE `progress_note_item_type`='subjective',`progress_note_item_type_name`='Subjective',`display_order`=3;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_progress_note_item_type` VALUES ('objective', 'Objective', 4) ON DUPLICATE KEY UPDATE `progress_note_item_type`='objective',`progress_note_item_type_name`='Objectie',`display_order`=4;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_progress_note_item_type` VALUES ('assessment', 'Assessment', 5) ON DUPLICATE KEY UPDATE `progress_note_item_type`='assessment',`progress_note_item_type_name`='Assessment',`display_order`=5;"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_progress_note_item_type` VALUES ('plan', 'Plan', 6) ON DUPLICATE KEY UPDATE `progress_note_item_type`='plan',`progress_note_item_type_name`='Plan',`display_order`=6;"].concat(),
        // Update opd_er_patient_status
        ["INSERT INTO `",kphis,"`.`opd_er_patient_status` VALUES (9, 'รอ ATK', 'รอตรวจเขื้อ', 5, 'jommarn', '2021-11-04 12:30:42', 'jommarn', '2021-11-04 12:30:42', 1) ON DUPLICATE KEY UPDATE `er_patient_status_id`=9,`er_patient_status_name`='รอ ATK',`er_patient_status_name_pt`='รอตรวจเขื้อ',`display_order`=5,`create_user`='jommarn',`create_datetime`='2021-11-04 12:30:42',`update_user`='jommarn',`update_datetime`='2021-11-04 12:30:42',`version`=1;"].concat(),
        ["INSERT INTO `",kphis,"`.`opd_er_patient_status` VALUES (10, 'Admit แล้ว', 'นอน รพ.แล้ว', 10, 'jommarn', '2021-11-04 12:30:42', 'jommarn', '2021-11-04 12:30:42', 1) ON DUPLICATE KEY UPDATE `er_patient_status_id`=10,`er_patient_status_name`='Admit แล้ว',`er_patient_status_name_pt`='นอน รพ.แล้ว',`display_order`=10,`create_user`='jommarn',`create_datetime`='2021-11-04 12:30:42',`update_user`='jommarn',`update_datetime`='2021-11-04 12:30:42',`version`=1;"].concat(),
        // Add Unresponsive to AVPU score
        ["INSERT INTO `",kphis,"`.`ipd_vs_avpu` VALUES (4, 'Unresponsive') ON DUPLICATE KEY UPDATE `avpu_id`=4,`avpu_name`='Unresponsive';"].concat(),
        // Add SRM and ARM to ipd_vs_lr_mem
        ["INSERT INTO `",kphis,"`.`ipd_vs_lr_mem` VALUES (4, 'SRM') ON DUPLICATE KEY UPDATE lr_mem_id=4, lr_mem_name='SRM';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_lr_mem` VALUES (5, 'ARM') ON DUPLICATE KEY UPDATE lr_mem_id=5, lr_mem_name='ARM';"].concat(),
        // Add crt and band for EWS score (IPD), partograph parameters
        ["ALTER TABLE `",kphis,"`.`ipd_vs_vital_sign` \
            ROW_FORMAT = DYNAMIC,\
            MODIFY COLUMN `braden` TEXT DEFAULT NULL,\
            ADD COLUMN IF NOT EXISTS `action_id` INT(11) UNSIGNED DEFAULT NULL AFTER `vs_id`,\
            ADD COLUMN IF NOT EXISTS `sat_room_air` INT(3) UNSIGNED DEFAULT NULL AFTER `sat`,\
            ADD COLUMN IF NOT EXISTS `crt` INT(2) DEFAULT NULL AFTER `pleak_flow`,\
            ADD COLUMN IF NOT EXISTS `band` INT(3) DEFAULT NULL AFTER `crt`,\
            ADD COLUMN IF NOT EXISTS `lr_pos` VARCHAR(3) DEFAULT NULL AFTER `band`,\
            ADD COLUMN IF NOT EXISTS `lr_moulding` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_pos`,\
            ADD COLUMN IF NOT EXISTS `lr_oxytocin_unit` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_moulding`,\
            ADD COLUMN IF NOT EXISTS `lr_oxytocin_rate` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_oxytocin_unit`,\
            ADD COLUMN IF NOT EXISTS `lr_urine_vol` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_oxytocin_rate`,\
            ADD COLUMN IF NOT EXISTS `urine_protein` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_urine_vol`,\
            ADD COLUMN IF NOT EXISTS `urine_sugar` INT(11) UNSIGNED DEFAULT NULL AFTER `urine_protein`,\
            ADD COLUMN IF NOT EXISTS `diet` VARCHAR(20) DEFAULT NULL AFTER `urine_sugar`,\
            ADD COLUMN IF NOT EXISTS `barthel_index` TEXT DEFAULT NULL AFTER `diet`,\
            ADD COLUMN IF NOT EXISTS `aggression_oas` TEXT DEFAULT NULL AFTER `barthel_index`,\
            ADD COLUMN IF NOT EXISTS `alcohol_ciwa` TEXT DEFAULT NULL AFTER `aggression_oas`,\
            ADD COLUMN IF NOT EXISTS `alcohol_aws` TEXT DEFAULT NULL AFTER `alcohol_ciwa`,\
            ADD COLUMN IF NOT EXISTS `amphetamine_awq` TEXT DEFAULT NULL AFTER `alcohol_aws`,\
            ADD COLUMN IF NOT EXISTS `motivation_scale` TINYINT(2) UNSIGNED DEFAULT NULL AFTER `amphetamine_awq`,\
            ADD COLUMN IF NOT EXISTS `craving_scale` TINYINT(2) UNSIGNED DEFAULT NULL AFTER `motivation_scale`,\
            ADD COLUMN IF NOT EXISTS `stage_of_change_id` INT(11) UNSIGNED DEFAULT NULL AFTER `craving_scale`,\
            ADD COLUMN IF NOT EXISTS `depress_2q` TEXT DEFAULT NULL AFTER `stage_of_change_id`,\
            ADD COLUMN IF NOT EXISTS `depress_9q` TEXT DEFAULT NULL AFTER `depress_2q`,\
            ADD COLUMN IF NOT EXISTS `suicide_8q` TEXT DEFAULT NULL AFTER `depress_9q`;"].concat(),
        // Add crt and band for EWS score (OPD-ER), partograph parameters and change type of OPD-ER dtx to the same as IPD
        ["ALTER TABLE `",kphis,"`.`opd_er_vs_vital_sign` \
            ROW_FORMAT = DYNAMIC,
            MODIFY COLUMN `dtx` VARCHAR(10) DEFAULT NULL,\
            MODIFY COLUMN `braden` TEXT DEFAULT NULL,\
            ADD COLUMN IF NOT EXISTS `action_id` INT(11) UNSIGNED DEFAULT NULL AFTER `vs_id`,\
            ADD COLUMN IF NOT EXISTS `sat_room_air` INT(3) UNSIGNED DEFAULT NULL AFTER `sat`,\
            ADD COLUMN IF NOT EXISTS `crt` INT(2) DEFAULT NULL AFTER `pleak_flow`,\
            ADD COLUMN IF NOT EXISTS `band` INT(3) DEFAULT NULL AFTER `crt`,\
            ADD COLUMN IF NOT EXISTS `lr_pos` VARCHAR(3) DEFAULT NULL AFTER `band`,\
            ADD COLUMN IF NOT EXISTS `lr_moulding` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_pos`,\
            ADD COLUMN IF NOT EXISTS `lr_oxytocin_unit` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_moulding`,\
            ADD COLUMN IF NOT EXISTS `lr_oxytocin_rate` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_oxytocin_unit`,\
            ADD COLUMN IF NOT EXISTS `lr_urine_vol` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_oxytocin_rate`,\
            ADD COLUMN IF NOT EXISTS `urine_protein` INT(11) UNSIGNED DEFAULT NULL AFTER `lr_urine_vol`,\
            ADD COLUMN IF NOT EXISTS `urine_sugar` INT(11) UNSIGNED DEFAULT NULL AFTER `urine_protein`,\
            ADD COLUMN IF NOT EXISTS `diet` VARCHAR(20) DEFAULT NULL AFTER `urine_sugar`,\
            ADD COLUMN IF NOT EXISTS `barthel_index` TEXT DEFAULT NULL AFTER `diet`,\
            ADD COLUMN IF NOT EXISTS `aggression_oas` TEXT DEFAULT NULL AFTER `barthel_index`,\
            ADD COLUMN IF NOT EXISTS `alcohol_ciwa` TEXT DEFAULT NULL AFTER `aggression_oas`,\
            ADD COLUMN IF NOT EXISTS `alcohol_aws` TEXT DEFAULT NULL AFTER `alcohol_ciwa`,\
            ADD COLUMN IF NOT EXISTS `amphetamine_awq` TEXT DEFAULT NULL AFTER `alcohol_aws`,\
            ADD COLUMN IF NOT EXISTS `motivation_scale` TINYINT(2) UNSIGNED DEFAULT NULL AFTER `amphetamine_awq`,\
            ADD COLUMN IF NOT EXISTS `craving_scale` TINYINT(2) UNSIGNED DEFAULT NULL AFTER `motivation_scale`,\
            ADD COLUMN IF NOT EXISTS `stage_of_change_id` INT(11) UNSIGNED DEFAULT NULL AFTER `craving_scale`,\
            ADD COLUMN IF NOT EXISTS `depress_2q` TEXT DEFAULT NULL AFTER `stage_of_change_id`,\
            ADD COLUMN IF NOT EXISTS `depress_9q` TEXT DEFAULT NULL AFTER `depress_2q`,\
            ADD COLUMN IF NOT EXISTS `suicide_8q` TEXT DEFAULT NULL AFTER `depress_9q`;"].concat(),
        // Add pharmacist_check
        ["ALTER TABLE `",kphis,"`.`ipd_pre_order` \
            ADD COLUMN IF NOT EXISTS `pharmacist_check` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL AFTER `pharmacist_accept_time`,\
            ADD COLUMN IF NOT EXISTS `pharmacist_check_time` DATETIME DEFAULT NULL AFTER `pharmacist_check`;"].concat(),
        // Add nuser_order_as and doctor_confirm_time (IPD)
        ["ALTER TABLE `",kphis,"`.`ipd_order` \
            ADD COLUMN IF NOT EXISTS `nurse_order_as` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL AFTER `order_confirm`,\
            ADD COLUMN IF NOT EXISTS `doctor_confirm_time` DATETIME NULL DEFAULT NULL AFTER `nurse_order_as`,\
            ADD COLUMN IF NOT EXISTS `pharmacist_check` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL AFTER `pharmacist_accept_time`,\
            ADD COLUMN IF NOT EXISTS `pharmacist_check_time` DATETIME DEFAULT NULL AFTER `pharmacist_check`;"].concat(),
        // Add nuser_order_as and doctor_confirm_time (OPD-ER)
        ["ALTER TABLE `",kphis,"`.`opd_er_order` \
            ADD COLUMN IF NOT EXISTS `nurse_order_as` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL AFTER `order_confirm`,\
            ADD COLUMN IF NOT EXISTS `doctor_confirm_time` DATETIME NULL DEFAULT NULL AFTER `nurse_order_as`,\
            ADD COLUMN IF NOT EXISTS `pharmacist_check` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL AFTER `pharmacist_accept_time`,\
            ADD COLUMN IF NOT EXISTS `pharmacist_check_time` DATETIME DEFAULT NULL AFTER `pharmacist_check`,\
            ADD COLUMN IF NOT EXISTS `pre_order_id` INT(11) UNSIGNED DEFAULT NULL AFTER `pharmacist_order_status`,\
            ADD COLUMN IF NOT EXISTS `pre_order_date` DATE DEFAULT NULL AFTER `pre_order_id`,\
            ADD COLUMN IF NOT EXISTS `pre_order_time` TIME DEFAULT NULL AFTER `pre_order_date`;"].concat(),
        // Add nurse_assign, first_qty, qty, due_doctor, due_doctor_note, due_pharm, due_pharm_note to ipd_order_item
        ["ALTER TABLE `",kphis,"`.`ipd_order_item` \
            ADD COLUMN IF NOT EXISTS `nurse_assign` VARCHAR(20) DEFAULT NULL AFTER `med_reconciliation_item_id`,\
            ADD COLUMN IF NOT EXISTS `first_qty` INT(11) NULL DEFAULT NULL AFTER `nurse_assign`,\
            ADD COLUMN IF NOT EXISTS `qty` INT(11) NULL DEFAULT NULL AFTER `first_qty`,\
            ADD COLUMN IF NOT EXISTS `due_doctor` VARCHAR(1) DEFAULT NULL AFTER `qty`,\
            ADD COLUMN IF NOT EXISTS `due_doctor_note` TEXT DEFAULT NULL AFTER `due_doctor`,\
            ADD COLUMN IF NOT EXISTS `due_pharm` VARCHAR(1) DEFAULT NULL AFTER `due_doctor_note`,\
            ADD COLUMN IF NOT EXISTS `due_pharm_note` TEXT DEFAULT NULL AFTER `due_pharm`;"].concat(),
        // Add nurse_assign, med_reconciliation_item_id, first_qty, qty, due_doctor, due_doctor_note, due_pharm, due_pharm_note to opd_er_order_item
        ["ALTER TABLE `",kphis,"`.`opd_er_order_item` \
            ADD COLUMN IF NOT EXISTS `med_reconciliation_item_id` INT(11) UNSIGNED DEFAULT NULL AFTER `icode`,\
            ADD COLUMN IF NOT EXISTS `nurse_assign` VARCHAR(20) DEFAULT NULL AFTER `med_reconciliation_item_id`,\
            ADD COLUMN IF NOT EXISTS `first_qty` INT(11) NULL DEFAULT NULL AFTER `nurse_assign`,\
            ADD COLUMN IF NOT EXISTS `qty` INT(11) NULL DEFAULT NULL AFTER `first_qty`,\
            ADD COLUMN IF NOT EXISTS `due_doctor` VARCHAR(1) DEFAULT NULL AFTER `qty`,\
            ADD COLUMN IF NOT EXISTS `due_doctor_note` TEXT DEFAULT NULL AFTER `due_doctor`,\
            ADD COLUMN IF NOT EXISTS `due_pharm` VARCHAR(1) DEFAULT NULL AFTER `due_doctor_note`,\
            ADD COLUMN IF NOT EXISTS `due_pharm_note` TEXT DEFAULT NULL AFTER `due_pharm`;"].concat(),
        // Add progress_note_enter_datetime to opd_er_order_progress_note
        ["ALTER TABLE `",kphis,"`.`opd_er_order_progress_note` \
            ADD COLUMN IF NOT EXISTS `progress_note_enter_datetime` DATETIME DEFAULT NULL AFTER `progress_note_doctor`,\
            ADD COLUMN IF NOT EXISTS `pre_order_progress_note_id` INT(11) UNSIGNED DEFAULT NULL AFTER `progress_note_enter_datetime`,\
            ADD COLUMN IF NOT EXISTS `pre_order_progress_note_date` DATE DEFAULT NULL AFTER `pre_order_progress_note_id`,\
            ADD COLUMN IF NOT EXISTS `pre_order_progress_note_time` TIME DEFAULT NULL AFTER `pre_order_progress_note_date`;"].concat(),
        // Add progress_note_item_detail_2 to opd_er_order_progress_note_item
        ["ALTER TABLE `",kphis,"`.`opd_er_order_progress_note_item` ADD COLUMN IF NOT EXISTS `progress_note_item_detail_2` TEXT DEFAULT NULL AFTER `progress_note_item_detail`;"].concat(),
        // Change size of fcnote_patient_type to 2 (IPD)
        ["ALTER TABLE `",kphis,"`.`ipd_focus_note` MODIFY COLUMN `fcnote_patient_type` VARCHAR(2) DEFAULT NULL;"].concat(),
        // Change size of fcnote_patient_type to 2 (OPD-ER)
        ["ALTER TABLE `",kphis,"`.`opd_er_focus_note` MODIFY COLUMN `fcnote_patient_type` VARCHAR(2) DEFAULT NULL;"].concat(),
        // Add coder to ipd_summary_2
        ["ALTER TABLE `",kphis,"`.`ipd_summary_2` \
            MODIFY COLUMN `hospital_refer` varchar(9) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL AFTER `discharge_type`,\
            ADD COLUMN IF NOT EXISTS `special_other` VARCHAR(1) DEFAULT NULL AFTER `non_or_other_text`,\
            ADD COLUMN IF NOT EXISTS `special_other_text` TEXT DEFAULT NULL AFTER `special_other`,\
            ADD COLUMN IF NOT EXISTS `coder_name` VARCHAR(50) DEFAULT NULL AFTER `hospital_refer`,\
            ADD COLUMN IF NOT EXISTS `principal_diagnosis_code` VARCHAR(7) DEFAULT NULL AFTER `coder_name`,\
            ADD COLUMN IF NOT EXISTS `pre_admission_comorbidity_codes` TEXT DEFAULT NULL AFTER `principal_diagnosis_code`,\
            ADD COLUMN IF NOT EXISTS `post_admission_comorbidity_codes` TEXT DEFAULT NULL AFTER `pre_admission_comorbidity_codes`,\
            ADD COLUMN IF NOT EXISTS `other_diagnosis_codes` TEXT DEFAULT NULL AFTER `post_admission_comorbidity_codes`,\
            ADD COLUMN IF NOT EXISTS `external_cause_codes` TEXT DEFAULT NULL AFTER `other_diagnosis_codes`,\
            ADD COLUMN IF NOT EXISTS `main_procedure_code` VARCHAR(7) DEFAULT NULL AFTER `external_cause_codes`,\
            ADD COLUMN IF NOT EXISTS `other_procedure_codes` TEXT DEFAULT NULL AFTER `main_procedure_code`,\
            ADD COLUMN IF NOT EXISTS `status` VARCHAR(7) DEFAULT NULL AFTER `other_procedure_codes`,\
            ADD INDEX IF NOT EXISTS `status` (`status`) USING BTREE;"].concat(),
        // Change ipd_dr_admission_note to be able to create with pre-admit
        ["ALTER TABLE `",kphis,"`.`ipd_dr_admission_note` \
            ROW_FORMAT = DYNAMIC,\
            MODIFY COLUMN `receiver_medication_date` DATE DEFAULT NULL,\
            MODIFY COLUMN `receiver_medication_time` TIME DEFAULT NULL,\
            MODIFY COLUMN `anc` VARCHAR(200) DEFAULT NULL,\
            MODIFY COLUMN `braden_scale` TEXT DEFAULT NULL,\
            ADD COLUMN IF NOT EXISTS `nurse_licenseno` VARCHAR(150) DEFAULT NULL AFTER `nurse_pos`,\
            ADD COLUMN IF NOT EXISTS `mem_ruptured_hours` SMALLINT(5) UNSIGNED DEFAULT NULL AFTER `doc_pos`,\
            ADD COLUMN IF NOT EXISTS `lr_back_fetus` VARCHAR(20) DEFAULT NULL AFTER `mem_ruptured_hours`,\
            ADD COLUMN IF NOT EXISTS `lr_presentation` VARCHAR(20) DEFAULT NULL AFTER `lr_back_fetus`,\
            ADD COLUMN IF NOT EXISTS `lr_engagement` VARCHAR(1) DEFAULT NULL AFTER `lr_presentation`,\
            ADD COLUMN IF NOT EXISTS `lr_prominence` VARCHAR(20) DEFAULT NULL AFTER `lr_engagement`,\
            ADD COLUMN IF NOT EXISTS `lr_attitude` VARCHAR(20) DEFAULT NULL AFTER `lr_prominence`,\
            ADD COLUMN IF NOT EXISTS `lr_fhr` SMALLINT(5) UNSIGNED DEFAULT NULL AFTER `lr_attitude`,\
            ADD COLUMN IF NOT EXISTS `lr_fhr_irrigular` VARCHAR(1) DEFAULT NULL AFTER `lr_fhr`,\
            ADD COLUMN IF NOT EXISTS `lr_efw` SMALLINT(5) UNSIGNED DEFAULT NULL AFTER `lr_fhr_irrigular`,\
            ADD COLUMN IF NOT EXISTS `lr_interval` VARCHAR(20) DEFAULT NULL AFTER `lr_efw`,\
            ADD COLUMN IF NOT EXISTS `lr_duration` TINYINT(3) UNSIGNED DEFAULT NULL AFTER `lr_interval`,\
            ADD COLUMN IF NOT EXISTS `lr_intensity` VARCHAR(20) DEFAULT NULL AFTER `lr_duration`,\
            ADD COLUMN IF NOT EXISTS `lr_pelvic_diagonal` Decimal(3,1) DEFAULT NULL AFTER `lr_intensity`,\
            ADD COLUMN IF NOT EXISTS `lr_pelvic_interspinous` Decimal(3,1) DEFAULT NULL AFTER `lr_pelvic_diagonal`,\
            ADD COLUMN IF NOT EXISTS `lr_pelvic_sidewall` VARCHAR(20) DEFAULT NULL AFTER `lr_pelvic_interspinous`,\
            ADD COLUMN IF NOT EXISTS `lr_ischeal_spine` VARCHAR(20) DEFAULT NULL AFTER `lr_pelvic_sidewall`,\
            ADD COLUMN IF NOT EXISTS `lr_sacral_curve` VARCHAR(20) DEFAULT NULL AFTER `lr_ischeal_spine`,\
            ADD COLUMN IF NOT EXISTS `lr_pubic_angle` TINYINT(3) UNSIGNED DEFAULT NULL AFTER `lr_sacral_curve`,\
            ADD COLUMN IF NOT EXISTS `lr_pelvic_ok` VARCHAR(1) DEFAULT NULL AFTER `lr_pubic_angle`,\
            ADD COLUMN IF NOT EXISTS `lr_cx_dilate` TINYINT(3) UNSIGNED DEFAULT NULL AFTER `lr_pelvic_ok`,\
            ADD COLUMN IF NOT EXISTS `lr_cx_efface` TINYINT(3) UNSIGNED DEFAULT NULL AFTER `lr_cx_dilate`,\
            ADD COLUMN IF NOT EXISTS `lr_cx_station` TINYINT(3) DEFAULT NULL AFTER `lr_cx_efface`,\
            ADD COLUMN IF NOT EXISTS `lr_cx_position` VARCHAR(20) DEFAULT NULL AFTER `lr_cx_station`,\
            ADD COLUMN IF NOT EXISTS `lr_cx_consistency` VARCHAR(20) DEFAULT NULL AFTER `lr_cx_position`,\
            ADD COLUMN IF NOT EXISTS `lr_cx_bishop` TINYINT(3) UNSIGNED DEFAULT NULL AFTER `lr_cx_consistency`,\
            ADD COLUMN IF NOT EXISTS `lr_cx_ok` VARCHAR(1) DEFAULT NULL AFTER `lr_cx_bishop`,\
            ADD COLUMN IF NOT EXISTS `lr_membrane` VARCHAR(20) DEFAULT NULL AFTER `lr_cx_ok`,\
            ADD COLUMN IF NOT EXISTS `lr_amniotic_color` VARCHAR(10) DEFAULT NULL AFTER `lr_membrane`,\
            ADD COLUMN IF NOT EXISTS `lr_amniotic_smell` VARCHAR(10) DEFAULT NULL AFTER `lr_amniotic_color`,\
            ADD COLUMN IF NOT EXISTS `ros_eent` VARCHAR(200) DEFAULT NULL AFTER `lr_amniotic_smell`,\
            ADD COLUMN IF NOT EXISTS `ros_neuro` VARCHAR(200) DEFAULT NULL AFTER `ros_eent`,\
            ADD COLUMN IF NOT EXISTS `ros_lung` VARCHAR(200) DEFAULT NULL AFTER `ros_neuro`,\
            ADD COLUMN IF NOT EXISTS `ros_tb` VARCHAR(200) DEFAULT NULL AFTER `ros_lung`,\
            ADD COLUMN IF NOT EXISTS `ros_ht` VARCHAR(200) DEFAULT NULL AFTER `ros_tb`,\
            ADD COLUMN IF NOT EXISTS `ros_heart` VARCHAR(200) DEFAULT NULL AFTER `ros_ht`,\
            ADD COLUMN IF NOT EXISTS `ros_liver` VARCHAR(200) DEFAULT NULL AFTER `ros_heart`,\
            ADD COLUMN IF NOT EXISTS `ros_gi` VARCHAR(200) DEFAULT NULL AFTER `ros_liver`,\
            ADD COLUMN IF NOT EXISTS `ros_endocrine` VARCHAR(200) DEFAULT NULL AFTER `ros_gi`,\
            ADD COLUMN IF NOT EXISTS `ros_kidney` VARCHAR(200) DEFAULT NULL AFTER `ros_endocrine`,\
            ADD COLUMN IF NOT EXISTS `ros_tumour` VARCHAR(200) DEFAULT NULL AFTER `ros_kidney`,\
            ADD COLUMN IF NOT EXISTS `ros_hemato` VARCHAR(200) DEFAULT NULL AFTER `ros_tumour`,\
            ADD COLUMN IF NOT EXISTS `ros_rheumato` VARCHAR(200) DEFAULT NULL AFTER `ros_hemato`,\
            ADD COLUMN IF NOT EXISTS `ros_psychia` VARCHAR(200) DEFAULT NULL AFTER `ros_rheumato`,\
            ADD COLUMN IF NOT EXISTS `ros_other` VARCHAR(200) DEFAULT NULL AFTER `ros_psychia`,\
            ADD COLUMN IF NOT EXISTS `addict` VARCHAR(20) DEFAULT NULL AFTER `ros_other`,\
            ADD COLUMN IF NOT EXISTS `addict_assist` TEXT DEFAULT NULL AFTER `addict`,\
            ADD COLUMN IF NOT EXISTS `addict_inj` VARCHAR(1) DEFAULT NULL AFTER `addict_assist`,\
            ADD COLUMN IF NOT EXISTS `addict_inj_often` VARCHAR(1) DEFAULT NULL AFTER `addict_inj`,\
            ADD COLUMN IF NOT EXISTS `amphetamine_awq` TEXT DEFAULT NULL AFTER `addict_inj_often`,\
            ADD COLUMN IF NOT EXISTS `aggression_oas` TEXT DEFAULT NULL AFTER `amphetamine_awq`,\
            ADD COLUMN IF NOT EXISTS `motivation_scale` TINYINT(2) UNSIGNED DEFAULT NULL AFTER `aggression_oas`,\
            ADD COLUMN IF NOT EXISTS `craving_scale` TINYINT(2) UNSIGNED DEFAULT NULL AFTER `motivation_scale`,\
            ADD COLUMN IF NOT EXISTS `stage_of_change_id` INT(11) UNSIGNED DEFAULT NULL AFTER `craving_scale`,\
            ADD COLUMN IF NOT EXISTS `alcohol_audit` TEXT DEFAULT NULL AFTER `stage_of_change_id`,\
            ADD COLUMN IF NOT EXISTS `alcohol_aws` TEXT DEFAULT NULL AFTER `alcohol_audit`,\
            ADD COLUMN IF NOT EXISTS `alcohol_ciwa` TEXT DEFAULT NULL AFTER `alcohol_aws`,\
            ADD COLUMN IF NOT EXISTS `depress_2q` TEXT DEFAULT NULL AFTER `alcohol_ciwa`,\
            ADD COLUMN IF NOT EXISTS `depress_9q` TEXT DEFAULT NULL AFTER `depress_2q`,\
            ADD COLUMN IF NOT EXISTS `depress_cdi` TEXT DEFAULT NULL AFTER `depress_9q`,\
            ADD COLUMN IF NOT EXISTS `depress_cesd` TEXT DEFAULT NULL AFTER `depress_cdi`,\
            ADD COLUMN IF NOT EXISTS `depress_phqa` TEXT DEFAULT NULL AFTER `depress_cesd`,\
            ADD COLUMN IF NOT EXISTS `nicotin_ftnd` TEXT DEFAULT NULL AFTER `depress_phqa`,\
            ADD COLUMN IF NOT EXISTS `ptsd_screen` TEXT DEFAULT NULL AFTER `nicotin_ftnd`,\
            ADD COLUMN IF NOT EXISTS `ptsd_pisces` TEXT DEFAULT NULL AFTER `ptsd_screen`,\
            ADD COLUMN IF NOT EXISTS `ptsd_cries` TEXT DEFAULT NULL AFTER `ptsd_pisces`,\
            ADD COLUMN IF NOT EXISTS `suicide_8q` TEXT DEFAULT NULL AFTER `ptsd_cries`,\
            ADD COLUMN IF NOT EXISTS `stress_st5` TEXT DEFAULT NULL AFTER `suicide_8q`;"].concat(),
        // Add cc, hpi, vs, informants to ipd_nurse_admission_note
        ["ALTER TABLE `",kphis,"`.`ipd_nurse_admission_note` \
            ADD COLUMN IF NOT EXISTS `info_patient` VARCHAR(1) DEFAULT NULL AFTER `an`,\
            ADD COLUMN IF NOT EXISTS `info_parent` VARCHAR(1) DEFAULT NULL AFTER `info_patient`,\
            ADD COLUMN IF NOT EXISTS `info_spouse` VARCHAR(1) DEFAULT NULL AFTER `info_parent`,\
            ADD COLUMN IF NOT EXISTS `info_child` VARCHAR(1) DEFAULT NULL AFTER `info_spouse`,\
            ADD COLUMN IF NOT EXISTS `info_relatives` VARCHAR(1) DEFAULT NULL AFTER `info_child`,\
            ADD COLUMN IF NOT EXISTS `info_sender` VARCHAR(1) DEFAULT NULL AFTER `info_relatives`,\
            ADD COLUMN IF NOT EXISTS `chief_complaints` TEXT DEFAULT NULL AFTER `info_sender`,\
            ADD COLUMN IF NOT EXISTS `medical_history` TEXT DEFAULT NULL AFTER `chief_complaints`,\
            ADD COLUMN IF NOT EXISTS `vs_admit` TEXT DEFAULT NULL AFTER `medical_history`;"].concat(),
        // Add ipd nurse index check
        ["ALTER TABLE `",kphis,"`.`ipd_nurse_index_action` \
            ADD COLUMN IF NOT EXISTS `check_datetime` DATETIME DEFAULT NULL AFTER `an`,\
            ADD COLUMN IF NOT EXISTS `check_person` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL AFTER `check_datetime`;"].concat(),
        // Add opd-er nurse index check
        ["ALTER TABLE `",kphis,"`.`opd_er_nurse_index_action` \
            ADD COLUMN IF NOT EXISTS `check_datetime` DATETIME DEFAULT NULL AFTER `opd_er_order_master_id`,\
            ADD COLUMN IF NOT EXISTS `check_person` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL AFTER `check_datetime`;"].concat(),
        // Add dosage and status
        ["ALTER TABLE `",kphis,"`.`kphis_drug_use_duration` \
            ADD COLUMN IF NOT EXISTS `usage` TEXT DEFAULT NULL AFTER `icode`,\
            ADD COLUMN IF NOT EXISTS `status` VARCHAR(1) DEFAULT NULL AFTER `exceed_duration3_color`,\
            ADD COLUMN IF NOT EXISTS `monitor` TEXT DEFAULT NULL AFTER `status`,\
            ADD COLUMN IF NOT EXISTS `monitor_count` TINYINT(3) UNSIGNED DEFAULT NULL AFTER `monitor`,\
            ADD COLUMN IF NOT EXISTS `monitor_duration` INT(11) UNSIGNED DEFAULT NULL AFTER `monitor_count`,\
            ADD COLUMN IF NOT EXISTS `monitor_status` VARCHAR(1) DEFAULT NULL AFTER `monitor_duration`,\
            ADD COLUMN IF NOT EXISTS `info` TEXT DEFAULT NULL AFTER `monitor_status`,\
            ADD COLUMN IF NOT EXISTS `info_status` VARCHAR(1) DEFAULT NULL AFTER `info`,\
            ADD PRIMARY KEY IF NOT EXISTS `icode` (`icode`) USING BTREE,\
            ADD INDEX IF NOT EXISTS `status` (`status`) USING BTREE,\
            ADD INDEX IF NOT EXISTS `monitor_status` (`monitor_status`) USING BTREE,\
            ADD INDEX IF NOT EXISTS `info_status` (`info_status`) USING BTREE;"].concat(),
        // Change O2 name
        ["INSERT INTO `",kphis,"`.`ipd_vs_o2` VALUES (2, 'Mask c bag') ON DUPLICATE KEY UPDATE o2_id=2, o2_name='Mask c bag';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_o2` VALUES (3, 'Collar') ON DUPLICATE KEY UPDATE o2_id=3, o2_name='Collar';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_o2` VALUES (4, 'HFNC') ON DUPLICATE KEY UPDATE o2_id=4, o2_name='HFNC';"].concat(),
        ["INSERT INTO `",kphis,"`.`ipd_vs_o2` VALUES (8, 'Tube') ON DUPLICATE KEY UPDATE o2_id=8, o2_name='Tube';"].concat(),
        // ----------------------------------------
        // Add constraint to prevent duplicate data
        // ----------------------------------------
        ["ALTER TABLE `",kphis,"`.`ipd_focus_list_goal_item` ADD CONSTRAINT `fclist_goal` UNIQUE IF NOT EXISTS (`fclist_id`,`goal_id`) USING BTREE;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_focus_list_goal_item` ADD CONSTRAINT `fclist_goal` UNIQUE IF NOT EXISTS (`fclist_id`,`goal_id`) USING BTREE;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_focus_note_intvt_item` ADD CONSTRAINT `fcnote_intvt` UNIQUE IF NOT EXISTS (`fcnote_id`,`intvt_id`) USING BTREE;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_focus_note_intvt_item` ADD CONSTRAINT `fcnote_intvt` UNIQUE IF NOT EXISTS (`fcnote_id`,`intvt_id`) USING BTREE;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_focus_note_dlc_item` ADD CONSTRAINT `fcnote_dlc` UNIQUE IF NOT EXISTS (`fcnote_id`,`dlc_id`) USING BTREE;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_focus_note_dlc_item` ADD CONSTRAINT `fcnote_dlc` UNIQUE IF NOT EXISTS (`fcnote_id`,`dlc_id`) USING BTREE;"].concat(),
        // ----------
        // Add index
        // ----------
        ["ALTER TABLE `",kphis,"`.`opd_er_focus_note` \
            ADD INDEX IF NOT EXISTS `fclist_id` (`fclist_id`) USING BTREE,\
            ADD INDEX IF NOT EXISTS `fcnote_date` (`fcnote_date`) USING BTREE;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_io` \
            ADD INDEX IF NOT EXISTS `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE,\
            ADD INDEX IF NOT EXISTS `opd_er_io_date` (`opd_er_io_date`) USING BTREE;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_nurse_index_plan` \
            DROP INDEX IF EXISTS `an_plan_date`,\
            ADD INDEX IF NOT EXISTS `opd_er_order_master_id_plan_date` (`opd_er_order_master_id`,`plan_date`) USING BTREE,\
            ADD INDEX IF NOT EXISTS `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_nurse_index_action` \
            DROP INDEX IF EXISTS `an_action_date`,\
            ADD INDEX IF NOT EXISTS `opd_er_order_master_id_action_date` (`opd_er_order_master_id`,`action_date`) USING BTREE,\
            ADD INDEX IF NOT EXISTS `action_date_time` (`action_date`,`action_time`) USING BTREE;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_vs_vital_sign` \
            ROW_FORMAT = DYNAMIC,\
            ADD INDEX IF NOT EXISTS `action_id` (`action_id`) USING BTREE,\
            ADD INDEX IF NOT EXISTS `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_vs_vital_sign` \
            ROW_FORMAT = DYNAMIC,\
            ADD INDEX IF NOT EXISTS `action_id` (`action_id`) USING BTREE;"].concat(),
        // ----------------------------------
        // Remove tis620 on non-HOSxP columns
        // --------------------------------//
        ["ALTER TABLE `",kphis,"`.`ipd_focus_note` \
            MODIFY COLUMN `general_symptoms` text DEFAULT NULL,\
            MODIFY COLUMN `assessment` text DEFAULT NULL,\
            MODIFY COLUMN `intvt_text` text DEFAULT NULL,\
            MODIFY COLUMN `evalution` text DEFAULT NULL,\
            MODIFY COLUMN `dlc_text` text DEFAULT NULL,\
            MODIFY COLUMN `other` text DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_focus_note` \
            MODIFY COLUMN `general_symptoms` text DEFAULT NULL,\
            MODIFY COLUMN `assessment` text DEFAULT NULL,\
            MODIFY COLUMN `intvt_text` text DEFAULT NULL,\
            MODIFY COLUMN `evalution` text DEFAULT NULL,\
            MODIFY COLUMN `dlc_text` text DEFAULT NULL,\
            MODIFY COLUMN `other` text DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_focus_list` \
            MODIFY COLUMN `focus_text` text DEFAULT NULL,\
            MODIFY COLUMN `goal_id` text DEFAULT NULL,\
            MODIFY COLUMN `goal_text` text DEFAULT NULL,\
            MODIFY COLUMN `fclist_status` VARCHAR(1) NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_focus_list` \
            MODIFY COLUMN `focus_text` text DEFAULT NULL,\
            MODIFY COLUMN `goal_id` text DEFAULT NULL,\
            MODIFY COLUMN `goal_text` text DEFAULT NULL,\
            MODIFY COLUMN `fclist_status` VARCHAR(1) NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_tmp_dlc` MODIFY COLUMN `dlc_name` text NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_tmp_intvt` MODIFY COLUMN `intvt_name` text NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_vs_lt_arm` MODIFY COLUMN `lt_arm_name` text DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_vs_lt_leg` MODIFY COLUMN `lt_leg_name` text DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_vs_rt_arm` MODIFY COLUMN `rt_arm_name` text DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_vs_rt_leg` MODIFY COLUMN `rt_leg_name` text DEFAULT NULL;"].concat(),
        // -----------------------------------------
        // an to VARCHAR(13) (for fail-safe with VN)
        // -----------------------------------------
        ["ALTER TABLE `",kphis,"`.`ipd_doctor_in_charge` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_dr_admission_note` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_dr_admission_note_item` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_dr_consult` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_dr_consult_signature_reply` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_dr_consult_signature_request` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_focus_list` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_focus_note` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_io` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_med_reconciliation` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_med_reconciliation_item` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_nurse_admission_note` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_nurse_index_action` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_nurse_index_note` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_nurse_index_plan` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_order` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_order_item` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_progress_note` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_progress_note_item` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_summary` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_summary_2` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_vs_vital_sign` MODIFY COLUMN `an` VARCHAR(13) NOT NULL COLLATE 'tis620_thai_ci';"].concat(),
        ["ALTER TABLE `",kphis,"`.`system_patient_lock` MODIFY COLUMN `an` VARCHAR(13) NULL DEFAULT NULL COLLATE 'tis620_thai_ci';"].concat(),
        // -----------------------------------------
        // signed to unsigned id, INT(10) to INT(11)
        // -----------------------------------------
        ["ALTER TABLE `",kphis,"`.`ipd_doctor_in_charge` MODIFY COLUMN `doctor_in_charge_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_dr_admission_note` MODIFY COLUMN `admission_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_dr_admission_note_item` \
            MODIFY COLUMN `admission_note_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `admission_note_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_dr_consult` \
            MODIFY COLUMN `consult_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `consult_type` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_dr_consult_signature_reply` \
            MODIFY COLUMN `consult_reply_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `consult_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_dr_consult_signature_request` \
            MODIFY COLUMN `consult_signature_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `consult_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_dr_consult_type` MODIFY COLUMN `consult_type_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_emergency` MODIFY COLUMN `emergency_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_focus_list` \
            MODIFY COLUMN `fclist_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `smp_id` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `focus_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_focus_list_goal_item` \
            MODIFY COLUMN `fclist_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `fclist_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `goal_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_focus_note` \
            MODIFY COLUMN `fcnote_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `fclist_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_focus_note_dlc_item` \
            MODIFY COLUMN `fcnote_dlc_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `fcnote_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `dlc_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_focus_note_intvt_item` \
            MODIFY COLUMN `fcnote_intvt_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `fcnote_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `intvt_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_io` MODIFY COLUMN `io_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_med_reconciliation` MODIFY COLUMN `med_reconciliation_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_med_reconciliation_item` \
            MODIFY COLUMN `med_reconciliation_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `med_reconciliation_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_nurse_admission_note` MODIFY COLUMN `nurse_admission_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_nurse_index_action` \
            MODIFY COLUMN `action_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `plan_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_nurse_index_note` MODIFY COLUMN `nurse_index_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_nurse_index_plan` \
            MODIFY COLUMN `plan_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `order_item_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_order` \
            MODIFY COLUMN `order_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `pre_order_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_order_item` \
            MODIFY COLUMN `order_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `order_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `off_order_item_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `med_reconciliation_item_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_pre_order` \
            MODIFY COLUMN `order_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `pre_order_master_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_pre_order_item` \
            MODIFY COLUMN `order_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `pre_order_master_id` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `order_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `off_order_item_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_pre_order_master` MODIFY COLUMN `pre_order_master_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_pre_order_progress_note` \
            MODIFY COLUMN `progress_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `pre_order_master_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_pre_order_progress_note_item` \
            MODIFY COLUMN `progress_note_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `progress_note_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `pre_order_master_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_progress_note` MODIFY COLUMN `progress_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_progress_note_item` \
            MODIFY COLUMN `progress_note_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `progress_note_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_summary` MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_summary_2` MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_summary_approve_doctor` MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_summary_attending_doctor` MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_summary_dx` \
            MODIFY COLUMN `id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `sort_index` INT(11) UNSIGNED NOT NULL DEFAULT 0;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_summary_external_cause` \
            MODIFY COLUMN `external_cause_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_summary_other_diagnosis` \
            MODIFY COLUMN `other_diagnosis_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_summary_post_admission_comorbidity` \
            MODIFY COLUMN `post_admission_comorbidity_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_summary_pre_admission_comorbidity` \
            MODIFY COLUMN `pre_admission_comorbidity_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `summary_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_tmp_dlc` \
            MODIFY COLUMN `dlc_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `dlc_order` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_tmp_focus` \
            MODIFY COLUMN `focus_id` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `smp_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `subgroup` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `focus_order` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_tmp_goal` \
            MODIFY COLUMN `goal_id` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `smp_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `subgroup` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `goal_order` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_tmp_group_smp` \
            MODIFY COLUMN `smp_id` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `smp_group` INT(11) UNSIGNED DEFAULT NULL,\
            MODIFY COLUMN `smp_order` INT(11) UNSIGNED DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_tmp_intvt` \
            MODIFY COLUMN `intvt_id` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `smp_id` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `subgroup` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `intvt_order` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_tmp_subgroup` \
            MODIFY COLUMN `smp_id` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `subgroup` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `subgroup_order` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_vs_lt_arm` MODIFY COLUMN `lt_arm` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_vs_lt_leg` MODIFY COLUMN `lt_leg` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_vs_rt_arm` MODIFY COLUMN `rt_arm` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_vs_rt_leg` MODIFY COLUMN `rt_leg` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`ipd_vs_vital_sign` \
            MODIFY COLUMN `vs_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `conscious_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `urine_amount` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `urine_duration` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `line_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `right_cha_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `left_cha_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `va_id` INT(1) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `mass_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `lt_arm` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `lt_leg` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `rt_arm` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `rt_leg` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `o2_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `tube_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `intake_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `output_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `lr_sta` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `lr_mem` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `breathing_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `avpu_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `gut_feeling_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `pops_other_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`kphis_spclty` MODIFY COLUMN `spclty_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_allergy_history` \
            MODIFY COLUMN `er_allergy_history_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_bed` MODIFY COLUMN `opd_er_bed_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_consult` \
            MODIFY COLUMN `er_consult_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_dch_type` MODIFY COLUMN `er_dch_type_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_document_scan` \
            MODIFY COLUMN `opd_er_document_scan_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_dr_pe` \
            MODIFY COLUMN `opd_er_pe_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_emergency_level` MODIFY COLUMN `emergency_level_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_focus_list` \
            MODIFY COLUMN `fclist_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `smp_id` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `focus_id` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_focus_list_goal_item` \
            MODIFY COLUMN `fclist_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `fclist_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `goal_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_focus_note` \
            MODIFY COLUMN `fcnote_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `fclist_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_focus_note_dlc_item` \
            MODIFY COLUMN `fcnote_dlc_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `fcnote_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `dlc_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_focus_note_intvt_item` \
            MODIFY COLUMN `fcnote_intvt_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `fcnote_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `intvt_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_io` \
            MODIFY COLUMN `opd_er_io_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_nurse_index_action` \
            MODIFY COLUMN `action_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_nurse_index_plan` \
            MODIFY COLUMN `plan_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `order_item_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_nurse_screening` \
            MODIFY COLUMN `opd_er_screening_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_order` \
            MODIFY COLUMN `order_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_order_item` \
            MODIFY COLUMN `order_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `order_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `off_order_item_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_order_master` \
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `bedno` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `er_patient_status_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `er_dch_type_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_order_progress_note` \
            MODIFY COLUMN `progress_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_order_progress_note_item` \
            MODIFY COLUMN `progress_note_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `progress_note_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_patient_status` MODIFY COLUMN `er_patient_status_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_set_fast_track` \
            MODIFY COLUMN `set_ft_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        ["ALTER TABLE `",kphis,"`.`opd_er_vs_vital_sign` \
            MODIFY COLUMN `vs_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            MODIFY COLUMN `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,\
            MODIFY COLUMN `conscious_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `urine_amount` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `urine_duration` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `line_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `right_cha_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `left_cha_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `va_id` INT(1) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `mass_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `lt_arm` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `lt_leg` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `rt_arm` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `rt_leg` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `o2_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `tube_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `intake_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `output_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `lr_sta` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `lr_mem` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `breathing_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `avpu_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `gut_feeling_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            MODIFY COLUMN `pops_other_id` INT(11) UNSIGNED NULL DEFAULT NULL;"].concat(),
        // // V0.4.16 fixed from v0.4.4
        // ["ALTER TABLE `",kphis,"`.`ipd_vs_vital_sign` \
        //     MODIFY COLUMN `barthel_index` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `aggression_oas` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `alcohol_ciwa` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `alcohol_aws` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `amphetamine_awq` TEXT DEFAULT NULL;"].concat(),
        // // V0.4.16 fixed from v0.4.4
        // ["ALTER TABLE `",kphis,"`.`opd_er_vs_vital_sign` \
        //     MODIFY COLUMN `barthel_index` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `aggression_oas` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `alcohol_ciwa` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `alcohol_aws` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `amphetamine_awq` TEXT DEFAULT NULL;"].concat(),
        // // V0.4.16 fixed from v0.4.4
        // ["ALTER TABLE `",kphis,"`.`ipd_dr_admission_note` \
        //     MODIFY COLUMN `amphetamine_awq` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `aggression_oas` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `alcohol_audit` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `alcohol_aws` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `alcohol_ciwa` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `depress_2q` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `depress_9q` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `depress_cdi` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `depress_cesd` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `depress_phqa` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `nicotin_ftnd` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `ptsd_screen` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `ptsd_pisces` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `ptsd_cries` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `suicide_8q` TEXT DEFAULT NULL,\
        //     MODIFY COLUMN `stress_st5` TEXT DEFAULT NULL;"].concat(),
    ]
}

pub fn update_kphis_log(kphis_log: &str) -> Vec<String> {
    vec![
        ["CREATE TABLE IF NOT EXISTS `",kphis_log,"`.`history_log` (\
            `history_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `history_datetime` DATETIME NULL DEFAULT NULL,\
            `history_table_name` VARCHAR(255) NULL DEFAULT NULL,\
            `history_type` VARCHAR(1) NULL DEFAULT NULL,\
            `history_user` VARCHAR(50) NULL DEFAULT NULL,\
            `data` LONGTEXT NULL DEFAULT NULL,\
            PRIMARY KEY (`history_id`) USING BTREE,\
            INDEX `history_datetime`(`history_datetime`) USING BTREE,\
            INDEX `history_table_name`(`history_table_name`(191)) USING BTREE\
            ) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_log,"`.`system_access_log` (\
            `access_log_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `access_datetime` DATETIME NULL DEFAULT NULL,\
            `access_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `access_host` VARCHAR(100) NULL DEFAULT NULL,\
            `access_detail` TEXT NULL DEFAULT NULL,\
            PRIMARY KEY (`access_log_id`) USING BTREE\
            ) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_log,"`.`message` (\
            `message_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `message_datetime` DATETIME DEFAULT NULL,\
            `message` TEXT NULL DEFAULT NULL,\
            `sender_code` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL DEFAULT '',\
            `sender_name` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `person` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `ward` VARCHAR(4) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `spclty_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `route` TEXT NULL DEFAULT NULL,\
            `reference` LONGTEXT NULL DEFAULT NULL,\
            PRIMARY KEY (`message_id`) USING BTREE,\
            INDEX `message_datetime` (`message_datetime`) USING BTREE,\
            INDEX `person` (`person`) USING BTREE,\
            INDEX `ward` (`ward`) USING BTREE,\
            INDEX `sender_code` (`sender_code`) USING BTREE,\
            INDEX `spclty_id` (`spclty_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_log,"`.`message_read` (\
            `message_read_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `message_id` INT(11) UNSIGNED NOT NULL,\
            `read_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,\
            `read_datetime` DATETIME NOT NULL,\
            PRIMARY KEY (`message_read_id`) USING BTREE,\
            UNIQUE INDEX `id_user` (`message_id`, `read_user`) USING BTREE\
        ) COLLATE='utf8mb4_general_ci' ENGINE=InnoDB;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_log,"`.`ipt_log` (\
            `ipt_log_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `ipt_log_type` VARCHAR(1) NOT NULL,\
            `an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `vn` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `hn` VARCHAR(9) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `ward` VARCHAR(4) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            PRIMARY KEY (`ipt_log_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
    ]
}

pub fn update_kphis_extra(kphis_extra: &str) -> Vec<String> {
    vec![
        ["CREATE DATABASE IF NOT EXISTS `",kphis_extra,"` COLLATE 'utf8mb4_general_ci';"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`image` (\
            `image_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `path` CHAR(33) NOT NULL,\
            `title` TEXT NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`image_id`) USING BTREE,\
            UNIQUE KEY `path` (`path`) USING BTREE,\
            INDEX `create_user` (`create_user`) USING BTREE\
        ) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`image_usage` (\
            `image_usage_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `usage_id` TINYINT(3) UNSIGNED NOT NULL,\
            `usage_key_id` INT(11) UNSIGNED NOT NULL,\
            `image_id` INT(11) UNSIGNED NOT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`image_usage_id`) USING BTREE,\
            UNIQUE KEY `usage_triple` (`usage_id`,`usage_key_id`,`image_id`) USING BTREE,\
            INDEX `image_id` (`image_id`) USING BTREE\
        ) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_document` (\
            `document_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `an` VARCHAR(13) COLLATE 'tis620_thai_ci' NOT NULL,\
            `document_type_id` TINYINT(3) UNSIGNED NOT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`document_id`) USING BTREE,\
            UNIQUE INDEX `an_type_id` (`an`,`document_type_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`opd_er_document` (\
            `document_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,\
            `document_type_id` TINYINT(3) UNSIGNED NOT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`document_id`) USING BTREE,\
            UNIQUE INDEX `om_type_id` (`opd_er_order_master_id`,`document_type_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_dc_plan` (\
            `dc_plan_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `dx_id` INT(11) UNSIGNED NOT NULL,\
            `dc_datetime` DATETIME NULL DEFAULT NULL,\
            `dc_type_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `dc_type_refer` VARCHAR(1) NULL DEFAULT NULL,\
            `dc_type_other` VARCHAR(50) NULL DEFAULT NULL,\
            `dc_symptom` TEXT NULL DEFAULT NULL,\
            `inst_none` VARCHAR(1) NULL DEFAULT NULL,\
            `inst_foley` VARCHAR(1) NULL DEFAULT NULL,\
            `inst_ett` VARCHAR(1) NULL DEFAULT NULL,\
            `inst_tt` VARCHAR(1) NULL DEFAULT NULL,\
            `inst_ng` VARCHAR(1) NULL DEFAULT NULL,\
            `inst_other` VARCHAR(50) NULL DEFAULT NULL,\
            `with_drug` VARCHAR(1) NULL DEFAULT NULL,\
            `with_appoint` VARCHAR(1) NULL DEFAULT NULL,\
            `with_cert` VARCHAR(1) NULL DEFAULT NULL,\
            `with_other` VARCHAR(50) NULL DEFAULT NULL,\
            `appoint_date` DATE NULL DEFAULT NULL,\
            `appoint_time` TIME NULL DEFAULT NULL,\
            `appoint_place` VARCHAR(250) NULL DEFAULT NULL,\
            `appoint_for` VARCHAR(250) NULL DEFAULT NULL,\
            `refer_to` VARCHAR(250) NULL DEFAULT NULL,\
            `dx_text` TEXT NULL DEFAULT NULL,\
            `dx_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `dx_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `dx_other` VARCHAR(50) NULL DEFAULT NULL,\
            `dx_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `dx_datetime` DATETIME NULL DEFAULT NULL,\
            `med_text` TEXT NULL DEFAULT NULL,\
            `med_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `med_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `med_other` VARCHAR(50) NULL DEFAULT NULL,\
            `med_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `med_datetime` DATETIME NULL DEFAULT NULL,\
            `env_text` TEXT NULL DEFAULT NULL,\
            `env_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `env_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `env_other` VARCHAR(50) NULL DEFAULT NULL,\
            `env_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `env_datetime` DATETIME NULL DEFAULT NULL,\
            `tx_text` TEXT NULL DEFAULT NULL,\
            `tx_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `tx_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `tx_other` VARCHAR(50) NULL DEFAULT NULL,\
            `tx_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `tx_datetime` DATETIME NULL DEFAULT NULL,\
            `health_text` TEXT NULL DEFAULT NULL,\
            `health_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `health_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `health_other` VARCHAR(50) NULL DEFAULT NULL,\
            `health_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `health_datetime` DATETIME NULL DEFAULT NULL,\
            `out_text` TEXT NULL DEFAULT NULL,\
            `out_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `out_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `out_other` VARCHAR(50) NULL DEFAULT NULL,\
            `out_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `out_datetime` DATETIME NULL DEFAULT NULL,\
            `diet_text` TEXT NULL DEFAULT NULL,\
            `diet_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `diet_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `diet_other` VARCHAR(50) NULL DEFAULT NULL,\
            `diet_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `diet_datetime` DATETIME NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`dc_plan_id`) USING BTREE,\
            UNIQUE INDEX `an_dx` (`an`,`dx_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_dc_plan_med_item` (\
            `med_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `dc_plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `med_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`med_item_id`) USING BTREE,\
            UNIQUE INDEX `plan_med` (`dc_plan_id`,`med_id`) USING BTREE,\
            INDEX `med_id` (`med_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_dc_plan_env_item` (\
            `env_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `dc_plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `env_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`env_item_id`) USING BTREE,\
            UNIQUE INDEX `plan_env` (`dc_plan_id`,`env_id`) USING BTREE,\
            INDEX `env_id` (`env_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_dc_plan_tx_item` (\
            `tx_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `dc_plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `tx_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`tx_item_id`) USING BTREE,\
            UNIQUE INDEX `plan_tx` (`dc_plan_id`,`tx_id`) USING BTREE,\
            INDEX `tx_id` (`tx_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_dc_plan_diet_item` (\
            `diet_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `dc_plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `diet_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`diet_item_id`) USING BTREE,\
            UNIQUE INDEX `plan_diet` (`dc_plan_id`,`diet_id`) USING BTREE,\
            INDEX `diet_id` (`diet_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`opd_er_dc_plan` (\
            `dc_plan_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,\
            `dx_id` INT(11) UNSIGNED NOT NULL,\
            `dc_datetime` DATETIME NULL DEFAULT NULL,\
            `dc_type_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `dc_type_refer` VARCHAR(1) NULL DEFAULT NULL,\
            `dc_type_other` VARCHAR(50) NULL DEFAULT NULL,\
            `dc_symptom` TEXT NULL DEFAULT NULL,\
            `inst_none` VARCHAR(1) NULL DEFAULT NULL,\
            `inst_foley` VARCHAR(1) NULL DEFAULT NULL,\
            `inst_ett` VARCHAR(1) NULL DEFAULT NULL,\
            `inst_tt` VARCHAR(1) NULL DEFAULT NULL,\
            `inst_ng` VARCHAR(1) NULL DEFAULT NULL,\
            `inst_other` VARCHAR(50) NULL DEFAULT NULL,\
            `with_drug` VARCHAR(1) NULL DEFAULT NULL,\
            `with_appoint` VARCHAR(1) NULL DEFAULT NULL,\
            `with_cert` VARCHAR(1) NULL DEFAULT NULL,\
            `with_other` VARCHAR(50) NULL DEFAULT NULL,\
            `appoint_date` DATE NULL DEFAULT NULL,\
            `appoint_time` TIME NULL DEFAULT NULL,\
            `appoint_place` VARCHAR(250) NULL DEFAULT NULL,\
            `appoint_for` VARCHAR(250) NULL DEFAULT NULL,\
            `refer_to` VARCHAR(250) NULL DEFAULT NULL,\
            `dx_text` TEXT NULL DEFAULT NULL,\
            `dx_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `dx_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `dx_other` VARCHAR(50) NULL DEFAULT NULL,\
            `dx_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `dx_datetime` DATETIME NULL DEFAULT NULL,\
            `med_text` TEXT NULL DEFAULT NULL,\
            `med_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `med_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `med_other` VARCHAR(50) NULL DEFAULT NULL,\
            `med_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `med_datetime` DATETIME NULL DEFAULT NULL,\
            `env_text` TEXT NULL DEFAULT NULL,\
            `env_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `env_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `env_other` VARCHAR(50) NULL DEFAULT NULL,\
            `env_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `env_datetime` DATETIME NULL DEFAULT NULL,\
            `tx_text` TEXT NULL DEFAULT NULL,\
            `tx_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `tx_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `tx_other` VARCHAR(50) NULL DEFAULT NULL,\
            `tx_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `tx_datetime` DATETIME NULL DEFAULT NULL,\
            `health_text` TEXT NULL DEFAULT NULL,\
            `health_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `health_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `health_other` VARCHAR(50) NULL DEFAULT NULL,\
            `health_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `health_datetime` DATETIME NULL DEFAULT NULL,\
            `out_text` TEXT NULL DEFAULT NULL,\
            `out_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `out_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `out_other` VARCHAR(50) NULL DEFAULT NULL,\
            `out_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `out_datetime` DATETIME NULL DEFAULT NULL,\
            `diet_text` TEXT NULL DEFAULT NULL,\
            `diet_patient_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `diet_relatives_ok` VARCHAR(1) NULL DEFAULT NULL,\
            `diet_other` VARCHAR(50) NULL DEFAULT NULL,\
            `diet_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `diet_datetime` DATETIME NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`dc_plan_id`) USING BTREE,\
            UNIQUE INDEX `opd_er_order_master_id_dx` (`opd_er_order_master_id`,`dx_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`opd_er_dc_plan_med_item` (\
            `med_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `dc_plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `med_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`med_item_id`) USING BTREE,\
            UNIQUE INDEX `plan_med` (`dc_plan_id`,`med_id`) USING BTREE,\
            INDEX `med_id` (`med_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`opd_er_dc_plan_env_item` (\
            `env_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `dc_plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `env_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`env_item_id`) USING BTREE,\
            UNIQUE INDEX `plan_env` (`dc_plan_id`,`env_id`) USING BTREE,\
            INDEX `env_id` (`env_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`opd_er_dc_plan_tx_item` (\
            `tx_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `dc_plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `tx_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`tx_item_id`) USING BTREE,\
            UNIQUE INDEX `plan_tx` (`dc_plan_id`,`tx_id`) USING BTREE,\
            INDEX `tx_id` (`tx_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`opd_er_dc_plan_diet_item` (\
            `diet_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `dc_plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `diet_id` INT(11) UNSIGNED NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`diet_item_id`) USING BTREE,\
            UNIQUE INDEX `plan_diet` (`dc_plan_id`,`diet_id`) USING BTREE,\
            INDEX `diet_id` (`diet_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_dc_plan_tmp_dx` (\
            `dx_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `dx_name` VARCHAR(250) NOT NULL,\
            `dx_knowledge` TEXT NULL DEFAULT NULL,\
            `dx_revisit` TEXT NULL DEFAULT NULL,\
            `dx_prevention` TEXT NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`dx_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_dc_plan_tmp_med` (\
            `med_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `med_text` TEXT NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`med_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_dc_plan_tmp_env` (\
            `env_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `env_text` TEXT NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`env_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_dc_plan_tmp_tx` (\
            `tx_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `tx_text` TEXT NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`tx_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_dc_plan_tmp_diet` (\
            `diet_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `diet_text` TEXT NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`diet_id`) USING BTREE\
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_mra` (\
            `mra_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `hn` VARCHAR(9) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,\
            `adm_date` DATE DEFAULT NULL,\
            `dch_date` DATE DEFAULT NULL,\
            `audit_type` CHAR(1) NOT NULL DEFAULT 'I',\
            `is_psychiatry` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `is_not_sorted` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `is_unknown` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `overall` CHAR(1) DEFAULT NULL,\
            `overall_text` TEXT DEFAULT NULL,\
            `auditor` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,\
            `audit_date` DATE DEFAULT NULL,\
            `sd_m` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `sd_n` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `sd_1` TINYINT(1) UNSIGNED DEFAULT 0,\
            `sd_2` TINYINT(1) UNSIGNED DEFAULT 0,\
            `sd_3` TINYINT(1) UNSIGNED DEFAULT 0,\
            `sd_4` TINYINT(1) UNSIGNED DEFAULT 0,\
            `sd_5` TINYINT(1) UNSIGNED DEFAULT 0,\
            `sd_6` TINYINT(1) UNSIGNED DEFAULT 0,\
            `sd_7` TINYINT(1) UNSIGNED DEFAULT 0,\
            `sd_8` TINYINT(1) UNSIGNED DEFAULT 0,\
            `sd_9` TINYINT(1) UNSIGNED DEFAULT 0,\
            `sd_text` TEXT DEFAULT NULL,\
            `so_m` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `so_n` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `so_1` TINYINT(1) UNSIGNED DEFAULT 0,\
            `so_2` TINYINT(1) UNSIGNED DEFAULT 0,\
            `so_3` TINYINT(1) UNSIGNED DEFAULT 0,\
            `so_4` TINYINT(1) UNSIGNED DEFAULT 0,\
            `so_5` TINYINT(1) UNSIGNED DEFAULT 0,\
            `so_6` TINYINT(1) UNSIGNED DEFAULT 0,\
            `so_7` TINYINT(1) UNSIGNED DEFAULT 0,\
            `so_text` TEXT DEFAULT NULL,\
            `ic_m` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `ic_n` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `ic_1` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ic_2` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ic_3` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ic_4` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ic_5` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ic_6` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ic_7` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ic_8` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ic_9` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ic_text` TEXT DEFAULT NULL,\
            `hx_m` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `hx_n` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `hx_1` TINYINT(1) UNSIGNED DEFAULT 0,\
            `hx_2` TINYINT(1) UNSIGNED DEFAULT 0,\
            `hx_3` TINYINT(1) UNSIGNED DEFAULT 0,\
            `hx_4` TINYINT(1) UNSIGNED DEFAULT 0,\
            `hx_5` TINYINT(1) UNSIGNED DEFAULT 0,\
            `hx_6` TINYINT(1) UNSIGNED DEFAULT 0,\
            `hx_7` TINYINT(1) UNSIGNED DEFAULT 0,\
            `hx_8` TINYINT(1) UNSIGNED DEFAULT 0,\
            `hx_9` TINYINT(1) UNSIGNED DEFAULT 0,\
            `hx_text` TEXT DEFAULT NULL,\
            `pe_m` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `pe_n` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `pe_1` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pe_2` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pe_3` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pe_4` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pe_5` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pe_6` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pe_7` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pe_8` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pe_9` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pe_text` TEXT DEFAULT NULL,\
            `pn_m` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `pn_n` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `pn_1` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pn_2` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pn_3` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pn_4` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pn_5` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pn_6` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pn_7` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pn_8` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pn_9` TINYINT(1) UNSIGNED DEFAULT 0,\
            `pn_text` TEXT DEFAULT NULL,\
            `cr_na` TINYINT(1) UNSIGNED NOT NULL DEFAULT 1,\
            `cr_m` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `cr_n` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `cr_1` TINYINT(1) UNSIGNED DEFAULT 0,\
            `cr_2` TINYINT(1) UNSIGNED DEFAULT 0,\
            `cr_3` TINYINT(1) UNSIGNED DEFAULT 0,\
            `cr_4` TINYINT(1) UNSIGNED DEFAULT 0,\
            `cr_5` TINYINT(1) UNSIGNED DEFAULT 0,\
            `cr_6` TINYINT(1) UNSIGNED DEFAULT 0,\
            `cr_7` TINYINT(1) UNSIGNED DEFAULT 0,\
            `cr_8` TINYINT(1) UNSIGNED DEFAULT 0,\
            `cr_9` TINYINT(1) UNSIGNED DEFAULT 0,\
            `cr_text` TEXT DEFAULT NULL,\
            `ar_na` TINYINT(1) UNSIGNED NOT NULL DEFAULT 1,\
            `ar_m` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `ar_n` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `ar_1` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ar_2` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ar_3` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ar_4` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ar_5` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ar_6` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ar_7` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ar_8` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ar_9` TINYINT(1) UNSIGNED DEFAULT 0,\
            `ar_text` TEXT DEFAULT NULL,\
            `on_na` TINYINT(1) UNSIGNED NOT NULL DEFAULT 1,\
            `on_m` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `on_n` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `on_1` TINYINT(1) UNSIGNED DEFAULT 0,\
            `on_2` TINYINT(1) UNSIGNED DEFAULT 0,\
            `on_3` TINYINT(1) UNSIGNED DEFAULT 0,\
            `on_4` TINYINT(1) UNSIGNED DEFAULT 0,\
            `on_5` TINYINT(1) UNSIGNED DEFAULT 0,\
            `on_6` TINYINT(1) UNSIGNED DEFAULT 0,\
            `on_7` TINYINT(1) UNSIGNED DEFAULT 0,\
            `on_8` TINYINT(1) UNSIGNED DEFAULT 0,\
            `on_9` TINYINT(1) UNSIGNED DEFAULT 0,\
            `on_text` TEXT DEFAULT NULL,\
            `lr_na` TINYINT(1) UNSIGNED NOT NULL DEFAULT 1,\
            `lr_m` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `lr_n` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `lr_1` TINYINT(1) UNSIGNED DEFAULT 0,\
            `lr_2` TINYINT(1) UNSIGNED DEFAULT 0,\
            `lr_3` TINYINT(1) UNSIGNED DEFAULT 0,\
            `lr_4` TINYINT(1) UNSIGNED DEFAULT 0,\
            `lr_5` TINYINT(1) UNSIGNED DEFAULT 0,\
            `lr_6` TINYINT(1) UNSIGNED DEFAULT 0,\
            `lr_7` TINYINT(1) UNSIGNED DEFAULT 0,\
            `lr_8` TINYINT(1) UNSIGNED DEFAULT 0,\
            `lr_9` TINYINT(1) UNSIGNED DEFAULT 0,\
            `lr_text` TEXT DEFAULT NULL,\
            `rr_na` TINYINT(1) UNSIGNED NOT NULL DEFAULT 1,\
            `rr_m` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `rr_n` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `rr_1` TINYINT(1) UNSIGNED DEFAULT 0,\
            `rr_2` TINYINT(1) UNSIGNED DEFAULT 0,\
            `rr_3` TINYINT(1) UNSIGNED DEFAULT 0,\
            `rr_4` TINYINT(1) UNSIGNED DEFAULT 0,\
            `rr_5` TINYINT(1) UNSIGNED DEFAULT 0,\
            `rr_6` TINYINT(1) UNSIGNED DEFAULT 0,\
            `rr_7` TINYINT(1) UNSIGNED DEFAULT 0,\
            `rr_8` TINYINT(1) UNSIGNED DEFAULT 0,\
            `rr_9` TINYINT(1) UNSIGNED DEFAULT 0,\
            `rr_text` TEXT DEFAULT NULL,\
            `nn_m` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `nn_n` TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,\
            `nn_1` TINYINT(1) UNSIGNED DEFAULT 0,\
            `nn_2` TINYINT(1) UNSIGNED DEFAULT 0,\
            `nn_3` TINYINT(1) UNSIGNED DEFAULT 0,\
            `nn_4` TINYINT(1) UNSIGNED DEFAULT 0,\
            `nn_5` TINYINT(1) UNSIGNED DEFAULT 0,\
            `nn_6` TINYINT(1) UNSIGNED DEFAULT 0,\
            `nn_7` TINYINT(1) UNSIGNED DEFAULT 0,\
            `nn_8` TINYINT(1) UNSIGNED DEFAULT 0,\
            `nn_9` TINYINT(1) UNSIGNED DEFAULT 0,\
            `nn_sub` TINYINT(1) UNSIGNED DEFAULT 0,\
            `nn_text` TEXT DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`mra_id`) USING BTREE,\
            INDEX `an` (`an`) USING BTREE\
        ) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`user_config` (\
            `loginname` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,\
            `wards` TEXT DEFAULT NULL,\
	        `spcltys` TEXT DEFAULT NULL,\
            `theme` VARCHAR(5) NULL DEFAULT NULL COLLATE 'utf8mb4_general_ci',\
            `wide_screen` VARCHAR(5) NULL DEFAULT NULL COLLATE 'utf8mb4_general_ci',\
            `totp` VARCHAR(50) NULL DEFAULT NULL COLLATE 'utf8mb4_general_ci',\
            `ts` BIGINT(20) UNSIGNED NULL DEFAULT NULL,\
            `totp_done` TINYINT(1) NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`loginname`) USING BTREE\
        ) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`prescription_screen` (\
            `vn` VARCHAR(13) COLLATE 'tis620_thai_ci' NOT NULL,\
            `pharmacist_accept` VARCHAR(7) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `pharmacist_accept_time` DATETIME NULL DEFAULT NULL,\
            `pharmacist_check` VARCHAR(7) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `pharmacist_check_time` DATETIME NULL DEFAULT NULL,\
            `pharmacist_done` VARCHAR(7) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `pharmacist_done_time` DATETIME NULL DEFAULT NULL,\
            `postal_status` VARCHAR(1) NULL DEFAULT NULL,\
            `postal_doctor` VARCHAR(7) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `postal_time` DATETIME NULL DEFAULT NULL,\
            `telemed_add` TEXT NULL DEFAULT NULL,\
            `telemed_dose_up` TEXT NULL DEFAULT NULL,\
            `telemed_dose_down` TEXT NULL DEFAULT NULL,\
            `telemed_off` TEXT NULL DEFAULT NULL,\
            `telemed_other` TEXT NULL DEFAULT NULL,\
            `telemed_doctor` VARCHAR(7) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `telemed_time` DATETIME NULL DEFAULT NULL,\
            `pharmacy_care` TEXT NULL DEFAULT NULL,\
            `pharmacy_care_doctor` VARCHAR(7) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
	        `pharmacy_care_time` DATETIME NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`vn`) USING BTREE,\
            INDEX `postal_status` (`postal_status`) USING BTREE\
        ) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`report_template` (\
            `template_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `template_name` VARCHAR(250) NOT NULL,\
            `title` VARCHAR(250) NOT NULL,\
            `content` TEXT NOT NULL,\
            `statement` TEXT NULL DEFAULT NULL,\
            `statement_params` TEXT NULL DEFAULT NULL,\
            `info` TEXT NULL DEFAULT NULL,\
            `disabled` TINYINT(1) NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`template_id`) USING BTREE,\
            UNIQUE INDEX `template_name` (`template_name`) USING BTREE\
        ) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_nurse_index_monitor` (\
            `monitor_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `action_id` INT(11) UNSIGNED NOT NULL,\
            `an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `monitor_datetime` DATETIME NOT NULL,\
            `monitor_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `monitor_abnormal` VARCHAR(1) DEFAULT NULL,\
            `monitor_result` TEXT DEFAULT NULL,\
            `monitor_remark` TEXT DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`monitor_id`) USING BTREE,\
            INDEX `action_id` (`action_id`) USING BTREE,\
            INDEX `an` (`an`) USING BTREE\
        ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`opd_er_nurse_index_monitor` (\
            `monitor_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `action_id` INT(11) UNSIGNED NOT NULL,\
            `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,\
            `monitor_datetime` DATETIME NOT NULL,\
            `monitor_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `monitor_abnormal` VARCHAR(1) DEFAULT NULL,\
            `monitor_result` TEXT DEFAULT NULL,\
            `monitor_remark` TEXT DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`monitor_id`) USING BTREE,\
            INDEX `action_id` (`action_id`) USING BTREE,\
            INDEX `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE\
        ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_summary_audit` (\
            `summary_audit_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `summary_id` INT(11) UNSIGNED NOT NULL,\
            `payer` VARCHAR(3) NULL DEFAULT NULL,\
            `audit_type` CHAR(1) NOT NULL DEFAULT 'I',\
            `doctor_auth` VARCHAR(1) NULL DEFAULT NULL,\
            `com_hn` VARCHAR(9) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `com_an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `com_adm_datetime` DATETIME NULL DEFAULT NULL,\
            `com_dch_datetime` DATETIME NOT NULL,\
            `com_leaveday` INT(11) NULL DEFAULT NULL,\
            `com_sex` CHAR(1) NULL DEFAULT NULL,\
            `com_birthday` DATE NULL DEFAULT NULL,\
            `com_bw` INT(11) NULL DEFAULT NULL,\
            `com_dchstts` VARCHAR(2) NULL DEFAULT NULL,\
            `com_dchtype` VARCHAR(2) NULL DEFAULT NULL,\
            `com_pid` VARCHAR(13) NULL DEFAULT NULL,\
            `com_drg` VARCHAR(5) NULL DEFAULT NULL,\
            `com_rw` DOUBLE(15,5) NULL DEFAULT NULL,\
            `com_adjrw` DOUBLE(15,5) NULL DEFAULT NULL,\
            `rev_hn` VARCHAR(9) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `rev_an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `rev_adm_datetime` DATETIME NULL DEFAULT NULL,\
            `rev_dch_datetime` DATETIME NULL DEFAULT NULL,\
            `rev_leaveday` INT(11) NULL DEFAULT NULL,\
            `rev_sex` CHAR(1) NULL DEFAULT NULL,\
            `rev_birthday` DATE NULL DEFAULT NULL,\
            `rev_bw` INT(11) NULL DEFAULT NULL,\
            `rev_dchstts` VARCHAR(2) NULL DEFAULT NULL,\
            `rev_dchtype` VARCHAR(2) NULL DEFAULT NULL,\
            `rev_pid` VARCHAR(13) NULL DEFAULT NULL,\
            `rev_drg` VARCHAR(5) NULL DEFAULT NULL,\
            `rev_rw` DOUBLE(15,5) NULL DEFAULT NULL,\
            `rev_adjrw` DOUBLE(15,5) NULL DEFAULT NULL,\
            `sa` VARCHAR(2) NOT NULL,\
            `ca` VARCHAR(2) NOT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`summary_audit_id`) USING BTREE,\
            INDEX `summary_id` (`summary_id`) USING BTREE,\
            INDEX `com_dch_datetime` (`com_dch_datetime`) USING BTREE\
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`ipd_summary_audit_item` (\
            `summary_audit_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `summary_audit_id` INT(11) UNSIGNED NOT NULL,\
            `summary_id` INT(11) UNSIGNED NOT NULL,\
            `ty` VARCHAR(3) NOT NULL,\
            `sum_dx` TEXT DEFAULT NULL,\
            `sum_icd` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `com_icd` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `rev_dx` TEXT DEFAULT NULL,\
            `rev_icd` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `sa` VARCHAR(2) NOT NULL,\
            `ca` VARCHAR(2) NOT NULL,\
            `remark` TEXT NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`summary_audit_item_id`) USING BTREE,\
            INDEX `summary_audit_id` (`summary_audit_id`) USING BTREE\
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
        ["CREATE TABLE IF NOT EXISTS `",kphis_extra,"`.`refer_note` (\
            `refernote_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,\
            `vn` VARCHAR(13) COLLATE 'tis620_thai_ci' NOT NULL,\
            `hn` VARCHAR(9) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `refer_hospcode` VARCHAR(9) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `refer_date` DATE NULL DEFAULT NULL,\
            `refer_time` TIME NULL DEFAULT NULL,\
            `docno` TEXT NULL DEFAULT NULL,\
            `pmh` TEXT NULL DEFAULT NULL,\
            `hpi` TEXT NULL DEFAULT NULL,\
            `lab_text` TEXT NULL DEFAULT NULL,\
            `treatment_text` TEXT NULL DEFAULT NULL,\
            `other_text` TEXT NULL DEFAULT NULL,\
            `diagnosis_text` TEXT NULL DEFAULT NULL,\
            `request_text` TEXT NULL DEFAULT NULL,\
            `doctor` VARCHAR(7) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,\
            `cc` TEXT NULL DEFAULT NULL,\
            `pe` TEXT NULL DEFAULT NULL,\
            `create_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,\
            `create_datetime` DATETIME NOT NULL,\
            `update_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,\
            `update_datetime` DATETIME NOT NULL,\
            `version` INT(11) NOT NULL,\
            PRIMARY KEY (`refernote_id`) USING BTREE,\
            INDEX `ix_refer_date` (`refer_date`) USING BTREE,\
            INDEX `ix_vn` (`vn`) USING BTREE,\
            INDEX `ix_hn` (`hn`) USING BTREE\
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;"].concat(),
    ]
}