use std::sync::mpsc::{Receiver, Sender};

use eframe::egui::{self, *};
use egui_extras::{Column, TableBuilder};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use crate::Tip;
use crate::Rec;
use crate::check_cz;
use crate::tip::ErrDlg;
use crate::get_bind_query_none_result;
use crate::get_bind_query_result;
use crate::get_check_pn_result;
use crate::unbind_0;
use crate::unbind_2;
use crate::unbind_3;

pub struct Bind {
    tx: Sender<((Vec<Vec<String>>, usize), Tip)>,
    rx: Receiver<((Vec<Vec<String>>, usize), Tip)>,
    tx_1: Sender<(Vec<Vec<String>>, usize)>,
    rx_1: Receiver<(Vec<Vec<String>>, usize)>,
    enabled: bool,
    body_sn: String,
    wx_sn: String,
    check: bool,
    teble_data: (Vec<Vec<String>>, usize),
    tip: ErrDlg,
    wx_none_show: bool,
    bt_none_show: bool,
    joborder_show: bool,
    hl_show: bool,
    add_show: bool,
    bind_show: bool,
    unbind_tip: bool,
    unbind_id_bcz_tip: bool,
    unbind_err_qx_tip: bool,
    unbind_err_pa_tip: bool,
    unbind_ok_tip: bool,
    tip1: Tip,
}

impl Bind {
    pub fn new(_ctx: &Context) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let (tx_1, rx_1) = std::sync::mpsc::channel();
        let tip = ErrDlg::new();
        let wx_none_show = false;
        let bt_none_show = false;
        let unbind_tip = false;
        let unbind_id_bcz_tip = false;
        let unbind_err_qx_tip = false;
        let unbind_err_pa_tip = false;
        let unbind_ok_tip = false;
        let joborder_show = false;
        let hl_show = false;
        let add_show = false;
        let bind_show = false;
        Bind {
            enabled: true,
            body_sn: Default::default(),
            wx_sn: Default::default(),
            check: true,
            teble_data: Default::default(),
            tip,
            unbind_tip,
            unbind_id_bcz_tip,
            unbind_err_qx_tip,
            unbind_err_pa_tip,
            wx_none_show,
            bt_none_show,
            unbind_ok_tip,
            joborder_show,
            hl_show,
            add_show,
            bind_show,
            tip1: Tip::Ok,
            tx,
            rx,
            tx_1,
            rx_1,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut Ui, frame: &mut eframe::Frame) {
        if let Ok(res) = self.rx.try_recv() {
            self.teble_data = res.0;
            self.tip1 = res.1;
        }
        if let Ok(res) = self.rx_1.try_recv() {
            self.teble_data = res;
        }
        let tx_1 = self.tx_1.clone();
        let ctx_1 = ctx.clone();
        ui.add_enabled_ui(self.enabled, |ui| {
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    // //
                    ui.checkbox(&mut self.check, "区分工单");
                    let wx_sn = egui::TextEdit::singleline(&mut self.wx_sn)
                        .hint_text("尾纤SN ")
                        .show(ui);
                    let body_sn = egui::TextEdit::singleline(&mut self.body_sn)
                        .hint_text("本体SN ")
                        .show(ui);
                    let body_sn_id = body_sn.response.id;
                    let wx_sn_id = wx_sn.response.id;
                    if wx_sn.response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        ui.memory_mut(|s| s.request_focus(body_sn_id));
                    }
                    if body_sn.response.lost_focus()
                        && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        let tip = self.insert_jz(&ctx, ui, frame);
                        match tip {
                            Tip::Wx => {
                                self.wx_none_show = true;
                            }
                            Tip::Bd => {
                                self.bt_none_show = true;
                            }
                            Tip::Gd => {
                                self.joborder_show = true;
                            }
                            Tip::Hl => {
                                self.hl_show = true;
                            }
                            Tip::Add => {
                                self.add_show = true;
                            }
                            Tip::Binded => {
                                self.bind_show = true;
                            }
                            Tip::Ok => {
                                ();
                            }
                        }

                        ui.memory_mut(|s| s.request_focus(wx_sn_id));
                        self.body_sn.clear();
                        self.wx_sn.clear();
                    }

                    if ui.button("ｘ").clicked() {
                        self.body_sn.clear();
                        self.wx_sn.clear();
                    }
                });
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    if ui.add(egui::Button::new("绑定")).clicked() {
                        let tip = self.insert_jz(&ctx, ui, frame);
                        match tip {
                            Tip::Wx => {
                                self.wx_none_show = true;
                            }
                            Tip::Bd => {
                                self.bt_none_show = true;
                            }
                            Tip::Gd => {
                                self.joborder_show = true;
                            }
                            Tip::Hl => {
                                self.hl_show = true;
                            }
                            Tip::Add => {
                                self.add_show = true;
                            }
                            Tip::Binded => {
                                self.bind_show = true;
                            }
                            Tip::Ok => {
                                ();
                            }
                        }

                        self.body_sn.clear();
                        self.wx_sn.clear();
                    }

                    if ui.add(egui::Button::new("查询")).clicked() {
                        let sn = self.body_sn.clone();
                        if self.body_sn.is_empty() | self.wx_sn.is_empty() {
                            get_bind_query_none_result(tx_1, ctx_1)
                        } else {
                            get_bind_query_result(sn, tx_1, ctx_1)
                        }
                    }
                    if ui.add(egui::Button::new("解绑")).clicked() {
                        self.unbind_tip = true;
                    }
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    let q = self.teble_data.1.clone();
                    ui.add(egui::Label::new(format!("今日绑定数量：{}", q)));
                });
            });
            ui.separator();
            let text_size = egui::TextStyle::Body.resolve(ui.style()).size + 10.0;
            let words: Vec<String> = vec![
                "尾纤SN".to_string(),
                "本体SN".to_string(),
                "操作员".to_string(),
                "绑定时间".to_string(),
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
                    // if let Some(recv) = self.teble_data.clone() {
                    //     match &(*recv.borrow()) {
                    //         Some(infos) => {
                    //             for info in &infos.0 {
                    let infos = self.teble_data.0.clone();
                    for info in infos {
                        body.row(text_size, |mut row| {
                            for data in info {
                                row.col(|ui| {
                                    ui.label(data);
                                });
                            }
                        })
                        //             })
                    }
                    //     }
                    //     None => (), //{self.rec_err(ctx)},
                    // };
                });

            if self.wx_none_show == true {
                let x = self.wx_none_show.clone();
                let sta = self.tip.wx_none_show(ui.ctx(), x);
                self.enabled = false;
                if sta == false {
                    self.enabled = true;
                    self.wx_none_show = false;
                }
            };
            if self.bt_none_show == true {
                let x = self.bt_none_show.clone();
                let sta = self.tip.bd_none_show(ui.ctx(), x);
                if sta == false {
                    self.bt_none_show = false;
                }
            };

            if self.joborder_show == true {
                let x = self.joborder_show.clone();
                let sta = self.tip.joborder_show(ui.ctx(), x);
                if sta == false {
                    self.joborder_show = false;
                }
            };
            if self.hl_show == true {
                let x = self.hl_show.clone();
                let sta = self.tip.hl_show(ui.ctx(), x);
                if sta == false {
                    self.hl_show = false;
                }
            };
            if self.add_show == true {
                let x = self.add_show.clone();
                let sta = self.tip.add_show(ui.ctx(), x);
                if sta == false {
                    self.add_show = false;
                }
            };
            if self.bind_show == true {
                let x = self.bind_show.clone();
                let sta = self.tip.bind_show(ui.ctx(), x);
                if sta == false {
                    self.bind_show = false;
                }
            };
            if self.unbind_tip == true {
                self.enabled = false;
                let x = self.unbind_tip.clone();
                let id = self.tip.unbind_show(ctx, x);
                if id.1 == false {
                    self.enabled = true;
                    self.unbind_tip = false;
                    let rt = Runtime::new().unwrap();
                    rt.block_on(async {
                        self.unbind(id.0 .0, id.0 .1, id.0 .2).await;
                    });
                }
            };
            if self.unbind_id_bcz_tip == true {
                let x = self.unbind_id_bcz_tip.clone();
                let sta = self.tip.login_err_bcz_show(ctx, x);
                if sta == false {
                    self.unbind_id_bcz_tip = false;
                }
            };

            if self.unbind_err_qx_tip == true {
                let x = self.unbind_err_qx_tip.clone();
                let sta = self.tip.login_err_qx_show(ctx, x);
                if sta == false {
                    let _ = self.unbind_err_qx_tip = false;
                }
            };

            if self.unbind_err_pa_tip == true {
                let x = self.unbind_err_pa_tip.clone();
                let sta = self.tip.login_err_pass_show(ctx, x);
                if sta == false {
                    let _ = self.unbind_err_pa_tip = false;
                }
            };
            if self.unbind_ok_tip == true {
                let x = self.unbind_ok_tip.clone();
                let sta = self.tip.login_ok_show(ctx, x);
                if sta == false {
                    let _ = self.unbind_ok_tip = false;
                }
            }
        });
    }

    pub fn insert_jz(
        &mut self,
        ctx: &egui::Context,
        _ui: &mut Ui,
        _frame: &mut eframe::Frame,
    ) -> Tip {
        if let Ok(res) = self.rx.try_recv() {
            self.teble_data = res.0;
            self.tip1 = res.1;
        }
        let tx = self.tx.clone();
        let ctx = ctx.clone();
        let j1 = self.body_sn.clone();
        let j2 = self.wx_sn.clone();
        let j_1 = self.body_sn.clone();
        let j_2 = self.wx_sn.clone();
        if self.wx_sn.len() != 14 {
            return Tip::Wx;
        }
        if self.body_sn.len() != 16 {
            return Tip::Bd;
        }
        let joborder1 = &j1[2..8];
        let joborder2 = &j2[2..8];
        if self.check == true {
            if joborder1 != joborder2 {
                return Tip::Gd;
            } else {
                get_check_pn_result(j_1, j_2, tx, ctx);
            }
        } else {
            get_check_pn_result(j_1, j_2, tx, ctx);
        };
        self.tip1
    }

    pub async fn unbind(&mut self, i: String, p: String, s: String) {
        let (tx, mut rx) = mpsc::channel(1);
        let i_1 = i.clone();
        let p_1 = p.clone();
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let r = check_cz(i_1, p_1).await;
            let _ = tx.send(r).await;
        });
        let i_1 = i.clone();
        let s_1 = s.clone();
        let rx = rx.recv().await.unwrap();
        self.show_tip(i_1, s_1, rx)
    }

    pub fn show_tip(&mut self, id: String, sn: String, r: Rec) {
        match r {
            Rec::Bcz => {
                self.unbind_id_bcz_tip = true;
            }
            Rec::QxErr => {
                self.unbind_err_qx_tip = true;
            }
            Rec::PassErr => {
                self.unbind_err_pa_tip = true;
            }
            Rec::UnbindOk => {
                let i_1 = id.clone();
                let s_1 = sn.clone();
                let i_2 = id.clone();
                let s_2 = sn.clone();
                let s_3 = sn.clone();
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    unbind_0(s_1, i_1).await;
                });
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    unbind_2(s_2, i_2).await;
                });
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    unbind_3(s_3).await;
                });
                self.unbind_ok_tip = true;
            }
        }
    }
}
