use std::sync::mpsc::Sender;

use eframe::egui;

use crate::Tip;


use super::sql::{check_pn, bind_query_none, bind_query};


pub fn get_check_pn_result(
    s: String,
    s1: String,
    tx: Sender<((Vec<Vec<String>>, usize), Tip)>,
    ctx: egui::Context,
) {
    tokio::spawn(async move {
        let res = check_pn(s, s1).await;
        let _ = tx.send(res);
        ctx.request_repaint();
    });
}

pub fn get_bind_query_none_result(tx: Sender<(Vec<Vec<String>>, usize)>, ctx: egui::Context) {
    tokio::spawn(async move {
        let res = bind_query_none().await;
        let _ = tx.send(res);
        ctx.request_repaint();
    });
}

pub fn get_bind_query_result(s: String, tx: Sender<(Vec<Vec<String>>, usize)>, ctx: egui::Context) {
    tokio::spawn(async move {
        let res = bind_query(s).await;
        let _ = tx.send(res);
        ctx.request_repaint();
    });
}
