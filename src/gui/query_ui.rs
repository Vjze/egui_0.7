use std::sync::mpsc::{Receiver, Sender};

use eframe::egui::{self, *};
use egui_extras::{Column, TableBuilder};

use tokio::{
    fs,
    io::{AsyncBufReadExt, BufReader},
    runtime::Runtime,
};
use crate::ErrDlg;
use crate::Enum;
use crate::DataInfo;
use crate::get_sn_result;
use crate::get_box_none_result;
use crate::get_box_result;
use crate::get_carton_none_result;
use crate::get_carton_result;
use crate::get_much_sn;
use crate::get_much_carton;
use crate::get_worker;
use crate::universal_file_out;
use crate::CustomError;

pub struct Query {
    tx: Sender<Vec<Vec<String>>>,
    rx: Receiver<Vec<Vec<String>>>,
    tx_1: Sender<(Vec<Vec<String>>, usize)>,
    rx_1: Receiver<(Vec<Vec<String>>, usize)>,
    tx_2: Sender<(Vec<Vec<String>>, i32)>,
    rx_2: Receiver<(Vec<Vec<String>>, i32)>,
    tx_3: Sender<f64>,
    rx_3: Receiver<f64>,
    tx_4: Sender<(Vec<DataInfo>, usize)>,
    rx_4: Receiver<(Vec<DataInfo>, usize)>,
    sn: String,
    check: bool,
    radio: Enum,
    date: Option<chrono::NaiveDate>,
    date_1: Option<chrono::NaiveDate>,
    teble_data: Vec<Vec<String>>,
    teble_data_1: (Vec<Vec<String>>, usize),
    teble_data_2: (Vec<Vec<String>>, i32),
    teble_data_4: (Vec<DataInfo>, usize),
    out_button: bool,
    tip: ErrDlg,
    cerr: String,
    sn_none: bool,
    file_err: bool,
    file_sn_err: bool,
    file_sn_none: bool,
    file_carton_none: bool,
    file_out: bool,
    enabled: bool,
    // ac:Instant,
    end: f64,
}

impl Query {
    pub fn new(_ctx: &Context) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let (tx_1, rx_1) = std::sync::mpsc::channel();
        let (tx_2, rx_2) = std::sync::mpsc::channel();
        let (tx_3, rx_3) = std::sync::mpsc::channel();
        let (tx_4, rx_4) = std::sync::mpsc::channel();
        let tip = ErrDlg::new();
        let sn_none = false;
        let file_err = false;
        let file_sn_err = false;
        let file_sn_none = false;
        let file_carton_none = false;
        let file_out = false;
        Query {
            sn: Default::default(),
            check: false,
            radio: Enum::default(),
            date: None,
            teble_data: Default::default(),
            teble_data_1: Default::default(),
            teble_data_2: Default::default(),
            teble_data_4: Default::default(),
            out_button: false,
            tip,
            cerr: Default::default(),
            sn_none,
            file_err,
            file_sn_err,
            file_sn_none,
            file_carton_none,
            file_out,
            date_1: None,
            enabled: true,
            tx,
            rx,
            tx_1,
            rx_1,
            tx_2,
            rx_2,
            end: 0.,
            // ac,
            tx_3,
            rx_3,
            tx_4,
            rx_4,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut Ui, frame: &mut eframe::Frame) {
        // let i = Instant::now();
        if let Ok(res) = self.rx.try_recv() {
            self.teble_data = res;
        }
        if let Ok(res) = self.rx_1.try_recv() {
            self.teble_data_1 = res;
        }
        if let Ok(res) = self.rx_2.try_recv() {
            self.teble_data_2 = res;
        }
        if let Ok(res) = self.rx_3.try_recv() {
            self.end = res;
        }
        if let Ok(res) = self.rx_4.try_recv() {
            self.teble_data_4 = res;
        }
        let c = ctx.clone();
        let s = self.sn.clone();
        let tx = self.tx.clone();
        let tx_1 = self.tx_1.clone();
        let tx_2 = self.tx_2.clone();
        let tx_3 = self.tx_3.clone();
        let tx_4 = self.tx_4.clone();
        ui.add_enabled_ui(self.enabled, |ui| {
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("选择查询项")
                        .selected_text(format!("{:?}", self.radio))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.radio, Enum::SN, "SN查询");
                            ui.selectable_value(&mut self.radio, Enum::BoxNo, "盒号查询");
                            ui.selectable_value(&mut self.radio, Enum::CartonNo, "箱号查询");
                            ui.selectable_value(&mut self.radio, Enum::WorkerId, "工号查产量");
                            ui.selectable_value(&mut self.radio, Enum::MultipleSn, "多SN查询");
                            ui.selectable_value(
                                &mut self.radio,
                                Enum::MultipleCarton,
                                "多箱号查询",
                            );
                        });
                    // //
                    ui.checkbox(&mut self.check, "以日期查询");
                    let date = self
                        .date
                        .get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                    let date_1 = self
                        .date_1
                        .get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                    if self.check == true {
                        ui.add(egui_extras::DatePickerButton::new(date).id_source("0"));
                        ui.add(egui_extras::DatePickerButton::new(date_1).id_source("1"));
                    }
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let input = egui::TextEdit::singleline(&mut self.sn)
                            .hint_text("输入SN号或者盒号或者箱号").show(ui);
                    let input_id = input.response.id;
                    if input.response.lost_focus()&&ui.input(|i|i.key_pressed(egui::Key::Enter)){
                        match self.radio {
                            Enum::SN => {
                                self.out_button = false;
                                if self.sn.is_empty() {
                                    self.sn_none = true;
                                    return;
                                } else {
                                    get_sn_result(s.clone(), tx_3.clone(), tx.clone(), c.clone());
                                }
                            }

                            Enum::BoxNo => {
                                self.out_button = false;
                                if self.sn.is_empty() {
                                    if self.check == true {
                                        let date_1 = self
                                            .date
                                            .clone()
                                            .unwrap()
                                            .format("%Y-%m-%d 00:00:00")
                                            .to_string();
                                        let date_2 = self
                                            .date_1
                                            .clone()
                                            .unwrap()
                                            .format("%Y-%m-%d 23:59:59")
                                            .to_string();
                                        get_box_none_result(date_1, date_2, tx_3.clone(), tx_1.clone(), c.clone());
                                    } else {
                                        self.sn_none = true;
                                        return;
                                    };
                                } else {
                                    get_box_result(s.clone(), tx_3.clone(), tx_4.clone(), c.clone());
                                }
                            }
                            Enum::CartonNo => {
                                self.out_button = true;
                                if self.sn.is_empty() {
                                    if self.check == true {
                                        let date_1 = self
                                            .date
                                            .clone()
                                            .unwrap()
                                            .format("%Y-%m-%d 00:00:00")
                                            .to_string();
                                        let date_2 = self
                                            .date_1
                                            .clone()
                                            .unwrap()
                                            .format("%Y-%m-%d 23:59:59")
                                            .to_string();
                                        get_carton_none_result(date_1, date_2, tx_3.clone(), tx_2.clone(), c.clone());
                                    } else {
                                        self.sn_none = true;
                                        return;
                                    }
                                } else {
                                    get_carton_result(s.clone(), tx_3.clone(), tx_1.clone(), c.clone())
                                }
                            }
                            Enum::MultipleSn => {
                                self.out_button = false;
                                let mut vec_sn = vec![];
                                // tokio::spawn(async move{
                                let rt = Runtime::new().unwrap();
                                rt.block_on(async {
                                    let res = self.get_sn_file(ui, frame).await;
                                    match res {
                                        Ok(mut value) => {
                                            while let Ok(Some(line)) = value.next_line().await {
                                                vec_sn.push(line)
                                            }
                                        }
                                        Err(err) => {
                                            self.cerr = err.to_string();
                                            self.file_sn_err = true;
                                            return;
                                        }
                                    }
                                });
                                if vec_sn.is_empty() {
                                    self.file_sn_none = true;
                                    return;
                                }
                                get_much_sn(vec_sn, tx_3.clone(), tx.clone(), c.clone())
                            }
                            Enum::MultipleCarton => {
                                let mut vec_carton = vec![];
                                let rt = Runtime::new().unwrap();
                                rt.block_on(async {
                                    let res = self.get_carton_file(ui, frame).await;

                                    match res {
                                        Ok(mut value) => {
                                            while let Ok(Some(line)) = value.next_line().await {
                                                vec_carton.push(line)
                                            }
                                        }
                                        Err(err) => {
                                            self.cerr = err.to_string();
                                            self.file_err = true;
                                            return;
                                        }
                                    }
                                });

                                if vec_carton.is_empty() {
                                    self.file_carton_none = true;
                                    return;
                                }
                                self.out_button = true;
                                get_much_carton(vec_carton, tx_3.clone(), tx_1.clone(), c.clone())
                            }
                            Enum::WorkerId => {
                                if self.sn.is_empty() {
                                    self.sn_none = true;
                                }
                                self.out_button = false;
                                let date_1 = self
                                    .date
                                    .clone()
                                    .unwrap()
                                    .format("%Y-%m-%d 00:00:00")
                                    .to_string(); //.format("%Y-%m-%d 00:00:00").to_string();
                                let date_2 = self
                                    .date_1
                                    .clone()
                                    .unwrap()
                                    .format("%Y-%m-%d 23:59:59")
                                    .to_string(); //.format("%Y-%m-%d 23:59:59").to_string();
                                get_worker(s.clone(), date_1, date_2, tx_3.clone(), tx_1.clone(), c.clone())
                            }
                        }
                };
                    if ui.button("ｘ").clicked() {
                        self.sn.clear();
                        self.teble_data.clear();
                        self.teble_data_1.0.clear();
                        self.teble_data_2.0.clear();
                    }
                });
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    if ui.add(egui::Button::new("查询")).clicked() {
                        match self.radio {
                            Enum::SN => {
                                self.out_button = false;
                                if self.sn.is_empty() {
                                    self.sn_none = true;
                                    return;
                                } else {
                                    get_sn_result(s, tx_3, tx, c);
                                }
                            }

                            Enum::BoxNo => {
                                self.out_button = false;
                                if self.sn.is_empty() {
                                    if self.check == true {
                                        let date_1 = self
                                            .date
                                            .clone()
                                            .unwrap()
                                            .format("%Y-%m-%d 00:00:00")
                                            .to_string();
                                        let date_2 = self
                                            .date_1
                                            .clone()
                                            .unwrap()
                                            .format("%Y-%m-%d 23:59:59")
                                            .to_string();
                                        get_box_none_result(date_1, date_2, tx_3, tx_1, c);
                                    } else {
                                        self.sn_none = true;
                                        return;
                                    };
                                } else {
                                    get_box_result(s, tx_3, tx_4, c);
                                }
                            }
                            Enum::CartonNo => {
                                self.out_button = true;
                                if self.sn.is_empty() {
                                    if self.check == true {
                                        let date_1 = self
                                            .date
                                            .clone()
                                            .unwrap()
                                            .format("%Y-%m-%d 00:00:00")
                                            .to_string();
                                        let date_2 = self
                                            .date_1
                                            .clone()
                                            .unwrap()
                                            .format("%Y-%m-%d 23:59:59")
                                            .to_string();
                                        get_carton_none_result(date_1, date_2, tx_3, tx_2, c);
                                    } else {
                                        self.sn_none = true;
                                        return;
                                    }
                                } else {
                                    get_carton_result(s, tx_3, tx_1, c)
                                }
                            }
                            Enum::MultipleSn => {
                                self.out_button = false;
                                let mut vec_sn = vec![];
                                // tokio::spawn(async move{
                                let rt = Runtime::new().unwrap();
                                rt.block_on(async {
                                    let res = self.get_sn_file(ui, frame).await;
                                    match res {
                                        Ok(mut value) => {
                                            while let Ok(Some(line)) = value.next_line().await {
                                                vec_sn.push(line)
                                            }
                                        }
                                        Err(err) => {
                                            self.cerr = err.to_string();
                                            self.file_sn_err = true;
                                            return;
                                        }
                                    }
                                });
                                if vec_sn.is_empty() {
                                    self.file_sn_none = true;
                                    return;
                                }
                                get_much_sn(vec_sn, tx_3, tx, c)
                            }
                            Enum::MultipleCarton => {
                                let mut vec_carton = vec![];
                                let rt = Runtime::new().unwrap();
                                rt.block_on(async {
                                    let res = self.get_carton_file(ui, frame).await;

                                    match res {
                                        Ok(mut value) => {
                                            while let Ok(Some(line)) = value.next_line().await {
                                                vec_carton.push(line)
                                            }
                                        }
                                        Err(err) => {
                                            self.cerr = err.to_string();
                                            self.file_err = true;
                                            return;
                                        }
                                    }
                                });

                                if vec_carton.is_empty() {
                                    self.file_carton_none = true;
                                    return;
                                }
                                self.out_button = true;
                                get_much_carton(vec_carton, tx_3, tx_1, c)
                            }
                            Enum::WorkerId => {
                                if self.sn.is_empty() {
                                    self.sn_none = true;
                                }
                                self.out_button = false;
                                let date_1 = self
                                    .date
                                    .clone()
                                    .unwrap()
                                    .format("%Y-%m-%d 00:00:00")
                                    .to_string(); //.format("%Y-%m-%d 00:00:00").to_string();
                                let date_2 = self
                                    .date_1
                                    .clone()
                                    .unwrap()
                                    .format("%Y-%m-%d 23:59:59")
                                    .to_string(); //.format("%Y-%m-%d 23:59:59").to_string();
                                get_worker(s, date_1, date_2, tx_3, tx_1, c)
                            }
                        }
                    };
                    if ui
                        .add_enabled(self.out_button, egui::Button::new("导出到 Excel"))
                        .clicked()
                    {
                        let res = self.teble_data_1.clone();
                        let rec = universal_file_out(&res.0, res.1);
                        if rec.unwrap() == () {
                            self.file_out = true;
                        }
                    };
                });
                match self.radio {
                    Enum::SN => (),
                    Enum::BoxNo => {
                        if self.sn.is_empty() {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                let rec = self.teble_data_1.clone().1;
                                ui.add(egui::Label::new(format!("数量: {}", rec)));
                            });
                        } else {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                let res = self.teble_data_4.clone().1;
                                ui.add(egui::Label::new(format!("数量: {}", res)));
                            });
                        }
                    }
                    Enum::CartonNo => {
                        if self.sn.is_empty() {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                let rec = self.teble_data_2.clone().1;
                                ui.add(egui::Label::new(format!("数量: {}", rec)));
                            });
                        } else {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                let res = self.teble_data_1.clone().1;
                                ui.add(egui::Label::new(format!("数量: {}", res)));
                            });
                        }
                    }
                    Enum::MultipleSn => (),
                    Enum::MultipleCarton => {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            let res = self.teble_data_1.clone().1;
                            ui.add(egui::Label::new(format!("数量: {}", res)));
                        });
                    }
                    Enum::WorkerId => {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            let res = self.teble_data_1.clone().1;
                            ui.add(egui::Label::new(format!("数量: {}", res)));
                        });
                    }
                }
            });
            ui.separator();
            let text_size = egui::TextStyle::Body.resolve(ui.style()).size + 10.0;
            match self.radio {
                Enum::SN => {
                    let words: Vec<String> = vec![
                        "SN".to_string(),
                        "工号".to_string(),
                        "料号".to_string(),
                        "结果".to_string(),
                        "Ith".to_string(),
                        "Pf".to_string(),
                        "Vop".to_string(),
                        "Im".to_string(),
                        "Rs".to_string(),
                        "Sen".to_string(),
                        "Res".to_string(),
                        "ICC".to_string(),
                        "Idark".to_string(),
                        "Vbr".to_string(),
                        "IXtalk".to_string(),
                        "Kink".to_string(),
                        "测试时间".to_string(),
                    ];
                    let len: usize = words.len() - 1;
                    TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(egui::Layout::top_down_justified(egui::Align::Center))
                        .column(Column::exact(200.0))
                        .columns(Column::remainder(), len)
                        .min_scrolled_height(0.0)
                        .header(15.0, |mut header| {
                            for word in words {
                                header.col(|ui| {
                                    ui.heading(word);
                                });
                            }
                        })
                        .body(|mut body| {
                            let res = self.teble_data.clone();

                            for info in res {
                                body.row(text_size, |mut row| {
                                    for data in info {
                                        row.col(|ui| {
                                            ui.label(data);
                                        });
                                    }
                                })
                            }
                        });
                }

                Enum::BoxNo => {
                    if self.check == true {
                        let words: Vec<String> = vec![
                            "盒号".to_string(),
                            "Sn".to_string(),
                            "料号".to_string(),
                            "工单号".to_string(),
                            "操作员".to_string(),
                            "装盒时间".to_string(),
                        ];
                        let len: usize = words.len();
                        TableBuilder::new(ui)
                            .striped(true)
                            .cell_layout(egui::Layout::top_down_justified(egui::Align::Center))
                            .columns(Column::remainder(), len)
                            .header(20.0, |mut header| {
                                for word in words {
                                    header.col(|ui| {
                                        ui.heading(word);
                                    });
                                }
                            })
                            .body(|mut body| {
                                let res = self.teble_data_1.clone();
                                for info in &res.0 {
                                    body.row(text_size, |mut row| {
                                        for data in info {
                                            row.col(|ui| {
                                                ui.label(data);
                                            });
                                        }
                                    })
                                }
                            });
                    } else {
                        let words: Vec<String> = vec![
                            "盒号".to_string(),
                            "Sn".to_string(),
                            "料号".to_string(),
                            "工单号".to_string(),
                            "操作员".to_string(),
                            "装盒时间".to_string(),
                        ];
                        let len: usize = words.len();
                        TableBuilder::new(ui)
                            .striped(true)
                            .cell_layout(egui::Layout::bottom_up(egui::Align::Center))
                            .columns(Column::remainder(), len)
                            .header(20.0, |mut header| {
                                for word in words {
                                    header.col(|ui| {
                                        ui.heading(word);
                                    });
                                }
                            })
                            .body(|mut body| {
                                let res = self.teble_data_4.clone();

                                for info in &res.0 {
                                    body.row(text_size, |mut row| {
                                        row.col(|ui| {
                                            ui.label(info.pno());
                                        });
                                        row.col(|ui| {
                                            ui.label(info.sn());
                                        });
                                        row.col(|ui| {
                                            ui.label(info.pn());
                                        });
                                        row.col(|ui| {
                                            ui.label(info.workorder());
                                        });
                                        row.col(|ui| {
                                            ui.label(info.creator());
                                        });
                                        row.col(|ui| {
                                            ui.label(info.createtime());
                                        });
                                        ctx.request_repaint();
                                        // for data in info {
                                        //     row.col(|ui| {
                                        //         ui.label(data);
                                        //     });
                                        // }
                                    })
                                }
                            });
                    }
                }
                Enum::CartonNo => {
                    if self.check == true {
                        let words: Vec<String> = vec![
                            "箱号".to_string(),
                            "料号".to_string(),
                            "操作员".to_string(),
                            "装箱数量".to_string(),
                            "装箱时间".to_string(),
                        ];
                        let len: usize = words.len();
                        TableBuilder::new(ui)
                            .striped(true)
                            .cell_layout(egui::Layout::bottom_up(egui::Align::Center))
                            .columns(Column::remainder(), len)
                            .header(20.0, |mut header| {
                                for word in words {
                                    header.col(|ui| {
                                        ui.heading(word);
                                    });
                                }
                            })
                            .body(|mut body| {
                                let res = self.teble_data_2.clone();
                                for info in &res.0 {
                                    body.row(text_size, |mut row| {
                                        for data in info {
                                            row.col(|ui| {
                                                ui.label(data);
                                            });
                                        }
                                    })
                                }
                            });
                    } else {
                        let words: Vec<String> = vec![
                            "SN".to_string(),
                            "TestTemp".to_string(),
                            "Ith".to_string(),
                            "SE".to_string(),
                            "Po".to_string(),
                            "Vf".to_string(),
                            "Im".to_string(),
                            "Sen".to_string(),
                            "Box_No".to_string(),
                            "Carton_No".to_string(),
                        ];
                        let len: usize = words.len() - 1;
                        TableBuilder::new(ui)
                            .striped(true)
                            .cell_layout(egui::Layout::bottom_up(egui::Align::Center))
                            .column(Column::initial(300.0).range(40.0..=300.0).clip(true))
                            .columns(Column::remainder(), len)
                            .header(20.0, |mut header| {
                                for word in words {
                                    header.col(|ui| {
                                        ui.heading(word);
                                    });
                                }
                            })
                            .body(|mut body| {
                                let res = self.teble_data_1.clone();
                                for info in &res.0 {
                                    body.row(text_size, |mut row| {
                                        for data in info {
                                            row.col(|ui| {
                                                ui.label(data);
                                            });
                                        }
                                    })
                                }
                            });
                    }
                }
                Enum::MultipleSn => {
                    let words: Vec<String> = vec![
                        "SN".to_string(),
                        "ProductBill".to_string(),
                        "TestType".to_string(),
                        "Result".to_string(),
                        "Ith".to_string(),
                        "Pf".to_string(),
                        "Vop".to_string(),
                        "Im".to_string(),
                        "Rs".to_string(),
                        "Sen".to_string(),
                        "Res".to_string(),
                        "ICC".to_string(),
                        "Idark".to_string(),
                        "Vbr".to_string(),
                        "IXtalk".to_string(),
                        "Kink".to_string(),
                        "TestDate".to_string(),
                    ];
                    TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(egui::Layout::bottom_up(egui::Align::Center))
                        .column(Column::initial(150.0).at_least(40.0).clip(true))
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::remainder())
                        .min_scrolled_height(0.0)
                        .header(15.0, |mut header| {
                            for word in words {
                                header.col(|ui| {
                                    ui.heading(word);
                                });
                            }
                        })
                        .body(|mut body| {
                            let res = self.teble_data.clone();
                            for info in &res {
                                body.row(text_size, |mut row| {
                                    for data in info {
                                        row.col(|ui| {
                                            ui.label(data);
                                        });
                                    }
                                })
                            }
                        });
                }
                Enum::MultipleCarton => {
                    let words: Vec<String> = vec![
                        "SN".to_string(),
                        "TestTemp".to_string(),
                        "Ith".to_string(),
                        "SE".to_string(),
                        "Po".to_string(),
                        "Vf".to_string(),
                        "Im".to_string(),
                        "Sen".to_string(),
                        "Box_No".to_string(),
                        "Carton_No".to_string(),
                    ];

                    TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(egui::Layout::bottom_up(egui::Align::Center))
                        .column(Column::initial(300.0).range(40.0..=300.0).clip(true))
                        .column(Column::initial(100.0).range(40.0..=300.0).clip(true))
                        .column(Column::initial(100.0).range(40.0..=300.0).clip(true))
                        .column(Column::initial(100.0).range(40.0..=300.0).clip(true))
                        .column(Column::initial(100.0).range(40.0..=300.0).clip(true))
                        .column(Column::initial(100.0).range(40.0..=300.0).clip(true))
                        .column(Column::initial(100.0).range(40.0..=300.0).clip(true))
                        .column(Column::initial(100.0).range(40.0..=300.0).clip(true))
                        .column(Column::initial(300.0).range(40.0..=300.0).clip(true))
                        .column(Column::remainder())
                        .header(20.0, |mut header| {
                            for word in words {
                                header.col(|ui| {
                                    ui.heading(word);
                                });
                            }
                        })
                        .body(|mut body| {
                            let res = self.teble_data_1.clone();
                            for info in &res.0 {
                                body.row(text_size, |mut row| {
                                    for data in info {
                                        row.col(|ui| {
                                            ui.label(data);
                                        });
                                    }
                                })
                            }
                        });
                }
                Enum::WorkerId => {
                    let words: Vec<String> = vec![
                        "SN".to_string(),
                        "TestType".to_string(),
                        "Result".to_string(),
                        "Ith".to_string(),
                        "Pf".to_string(),
                        "Im".to_string(),
                        "Sen".to_string(),
                        "Res".to_string(),
                        "ICC".to_string(),
                        "Idark".to_string(),
                        "Kink".to_string(),
                        "TestDate".to_string(),
                    ];
                    TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(egui::Layout::bottom_up(egui::Align::Center))
                        .column(Column::initial(150.0).at_least(40.0).clip(true))
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::remainder())
                        .min_scrolled_height(0.0)
                        .header(15.0, |mut header| {
                            for word in words {
                                header.col(|ui| {
                                    ui.heading(word);
                                });
                            }
                        })
                        .body(|mut body| {
                            let res = self.teble_data_1.clone();
                            for info in &res.0 {
                                body.row(text_size, |mut row| {
                                    for data in info {
                                        row.col(|ui| {
                                            ui.label(data);
                                        });
                                    }
                                })
                            }
                        });
                }
            }
        });
        if self.sn_none == true {
            self.enabled = false;
            let x = self.sn_none.clone();
            let t = self.tip.sn_none_show(ctx, x);
            if t == false {
                self.enabled = true;
                self.sn_none = false;
            }
        }
        if self.file_err == true {
            let err = self.cerr.clone();
            let x = self.file_err.clone();
            let t = self.tip.file_err_show(ctx, err, x);
            if t == false {
                self.file_err = false;
            }
        }

        if self.file_sn_err == true {
            let err = self.cerr.clone();
            let x = self.file_sn_err.clone();
            let t = self.tip.file_sn_err_show(ctx, err, x);
            if t == false {
                self.file_sn_err = false;
            }
        }

        if self.file_sn_none == true {
            let x = self.file_sn_none.clone();
            let t = self.tip.file_sn_none_show(ctx, x);
            if t == false {
                self.file_sn_none = false;
            }
        }

        if self.file_carton_none == true {
            let x = self.file_carton_none.clone();
            let t = self.tip.file_none_show(ctx, x);
            if t == false {
                self.file_carton_none = false;
            }
        }
        if self.file_out == true {
            let x = self.file_out.clone();
            let t = self.tip.file_out_show(ctx, x);
            if t == false {
                self.file_out = false;
            }
        }
        // match self.run {
        //     Run::Ac => {
        //         self.ac = Instant::now();
        //     },
        //     Run::End => {
        //         let i = self.ac.clone();
        //                                 let end = i.elapsed().as_secs();
        //                                 println!("end = {}",end);
        //                                 self.end = end;
        //     },
        // }
        // let i = self.ac.clone();
        // let end = i.elapsed().as_secs();
        // println!("end = {}",end);
        // self.end = end;
    }
    async fn get_carton_file(
        &mut self,
        _ui: &mut Ui,
        _frame: &mut eframe::Frame,
    ) -> Result<tokio::io::Lines<tokio::io::BufReader<tokio::fs::File>>, CustomError> {
        let file_name = String::from("carton.txt");

        let op = fs::File::open(&file_name).await?;
        let reader = BufReader::new(op).lines();
        Ok(reader)
    }
    async fn get_sn_file(
        &mut self,
        _ui: &mut Ui,
        _frame: &mut eframe::Frame,
    ) -> Result<tokio::io::Lines<tokio::io::BufReader<tokio::fs::File>>, CustomError> {
        let file_name = String::from("SN.txt");
        let op = fs::File::open(&file_name).await?;
        let reader = BufReader::new(op).lines();
        Ok(reader)
    }
    // pub fn get_ac(&mut self)->Instant{
    //     self.ac
    // }
    // pub fn get_end(&mut self)->Instant{
    //     self.ac
    // }
    pub fn run_time(&mut self) -> f64 {
        // let i = self.get_ac();
        // self.end = i.elapsed().as_secs();
        self.end
    }
}
