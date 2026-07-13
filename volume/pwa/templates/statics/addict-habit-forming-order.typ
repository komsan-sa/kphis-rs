#import "customs/config.typ": hospital-name, hospital-address
#let data = json("data.json")
#let is_addict = data.at("is_addict",default:false)
#let patient = data.at("patient",default: none)
#let order_item = data.at("order_item",default: none)
#let order_doctor_name = data.at("order_doctor_name",default: none)
#let order_doctor_licenseno = data.at("order_doctor_licenseno",default: none)
// PREPARED FUNCTIONS
#let parse_d(date) = if date == none {none} else {
  let (y,mo,d) = date.split("-")
  datetime(year:int(y),month:int(mo),day:int(d))
}
#let license_type(s) = if s == none {none} else {
  let tuple = s.split(".")
  if tuple.len() > 1 {
    let pre = tuple.at(0)
    ((pre == "ว","เวชกรรม"),
    (pre == "ท","ทันตกรรม"),
    (true, "")).find(t => t.at(0)).at(1)
  } else {
    "เวชกรรม"
  }
}
#let split_med_name(s) = if s == none {(none,none,none)} else {
  let tuples = s.split(" ")
  let tl = tuples.len()
  if tl > 3 {
    let unit = tuples.pop()
    let st_2 = tuples.pop()
    let st_1 = tuples.pop()
    (tuples.join(" "),[#st_1, " ", #st_2],unit)
  } else {
    (none,none,none)
  }
}
#let (med_name, stregnth, unit) = split_med_name(order_item.med_name) 
// RENDER
#let form_title = if is_addict {"ยาเสพติดให้โทษในประเภท ๒"} else {"วัตถุออกฤทธิ์ในประเภท ๒"}
#set text(font:"TH Sarabun New",size:12pt)
#set page(paper:"a4",margin:(x:1.5cm,y:2cm))
#align(center,text(size:20pt,weight:700,[ใบสั่งจ่าย#form_title]))
#align(center,line(length:50%,stroke:2pt))
#align(right,[วันที่ #parse_d(order_item.order_date)])
#v(20pt)
#h(20pt)ข้าพเจ้า #order_doctor_name ใบอนุญาตประกอบวิชาชีพ#license_type(order_doctor_licenseno) #order_doctor_licenseno ปฏิบัติงาน ณ สถานพยาบาลชื่อ #hospital-name ตั้งอยู่ที่ #hospital-address
#v(20pt)
#h(20pt)#underline(strong[ขอสั่งจ่าย]) #form_title ดังรายการดังต่อไปนี้ 
#table(columns:(40pt,150pt,120pt,120pt,120pt,1fr),stroke:.5pt,
  table.header(
    strong[ลำดับ],strong[ชื่อตัวยาสำคัญ],strong[ชื่อการค้า],strong[ความแรง],strong[ขนาดที่ใช้],[#strong[จำนวนที่สั่งจ่าย]#linebreak()\(ระบุหน่วยเป็น Amp, Vial, Tab, Cap, Patch ฯลฯ\)],
  ),[๑],[#med_name],[],[#stregnth],[#order_item.order_item_detail],[#unit],
)
#v(20pt)
#h(20pt)ให้แก่ (ชื่อผู้รับการรักษาหรือชื่อเจ้าของสัตว์ซึ่งรับการรักษา) #patient.pname #patient.fname #patient.lname บัตรประจำตัวประชาชนหรือบัตรประจำตัวอื่นที่ทางราชการออกให้ เลขที่ #patient.cid #patient.passport_no ที่อยู่ เลขที่ #patient.homeaddr โทรศัพท์ #patient.hometel
#v(40pt)
#align(right,[(ลงมือชื่อ).....................................ผู้อออกใบสั่งจ่าย])
#v(20pt)
#underline(strong[หมายเหตุ :])
#text(12pt, [๑. ให้ขีดฆ่าข้อความที่ไม่ต้องการออก
๒. ให้ผู้รับการรักษาหรือเจ้าของสัตว์ซึ่งรับการรักษา ส่งมอบต้นฉบับของเอกสารฉบับนี้ให้แก่สถานพยาบาลที่จ่ายยาเสพติดให้โทษหรือวัตถุออกฤทธิ์ตามใบสั่งจ่ายให้ และให้สถานพยาบาลที่จ่ายยาเสพติดให้โทษหรือวัตถุออกฤทธิ์เก็บรับษาเอกสารฉบับนี้ไว้ เพื่อเป็นหลักฐานประกอบการจ่ายยาเสพติดให้โทษหรือวัตถุออกฤทธิ์
๓. ให้สถานพยาบาลที่สั่งจ่ายยาเสพติดให้โทษหรือวัตถุออกฤทธิ์นี้ เก็บรักษาสำเนาเอกสารฉบับนี้ไว้ เพื่อเป็นหลักฐานประกอบการจ่ายยาเสพติดให้โทษหรือวัตถุออกฤทธิ์])
#box(stroke: .5pt + black, text(12pt, [ผู้รับอนุญาตจำหน่าย จะจำหน่ายยาเสพติดให้โทษในประเภท ๒ หรือวัตถุออกฤทธิ์ในประเภท ๒ ให้แก่บุคคลภายนอกที่ไม่ใช่ผู้ป่วยหรือสัตว์ป่วยซึ่งตนให้การรักษาได้ เฉพาะกรณีที่มีใบสั่งจ่ายยาเสพติดให้โทษในประเภท ๒ หรือวัตถุออกฤทธิ์ในประเภท ๒ จากผู้ประกอบวิชาขีพเวชกรรม ผู้ประกอบวิชาขีพทันตกรรม หรือผู้ประกอบวิชาชีพการสัตวแพทย์ชั้นหนึ่ง แล้วแต่กรณี]))