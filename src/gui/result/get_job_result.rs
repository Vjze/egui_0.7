use std::sync::mpsc::{Sender};

use eframe::egui;

use super::sql::{query_job, query_rule_id};






pub fn get_job_query_result(j:String, tx: Sender<(Vec<String>, String)>, ctx: egui::Context) {
    tokio::spawn(async move {

        let res = query_job(j).await;
        let _ = tx.send(res);
        ctx.request_repaint();
    });
}

pub fn get_query_rule_id(infos:Vec<String>,mcu:i32, tx: Sender<Vec<Vec<String>>>, ctx: egui::Context){
    tokio::spawn(async move {

        let res = query_rule_id(infos,mcu).await;
        let _ = tx.send(res);
        ctx.request_repaint();
    });
}