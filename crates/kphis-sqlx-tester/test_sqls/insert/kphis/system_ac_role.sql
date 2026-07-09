INSERT INTO `kphis`.`system_ac_role` (`role`, `role_desc`, `parent_role`, `create_user`, `create_datetime`, `update_user`, `update_datetime`, `version`) VALUES
	('IT_ADMIN', 'IT DEPARTMENT', NULL, 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0),
	('MSO', 'MED ASSOCIATE', NULL, 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0),
	('DOCTOR', 'GEN DOCTOR', 'MSO', 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0),
	('DOCTOR_INTERN', 'INTERN DOCTOR', 'DOCTOR', 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0),
	('DOCTOR_STAFF', 'STAFF DOCTOR', 'DOCTOR', 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0),
	('MEDICAL_RECORD', 'เวชระเบียน', NULL, 'jommarn', '2020-03-18 11:47:54', 'jommarn', '2020-03-18 11:47:54', 0),
	('NURSE', 'พยาบาล', NULL, 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0),
	('NURSE_ER', 'พยาบาล ER', 'NURSE', 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0),
	('NURSE_ER_RN_EMT', 'พยาบาล ER RN/EMT', 'NURSE_ER', 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0),
	('NURSE_IPD', 'พยาบาล IPD', 'NURSE', 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0),
	('NURSE_IPD_RN_TN', 'พยาบาล IPD RN/TN', 'NURSE_IPD', 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0),
	('NURSE_OPD', 'พยาบาล OPD', 'NURSE', 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0),
	('NURSE_OPD_RN_TN', 'พยาบาล OPD RN/TN', 'NURSE_OPD', 'jommarn', '2020-01-18 15:54:41', 'jommarn', '2020-01-18 15:54:41', 0);