#import "customs/config.typ": hospital-name, hospital-address
#import "templates/utils.typ": cid, date_th_full
#let data = json("data.json")
#let is_addict = data.at("is_addict",default:false)
#let patient = data.at("patient",default: none)
#let order_item = data.at("order_item",default: none)
// PREPARED FUNCTIONS
#let table_h(c) = [#align(center + horizon,strong(c))]
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
    (tuples.join(" "),[#st_1 #st_2],unit)
  } else {
    (none,none,none)
  }
}
#let (med_name, stregnth, unit) = split_med_name(order_item.med_name) 
// RENDER
#let form_title = if is_addict {"ยาเสพติดให้โทษในประเภท ๒"} else {"วัตถุออกฤทธิ์ในประเภท ๒"}
#set text(font:"TH Sarabun New",size:14pt)
#set page(paper:"a4",margin:(x:1.5cm,y:2cm))
#align(center,text(size:18pt,weight:700,[ใบสั่งจ่าย#form_title]))
#v(20pt)
#grid(columns:(1fr,1fr),[],[วันที่ #date_th_full(order_item.order_date)])
#v(10pt)
#h(55pt)ข้าพเจ้า #order_item.order_doctor_name ใบอนุญาตประกอบวิชาชีพ#license_type(order_item.order_doctor_licenseno) #order_item.order_doctor_licenseno ปฏิบัติงาน ณ สถานพยาบาลชื่อ #hospital-name ตั้งอยู่ที่ #hospital-address
#v(0pt)
#h(55pt)#underline(strong[ขอสั่งจ่าย]) #form_title ดังรายการดังต่อไปนี้ 
#table(columns:(35pt,100pt,80pt,80pt,80pt,1fr),stroke:.5pt,
  table.header(
    table_h[ลำดับ],table_h[ชื่อตัวยาสำคัญ],table_h[ชื่อการค้า],table_h[ความแรง],table_h[ขนาดที่ใช้],align(center,[#strong[จำนวนที่สั่งจ่าย]#linebreak()\(ระบุหน่วยเป็น Amp, Vial, Tab, Cap, Patch ฯลฯ\)]),
  ),align(center,[๑]),[#med_name],[],align(center,[#stregnth]),[#order_item.order_item_detail],align(center,[#unit]),
)
#v(10pt)
#h(55pt)ให้แก่ (ชื่อผู้รับการรักษาหรือชื่อเจ้าของสัตว์ซึ่งรับการรักษา) #patient.pname #patient.fname #patient.lname บัตรประจำตัวประชาชนหรือบัตรประจำตัวอื่นที่ทางราชการออกให้ เลขที่ #cid(patient.cid) #patient.passport_no ที่อยู่ เลขที่ #patient.homeaddr โทรศัพท์ #patient.hometel
#v(40pt)
#align(right,box(width: 280pt, [#align(center,[(ลงมือชื่อ)..............................................ผู้ออกใบสั่งจ่าย])
  #align(center, [(#order_item.order_doctor_name)#h(20pt)])]))
#v(10pt)
#grid(columns:(30pt,1fr),[],text(12pt,[#underline(strong[หมายเหตุ]) :
1. ให้ขีดฆ่าข้อความที่ไม่ต้องการออก
2. ให้ผู้รับการรักษาหรือเจ้าของสัตว์ซึ่งรับการรักษา ส่งมอบต้นฉบับของเอกสารฉบับนี้ให้แก่สถานพยาบาลที่จ่ายยาเสพติดให้โทษหรือวัตถุออกฤทธิ์ตามใบสั่งจ่ายให้ และให้สถานพยาบาลที่จ่ายยาเสพติดให้โทษหรือวัตถุออกฤทธิ์เก็บรับษาเอกสารฉบับนี้ไว้ เพื่อเป็นหลักฐานประกอบการจ่ายยาเสพติดให้โทษหรือวัตถุออกฤทธิ์
3. ให้สถานพยาบาลที่สั่งจ่ายยาเสพติดให้โทษหรือวัตถุออกฤทธิ์เก็บรักษาสำเนาเอกสารฉบับนี้ไว้ เพื่อเป็นหลักฐานประกอบการจ่ายยาเสพติดให้โทษหรือวัตถุออกฤทธิ์
]))
#box(stroke: .5pt + black, inset: 5pt, text(12pt, [ผู้รับอนุญาตจำหน่าย จะจำหน่ายยาเสพติดให้โทษในประเภท ๒ หรือวัตถุออกฤทธิ์ในประเภท ๒ ให้แก่บุคคลภายนอกที่ไม่ใช่ผู้ป่วยหรือสัตว์ป่วยซึ่งตนให้การรักษาได้ เฉพาะกรณีที่มีใบสั่งจ่ายยาเสพติดให้โทษในประเภท ๒ หรือวัตถุออกฤทธิ์ในประเภท ๒ จากผู้ประกอบวิชาขีพเวชกรรม ผู้ประกอบวิชาขีพทันตกรรม หรือผู้ประกอบวิชาชีพการสัตวแพทย์ชั้นหนึ่ง แล้วแต่กรณี]))