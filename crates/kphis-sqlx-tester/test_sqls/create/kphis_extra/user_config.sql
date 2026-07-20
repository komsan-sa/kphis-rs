CREATE TABLE `kphis_extra`.`user_config` (
	`loginname` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,
	`wards` TEXT DEFAULT NULL,
	`spcltys` TEXT DEFAULT NULL,
	`theme` VARCHAR(5) NULL DEFAULT NULL COLLATE 'utf8mb4_general_ci',
	`wide_screen` VARCHAR(5) NULL DEFAULT NULL COLLATE 'utf8mb4_general_ci',
	`totp` VARCHAR(50) NULL DEFAULT NULL COLLATE 'utf8mb4_general_ci',
	`ts` BIGINT(20) UNSIGNED NULL DEFAULT NULL,
	`failed` TINYINT(2) NULL DEFAULT NULL,
	`totp_done` TINYINT(1) NULL DEFAULT NULL,
    `create_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,
	`create_datetime` DATETIME NOT NULL,
	`update_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,
	`update_datetime` DATETIME NOT NULL,
	`version` INT(11) NOT NULL,
	PRIMARY KEY (`loginname`) USING BTREE
) ENGINE=InnoDB CHARACTER SET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;