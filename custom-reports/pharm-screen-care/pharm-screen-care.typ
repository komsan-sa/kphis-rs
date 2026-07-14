#import "templates/utils.typ": date_th, datetime_th
// PRELUDE
#let data = json("data.json")
#let rows = data.at("data",default: ())
// PREPARED FUNCTIONS
#let text_c(c) = align(center,c)
#let table_h(c) = [#align(center,strong(c))]
// RENDER
#set text(font:"TH Sarabun New",size:14pt)
#set page(paper:"a4",margin:(x:1cm,y:1.5cm),header-ascent:5pt,footer-descent:0pt,
  header: context[#h(1fr)#counter(page).display("1/1",both:true)],
)
#v(-20pt)
#h(1fr) #text(size:20pt,weight:700,align(center,[รายงาน การให้บริการเภสัชกรรมผู้ป่วยนอก#linebreak()ระหว่างวันที่ #date_th(data.start) ถึงวันที่ #date_th(data.end)]))#h(1fr)
#v(-30pt)
#table(columns:(30pt,70pt,50pt,120pt,1fr),stroke:.5pt,
  table.header(table_h[ลำดับ],table_h[วัน-เวลา],table_h[HN],table_h[ชื่อ-สกุล],table_h[รายละเอียด]),
  ..rows.enumerate(start:1).map(((i,row)) => {
    (text_c[#i],text_c(datetime_th(row.pharmacy_care_time)),text_c[#row.hn],[#row.pname#row.fname #row.lname],[#row.pharmacy_care])
  }).flatten()
)