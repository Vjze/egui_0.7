


use eframe::{
    egui::{self, Context, RichText, Window},
    emath::Align2,
};
use tokio::fs::File;

use crate::get_login_result;
pub struct ErrDlg {
    login_id: String,
    login_pass: String,
    unbind_sn: String,
    login_username: String,
    login_password: String,
    lebal: String,
    button: bool,
    res : bool,
}

impl ErrDlg {
    pub fn new() -> Self {
        let button = true;
        Self {
            login_id: Default::default(),
            login_pass: Default::default(),
            unbind_sn: Default::default(),
            login_username: Default::default(),
            login_password: Default::default(),
            lebal: Default::default(),
            button,
            res: Default::default(),
        }
    }

    pub fn sn_none_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("查询错误").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("查询框不能为空 ,必须输入对应信息才能查询!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }
        x
    }

    pub fn file_err_show(&mut self, ctx: &Context, e: String, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("读取文件失败").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label(e.to_string() + "点击创建进行创建");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("创建").clicked() {
                            x = false;
                            tokio::spawn(async move {
                                File::create("carton.txt").await.unwrap();
                            });
                        }
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn file_sn_err_show(&mut self, ctx: &Context, e: String, mut x: bool) -> bool {
        if x {
            // let available = ctx.available_rect();
            let mut open = true;

            Window::new(RichText::from("读取文件失败").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label(e.to_string() + "点击创建进行创建");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("创建").clicked() {
                            x = false;
                            tokio::spawn(async move {
                                File::create("SN.txt").await.unwrap();
                            });
                        }
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn file_none_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("读取文件失败").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("文件读取失败，请确认文件内容是否为空!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn file_sn_none_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("读取文件失败").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("文件读取失败，请确认文件内容是否为空!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn file_out_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("导出成功").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("箱号信息文件已经导出成功，请确认文件内容、格式是否正确!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn wx_none_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("尾纤SN错误").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("尾纤SN长度错误，请重新输入！");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn bd_none_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("本体SN错误").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("本体SN长度错误，请重新输入!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn joborder_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("绑定异常").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("尾纤SN与本体SN的工单不一致，请确认后再进行绑定!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn hl_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("绑定异常").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("尾纤SN与本体SN的料号不一致，请确认后再进行绑定!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn add_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("绑定异常").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("尾纤SN不存在，请确认后再进行绑定!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn bind_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("绑定异常").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("SN已绑定，请确认后再进行绑定!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn unbind_show(&mut self, ctx: &Context, mut x: bool) -> ((String, String, String), bool) {
        if x {
            let mut open = true;

            Window::new(RichText::from("确认解绑").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("请输入工号以及对应的解绑密码！");
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(4.0);
                        ui.label("工号：");
                        ui.text_edit_singleline(&mut self.login_id)
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(4.0);
                        ui.label("密码：");
                        ui.add(egui::TextEdit::singleline(&mut self.login_pass).password(true))
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(4.0);
                        ui.label("要解绑的条码：");
                        ui.text_edit_singleline(&mut self.unbind_sn)
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            //self.unbind_close();
                            x = false;
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.button("确认解绑").clicked() {
                            x = false;
                        };
                    })
                });
            if !open {
                x = false;
            }
        };

        let id = self.login_id.clone();
        let pass = self.login_pass.clone();
        let sn = self.unbind_sn.clone();

        return ((id, pass, sn), x);
    }

    pub fn login_ok_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("解绑成功").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("SN已解绑，可以进行重新绑定!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn login_err_qx_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;
            Window::new(RichText::from("解绑失败").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("该工号没有解绑权限，请重新尝试输入!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn login_err_pass_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;
            Window::new(RichText::from("解绑失败").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("密码错误，请重新尝试解绑!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn login_err_bcz_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;
            Window::new(RichText::from("解绑失败").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("工号不存在，请重新尝试解绑!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }

        x
    }

    pub fn job_err_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("工单号错误").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("输入的工单号长度不正确 ,请输入正确工单号再进行查询!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }
        x
    }

    pub fn much_none_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("生成错误").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("生成数量不能为空 ,必须输入数量才能够生成条码!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }
        x
    }

    pub fn much_0_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("生成数量错误").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("生成数量不能为0或者小于0 ,必须输入大于0的数量才能够生成条码!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }
        x
    }

    pub fn much_err_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("生成数量错误").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("生成数量不能大于剩余数量 ,请确认数量后重新输入!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }
        x
    }

    pub fn add_ok_show(&mut self, ctx: &Context, mut x: bool) -> bool {
        if x {
            let mut open = true;

            Window::new(RichText::from("条码生成成功").strong())
                .open(&mut open)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .current_pos([0.0, 24.0])
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label("条码已生成,需要自动打印请点击条码打印!");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("关闭").clicked() {
                            x = false;
                        }
                    });
                });
            if !open {
                x = false;
            }
        }
        x
    }

    pub fn login(&mut self, ctx: &egui::Context, mut x: bool) -> bool {
        
        if x {
            let open = true;

            let window = egui::containers::Window::new("Login")
                .collapsible(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]);
            window.show(ctx, |ui| {
                //egui::Grid::new("login box").show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Username");
                    let _ = egui::TextEdit::singleline(&mut self.login_username)
                        .hint_text("输入工号 ")
                        .show(ui);
                    ui.end_row();
                    ui.label("Password");
                    let pw = egui::TextEdit::singleline(&mut self.login_password)
                        .password(true)
                        .hint_text("输入密码")
                        .show(ui);
                    ui.end_row();
                    if pw.response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let id = self.login_username.clone();
                        let pa = self.login_password.clone();
                        let c = ctx.clone();
                        self.res = get_login_result(id, pa,c);

                        if self.res == true {
                            x = false;
                            self.login_username.clear();
                            self.login_password.clear();
                        } else {
                            x = true;
                            self.lebal = "工号不存在或者密码错误！请重新输入！".to_string();
                            self.login_username.clear();
                            self.login_password.clear();
                        }
                    }
                    ui.add(egui::Label::new(format!("{}", self.lebal)));
                    ui.end_row();
                });
                ui.vertical_centered(|ui| {
                    if ui
                        .add_enabled(self.button, egui::Button::new("Login"))
                        .clicked()
                    {
                        let id = self.login_username.clone();
                        let pa = self.login_password.clone();
                        let c = ctx.clone();
                        self.res = get_login_result(id, pa,c);
                        if self.res == true {
                            x = false;
                            self.login_username.clear();
                            self.login_password.clear();
                        } else {
                            x = true;
                            self.lebal = "工号不存在或者密码错误！请重新输入！".to_string();
                            self.login_username.clear();
                            self.login_password.clear();
                        }
                    }
                })
            });

            if !open {
                x = false;
            }
        }
        if self.login_username.is_empty() || self.login_password.is_empty() {
            self.button = false;
        } else {
            self.button = true;
        }
        
        x
    }
}
