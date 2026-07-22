#import "utils.typ": date_th, time_th, datetime_th, parse_d_t, note_type, explode_imgs
#let badge(m,c,n) = box(inset:(left:2pt),box(radius:3pt,outset:2pt,inset:(y:1pt),stroke:.3pt,[#text(10pt,m)#text(if n {6pt} else {10pt},top-edge:1pt,c)]))
#let card(c) = block(width:100%,spacing:0pt,above:8pt,below:5pt,c)
#let licenseno(s) = if s == none {none} else if s == "-99999" {none} else {s}
#let render_order(order, is_one) = {
  let meds = if is_one {("med","home-medication","injection","ivfluid")} else {("med","injection","ivfluid")}
  let is_confirm = order.order_confirm == "Y"
  let is_accepted = order.pharmacist_order_status == "accepted"
  let head = date_th(order.order_date)+" "+time_th(order.order_time)
  let types = if order.order_item_types.len() > 0 {
    order.order_item_types.map(ts => {
      let type_head = if ts.order_item_type == "home-medication" [#linebreak()#underline(offset:2pt,strong("Home Medication"))#linebreak()] else []
      let items = if ts.order_items.len() > 0 {
        ts.order_items.map(item => {
          if item.order_item_type == "off" [
            - *Off* #if item.off_med_name != none [#strong(item.off_med_name) #if item.off_order_item_detail != none {linebreak()}] #item.off_order_item_detail
          ] else {
            let medname = [#strong(item.med_name) #if item.order_item_type == "home-medication" {strong[ \##item.first_qty]} #if item.med_name != none and item.order_item_detail != none {linebreak()} #item.order_item_detail]
            let index_plans = item.index_plans.filter(p => p.plan_time != none)
            let plans = if index_plans.len() > 0 {
              index_plans.map(plan => {
                let index_actions = plan.actions.filter(a => a.action_time != none)
                let actions_len = index_actions.len()
                if actions_len == 0 []
                else if actions_len == 1 {
                    let action = index_actions.first()
                    if action.action_date == order.order_date {
                      badge(sym.checkmark,[#time_th(action.action_time) #text(fill:red,weight:700,action.action_result)],false)}
                    else {badge(sym.checkmark,[#date_th(action.action_date) #time_th(action.action_time) #text(fill:red,weight:700,action.action_result)],false)}
                } else if plan.plan_sch_type == "date" {
                  badge(sym.checkmark,"PRN x" + str(actions_len),false)
                } else {
                  badge(sym.checkmark,time_th(plan.plan_time) + " x" + str(actions_len),false)
                }
              }).join(h(5pt))
            } else {""}
            if item.off_by_datetime != none {medname = strike(medname)}
            if meds.contains(item.order_item_type) {medname = text(fill:blue,medname)}
            let line = [#medname #if item.stat == "Y" [*(Stat)*] #if item.off_by_datetime != none [*(Off #datetime_th(item.off_by_datetime))*] #if item.allergy_agent_symptom != none {text(fill:red,[*แพ้ยา/เฝ้าระวัง*])} #plans]
            if item.order_item_type == "home-medication" [+ #line] else [- #line]
          }
        }).join()
      } else []
      type_head+items
    }).join()
  } else []
  let dr = [#if order.order_doctor_is_intern == true [\(Intern\) ] #order.order_doctor_name #licenseno(order.order_doctor_licenseno)#linebreak()#order.order_doctor_entryposition #date_th(order.order_date) #time_th(order.order_time)]
  let ns = if order.nurse_order_as_name != none [#linebreak()รคส. #if order.nurse_order_as_is_intern == true [\(Intern\) ] #order.nurse_order_as_name #licenseno(order.nurse_order_as_licenseno)#linebreak()#order.nurse_order_as_entryposition]
  else if order.nurse_accept_time != none [#linebreak()\(RN\) #order.nurse_accept_name #licenseno(order.nurse_accept_licenseno)#linebreak()#order.nurse_accept_entryposition #datetime_th(order.nurse_accept_time)]
  else []
  let py_acpt = if order.pharmacist_done_time == none and order.pharmacist_check_time == none and order.pharmacist_accept_time != none [#linebreak()\(ห้องยารับรายการ\) #order.pharmacist_accept_name #licenseno(order.pharmacist_accept_licenseno)#linebreak()#order.pharmacist_accept_entryposition #datetime_th(order.pharmacist_accept_time)] else []
  let py_check = if order.pharmacist_done_time == none and order.pharmacist_check_time != none [#linebreak()\(ตรวจสอบ\) #order.pharmacist_check_name #licenseno(order.pharmacist_check_licenseno)#linebreak()#order.pharmacist_check_entryposition #datetime_th(order.pharmacist_check_time)] else []
  let py_done = if order.pharmacist_done_time != none [#linebreak()\(Rx\) #order.pharmacist_done_name #licenseno(order.pharmacist_done_licenseno)#linebreak()#order.pharmacist_done_entryposition #datetime_th(order.pharmacist_done_time)] else []
  let signs = align(right,text(size:10pt,v(5pt)+dr+ns+py_acpt+py_check+py_done))
  card(text(weight:700,size:14pt,head)+types+signs)
}
#let render_note(note) = if note.progress_note_owner_type == "auditor" {none} else {
  let note_date = [#date_th(note.progress_note_date) #time_th(note.progress_note_time)]
  let types = if note.progress_note_item_types.len() > 0 {
    note.progress_note_item_types.map(ts => {
      let items = if ts.progress_note_items.len() > 0 {
        ts.progress_note_items.map(item => [- #item.progress_note_item_detail #if item.progress_note_item_detail_2 != none [#linebreak()#item.progress_note_item_detail_2]]).join()
      } else []
      text(weight:700,underline(offset:2pt,note_type(ts.progress_note_item_type)))+items
    }).join()
  } else []
  let noter = align(right,text(size:10pt,[#if note.order_doctor_is_intern == true [\(Intern\) ] #note.order_doctor_name #licenseno(note.doctor_licenseno)#linebreak()#note.entryposition #note_date]))
  card(text(weight:700,size:14pt,note_date)+linebreak()+types+explode_imgs(2,false,note.imgs)+v(5pt)+noter)
}
#let grouper(s,e,datas,is_note) = {
  let group = ()
    for data in datas {
    let dt = if is_note {
      parse_d_t(data.progress_note_date,data.progress_note_time)
    } else {
      parse_d_t(data.order_date,data.order_time)
    }
    if dt >= s and dt < e {
      group.push(data)
    }
  }
  group
}
#let get_rows(o,c,n) = {
  let dt_o = o.map(od => parse_d_t(od.order_date,od.order_time))
  let dt_c = c.map(od => parse_d_t(od.order_date,od.order_time))
  let dt_n = n.map(nt => parse_d_t(nt.progress_note_date,nt.progress_note_time))
  let dts = (dt_o,dt_c,dt_n).flatten().sorted()
  let rows = ()
  let start = none
  for dt in dts {
    if start == none {
      start = dt
    } else if (dt - start).hours() > 3 {
      rows.push((start,dt))
      start = dt
    }
  }
  rows.push((start, start + duration(hours:3)))
  let groups = ()
  for r in rows {
    let (s,e) = r
    let r_o = grouper(s,e,o,false).map(od => render_order(od,true)).join(line(length:100%,stroke:0.5pt+gray))
    let r_c = grouper(s,e,c,false).map(od => render_order(od,false)).join(line(length:100%,stroke:0.5pt+gray))
    let r_n = grouper(s,e,n,true).map(nt => render_note(nt)).join(line(length:100%,stroke:0.5pt+gray))
    groups.push((r_n,r_o,r_c))
  }
  groups
}