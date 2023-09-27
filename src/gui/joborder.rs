use crate::get_job_query_result;
use crate::get_query_rule_id;
use eframe::egui::{self, Context, Ui};
use egui_extras::{Column, TableBuilder};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
};
use tokio::runtime::Runtime;
use crate::ErrDlg;

pub struct JobOrder {
    tx: Sender<(Vec<String>, String)>,
    rx: Receiver<(Vec<String>, String)>,
    tx_1: Sender<Vec<Vec<String>>>,
    rx_1: Receiver<Vec<Vec<String>>>,
    job: String,
    much: String,
    teble_data: (Vec<String>, String),
    teble_data_1: Vec<Vec<String>>,
    tip: ErrDlg,
    sn_none: bool,
    much_none: bool,
    much_0: bool,
    job_err: bool,
    much_err: bool,
    add_ok: bool,
    b_1: bool,
    b_2: bool,
}

impl JobOrder {
    pub fn new(_ctx: &Context) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let (tx_1, rx_1) = std::sync::mpsc::channel();
        let tip = ErrDlg::new();
        let sn_none = false;
        let much_none = false;
        let job_err = false;
        let much_0 = false;
        let much_err = false;
        let add_ok = false;
        let b_1 = false;
        let b_2 = false;
        JobOrder {
            job: Default::default(),
            much: Default::default(),
            teble_data_1: Default::default(),
            tip,
            sn_none,
            much_none,
            job_err,
            much_0,
            teble_data: Default::default(),
            much_err,
            add_ok,
            b_1,
            b_2,
            tx,
            rx,
            tx_1,
            rx_1,
        }
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui, _frame: &mut eframe::Frame) {
        if let Ok(res) = self.rx.try_recv() {
            self.teble_data = res;
        }
        if let Ok(res) = self.rx_1.try_recv() {
            self.teble_data_1 = res;
        }
        let tx = self.tx.clone();
        let ctx_1 = ctx.clone();
        let ctx_2 = ctx.clone();
        let tx_1 = self.tx_1.clone();
        ui.horizontal(|ui| {
            // ui.horizontal(|ui| {
            ui.label("工单");
            ui.add(egui::TextEdit::singleline(&mut self.job).hint_text("输入工单号 "));
            // });

            if ui.add(egui::Button::new("工单查询")).clicked() {
                if self.job.is_empty() {
                    self.sn_none = true;
                    return;
                } else if self.job.len() != 6 {
                    self.job_err = true;
                    return;
                }
                let j = self.job.clone();
                get_job_query_result(j, tx, ctx_1);
                self.b_1 = true;
            }
            ui.label("数量");
            // ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.much).hint_text("输入生成数量 "));
            // });
            if ui
                .add_enabled(self.b_1, egui::Button::new("条码生成"))
                .clicked()
            {
                if self.much.is_empty() {
                    self.much_none = true;
                } else if self.much.parse::<i32>().unwrap() <= 0 {
                    self.much_0 = true;
                }
                let infos = self.teble_data.clone().0;
                let s = infos.get(1).unwrap().parse::<i32>().unwrap();
                let mcu = self.much.clone().parse::<i32>().unwrap();
                if s < mcu {
                    self.much_err = true;
                } else {
                    get_query_rule_id(infos, mcu, tx_1, ctx_2);
                    self.add_ok = true;
                }
                self.b_2 = true;
            }
            // ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add_enabled(self.b_2, egui::Button::new("条码打印"))
                .clicked()
            {
                let rt = Runtime::new().unwrap();
                let _res = rt.block_on(async {
                    let z = get_libraries().await.unwrap();
                    let b = z.body.as_array().unwrap().get(1).unwrap();
                    let y = b.get("id").unwrap().to_string();
                    let s: String = y
                        .chars()
                        .map(|x| match x {
                            '"' => ' ',
                            '\\' => ' ',
                            _ => x,
                        })
                        .collect();
                    loop {
                        let s_1 = s.clone();
                        let muc = self.much.clone().parse::<i32>().unwrap();
                        let job = self.job.clone();
                        let sn_id = self
                            .teble_data_1
                            .clone()
                            .get(0)
                            .unwrap()
                            .get(0)
                            .unwrap()
                            .to_string();
                        let pn = self
                            .teble_data_1
                            .clone()
                            .get(0)
                            .unwrap()
                            .get(3)
                            .unwrap()
                            .to_string();
                        let x = print(job, muc, sn_id, pn, s_1).await;
                        if x == true {
                            break;
                        }
                    }
                });
                self.b_1 = false;
            }
            // });
            // });
            // ui.horizontal(|ui| {
            let x = self.teble_data.clone();
            ui.label(x.1);
        });
        ui.separator();
        let text_size = egui::TextStyle::Body.resolve(ui.style()).size + 10.0;
        let words: Vec<String> = vec![
            "SN".to_string(),
            "序号".to_string(),
            "工单".to_string(),
            "料号".to_string(),
        ];

        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::bottom_up(egui::Align::Center))
            .column(Column::initial(350.0).at_least(40.0))
            .column(Column::initial(350.0).at_least(40.0))
            .column(Column::initial(350.0).at_least(40.0))
            .column(Column::remainder())
            .header(20.0, |mut header| {
                for word in words {
                    header.col(|ui| {
                        ui.heading(word);
                    });
                }
            })
            .body(|mut body| {
                let infos = self.teble_data_1.clone();
                for info in infos {
                    body.row(text_size, |mut row| {
                        for data in info {
                            row.col(|ui| {
                                ui.label(data);
                            });
                        }
                    })
                }
            });
        if self.sn_none == true {
            let x = self.sn_none.clone();
            let t = self.tip.sn_none_show(ctx, x);
            if t == false {
                self.sn_none = false;
            }
        }
        if self.much_none == true {
            let x = self.much_none.clone();
            let t = self.tip.much_none_show(ctx, x);
            if t == false {
                self.much_none = false;
            }
        }
        if self.job_err == true {
            let x = self.job_err.clone();
            let t = self.tip.job_err_show(ctx, x);
            if t == false {
                self.job_err = false;
            }
        }
        if self.much_0 == true {
            let x = self.much_0.clone();
            let t = self.tip.much_0_show(ctx, x);
            if t == false {
                self.much_0 = false;
            }
        }
        if self.much_err == true {
            let x = self.much_err.clone();
            let t = self.tip.much_err_show(ctx, x);
            if t == false {
                self.much_err = false;
            }
        }
        if self.add_ok == true {
            let x = self.add_ok.clone();
            let t = self.tip.add_ok_show(ctx, x);
            if t == false {
                self.add_ok = false;
            }
        };
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Req {
    url: String,
    status: u16,
    headers: HashMap<String, String>,
    body: serde_json::Value,
}

async fn get_libraries() -> Result<Req, Error> {
    let url = "http://localhost/BarTender/api/v1/libraries".to_string();
    let request_url = url.clone();
    let client = reqwest::Client::new();
    let mut hm = HashMap::new();
    let res = client.get(url).send().await.unwrap();
    for (key, val) in res.headers().into_iter() {
        hm.insert(
            key.as_str().to_owned(),
            val.to_str().ok().unwrap_or("").to_owned(),
        );
    }
    let req = Req {
        status: res.status().as_u16(),
        url: request_url,
        body: res.json().await.unwrap(),
        headers: hm,
    };
    Ok(req)
}
async fn print(j: String, m: i32, sn_id: String, pn: String, l: String) -> bool {
    let l_id = l.clone();
    let pn_info = &pn[4..];
    let job = j.clone();
    let sn = sn_id.clone();
    let quantity = &sn[12..];
    let much = m.clone() / 5;
    if m % 5 == 0 {
        let data = json!({
        "LibraryID": format!("{}",l_id),
        // "AbsolutePath": "global_test.btw",
        "relativePath": "global_test.btw",
        //"printRequestID": "6af59a34-898d-4ad7-a189-c752111d8062",
        "printRequestID": "d879b64b-4060-46d3-acf5-0f4130b675c1",
        "printer": "TSC TX600",
        "StartingPosition": 1,
        "Copies": 1,
        "SerialNumbers": format!("{}",much),
        "DataEntryControls": {
            "PN":format!("{}",pn_info),
            "JOBORDER":format!("{}",job),
            "QUANTITY":format!("{}",quantity),
        }
            });
        let url = "http://localhost/BarTender/api/v1/print".to_string();
        let request_url = url.clone();
        let client = reqwest::Client::new();
        let mut hm = HashMap::new();
        let res = client.post(url).json(&data).send().await.unwrap();
        for (key, val) in res.headers().into_iter() {
            hm.insert(
                key.as_str().to_owned(),
                val.to_str().ok().unwrap_or("").to_owned(),
            );
        }
        let req = Req {
            status: res.status().as_u16(),
            url: request_url,
            body: res.json().await.unwrap(),
            headers: hm,
        };
        let b = req.body.get("success").unwrap().as_bool().unwrap();
        println!("{:?}", req.body);
        b
    } else {
        let data = json!({
        "LibraryID": format!("{}",l_id),
        // "AbsolutePath": "global_test.btw",
        "relativePath": "global_test.btw",
        "printRequestID": "d879b64b-4060-46d3-acf5-0f4130b675c1",
        "printer": "TSC TX600",
        "StartingPosition": 1,
        "Copies": 1,
        "SerialNumbers": format!("{}",much + 1),
        "DataEntryControls": {
            "PN":format!("{}",pn_info),
            "JOBORDER":format!("{}",job),
            "QUANTITY":format!("{}",quantity),
        }
            });
        let url = "http://localhost/BarTender/api/v1/print".to_string();
        let request_url = url.clone();
        let client = reqwest::Client::new();
        let mut hm = HashMap::new();
        let res = client.post(url).json(&data).send().await.unwrap();
        for (key, val) in res.headers().into_iter() {
            hm.insert(
                key.as_str().to_owned(),
                val.to_str().ok().unwrap_or("").to_owned(),
            );
        }
        let req = Req {
            status: res.status().as_u16(),
            url: request_url,
            body: res.json().await.unwrap(),
            headers: hm,
        };
        let b = req.body.get("success").unwrap().as_bool().unwrap();
        println!("{:?}", req.body);
        b
    }
}
