SELECT ps.pharmacy_care,ps.pharmacy_care_time,p.hn,p.pname,p.fname,p.lname
FROM __KPHIS_EXTRA__.prescription_screen ps
    LEFT JOIN __HOSXP__.ovst ON ovst.vn=ps.vn
    LEFT JOIN __HOSXP__.patient p ON p.hn=ovst.hn
WHERE ps.pharmacy_care IS NOT NULL AND DATE(ps.pharmacy_care_time) BETWEEN ? AND ? ORDER BY ps.pharmacy_care_time;