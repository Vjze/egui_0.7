use catppuccin_egui::{FRAPPE, LATTE, MACCHIATO, MOCHA};
use chrono::Local;
use crate::AppPages;
use crate::CatppuccinTheme;
use crate::ErrDlg;
use crate::bind_ui;
use crate::configure_fonts;
use crate::APP_VERSION;
use crate::joborder;
use crate::query_ui;
use eframe::egui::{self, ComboBox, RichText};
pub struct App {
    page: AppPages,
    query_ui: query_ui::Query,
    bind_ui: bind_ui::Bind,
    joborder: joborder::JobOrder,
    login: ErrDlg,
    pub login_bool: bool,
    theme: CatppuccinTheme,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        let ctx = &cc.egui_ctx;
        let login = ErrDlg::new();
        let query_ui = query_ui::Query::new(ctx);
        let bind_ui = bind_ui::Bind::new(ctx);
        let joborder = joborder::JobOrder::new(&ctx);
        let login_bool = true;
        App {
            page: AppPages::default(),
            query_ui,
            bind_ui,
            joborder,
            login,
            login_bool,
            theme: CatppuccinTheme::Mocha,
        }
    }

    fn draw_topbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.vertical_centered(|ui| {
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
        catppuccin_egui::set_theme(
            ctx,
            match self.theme {
                CatppuccinTheme::Frappe => FRAPPE,
                CatppuccinTheme::Latte => LATTE,
                CatppuccinTheme::Macchiato => MACCHIATO,
                CatppuccinTheme::Mocha => MOCHA,
            },
        );
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
                    .default_width(120.0)
                    .width_range(80.0..=160.0)
                    .show_inside(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("功能类型");
                        });
                        self.draw_topbar(ui);
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                            ComboBox::from_label("")
                                .selected_text(format!("{:?}", self.theme))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.theme,
                                        CatppuccinTheme::Latte,
                                        "Latte",
                                    );
                                    ui.selectable_value(
                                        &mut self.theme,
                                        CatppuccinTheme::Frappe,
                                        "Frappe",
                                    );
                                    ui.selectable_value(
                                        &mut self.theme,
                                        CatppuccinTheme::Macchiato,
                                        "Macchiato",
                                    );
                                    ui.selectable_value(
                                        &mut self.theme,
                                        CatppuccinTheme::Mocha,
                                        "Mocha",
                                    );
                                });
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
