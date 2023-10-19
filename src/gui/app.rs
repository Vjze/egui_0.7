use crate::bind_ui;
use crate::configure_fonts;
use crate::joborder;
use crate::query_ui;
use crate::AppPages;
use crate::ErrDlg;
use crate::APP_VERSION;
use chrono::Local;
use eframe::egui::{self, RichText};
pub struct App {
    page: AppPages,
    query_ui: query_ui::Query,
    bind_ui: bind_ui::Bind,
    joborder: joborder::JobOrder,
    login: ErrDlg,
    pub login_bool: bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        let ctx = &cc.egui_ctx;
        let login = ErrDlg::new();
        let query_ui = query_ui::Query::new(ctx);
        let bind_ui = bind_ui::Bind::new(ctx);
        let joborder = joborder::JobOrder::new(&ctx);
        let login_bool = false;
        App {
            page: AppPages::default(),
            query_ui,
            bind_ui,
            joborder,
            login,
            login_bool,
        }
    }

    fn draw_topbar(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(150.);
                ui.selectable_value(&mut self.page, AppPages::Query, "  🔎 数据查询");
                // ui.separator();
                ui.selectable_value(&mut self.page, AppPages::Bind, "📌 条码绑定");
                // ui.separator();
                ui.selectable_value(&mut self.page, AppPages::Bar, "📋 条码生成");
                // ui.separator();
            });
        });
        ui.separator();
    }
    fn draw_release_footer(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("release-footer").show(ctx, |ui| {
            // ui.separator();
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // ui.add_space(10.);
                    ui.label(
                        RichText::new(Local::now().format("%Y/%m/%d %H:%M:%S").to_string())
                            .monospace(),
                    );
                    // ui.add_space(50.);
                });

                // });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    let i = self.query_ui.run_time();
                    ui.label(
                        RichText::new(format!("本次查询耗时：{} 秒", i.to_string())).monospace(),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.);
                    ui.label(RichText::new(format!("Version {}", APP_VERSION)));
                    ui.add_space(10.);
                });
            });
        });
        ctx.request_repaint();
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.);
        self.draw_release_footer(ctx);
        if self.login_bool == true {
            let x = self.login_bool.clone();
            let t = self.login.login(ctx, x);
            if t == false {
                self.login_bool = false;
            }
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::SidePanel::left("类型")
                    .resizable(false)
                    .default_width(180.0)
                    .width_range(80.0..=160.0)
                    .show_inside(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            let style = (*ui.ctx().style()).clone().visuals.dark_mode;
                            if style == true {
                                ui.image(egui::include_image!(
                                    r"../../resources/logo/logo_dark.png"
                                ));
                            } else {
                                ui.image(egui::include_image!(
                                    r"../../resources/logo/logo_light.png"
                                ));
                            }
                        });
                        self.draw_topbar(ui);
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                            egui::widgets::global_dark_light_mode_switch(ui);
                        })
                    });

                match self.page {
                    AppPages::Bind => self.bind_ui.show(ctx, ui, _frame),

                    AppPages::Query => self.query_ui.show(ctx, ui, _frame),

                    AppPages::Bar => self.joborder.show(&ctx, ui, _frame),
                }
            });
        }
    }
}
