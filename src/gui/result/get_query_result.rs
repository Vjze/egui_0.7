use std::{sync::mpsc::Sender, time::Instant};

use eframe::egui;

use crate::{box_work_test, DataInfo};

use super::{sql::{box_none_work, carton_none_work, carton_work, multiple_carton_work, multiple_sn_work,
    sn_work, workerid_work,}};


pub fn get_sn_result(
    s: String,
    tx_3: Sender<f64>,
    tx: Sender<Vec<Vec<String>>>,
    ctx: egui::Context,
) {
    let i = Instant::now();
    tokio::spawn(async move {
        let res = sn_work(s).await;
        let _ = tx.send(res);
        let end = i.elapsed().as_secs_f64();
        let _ = tx_3.send(end);
    });
    ctx.request_repaint();
}

pub fn get_box_none_result(
    d1: String,
    d2: String,
    tx_3: Sender<f64>,
    tx: Sender<(Vec<Vec<String>>, usize)>,
    ctx: egui::Context,
) {
    let i = Instant::now();
    tokio::spawn(async move {
        let res = box_none_work(d1, d2).await;
        let _ = tx.send(res);
        let end = i.elapsed().as_secs_f64();
        let _ = tx_3.send(end);
    });
    ctx.request_repaint();
}

pub fn get_box_result(
    s: String,
    tx_3: Sender<f64>,
    tx: Sender<(Vec<DataInfo>, usize)>,
    ctx: egui::Context,
) {
    let i = Instant::now();
    tokio::spawn(async move {
        let res = box_work_test(s).await;
        // println!("res = {:?}",res);
        let _ = tx.send(res);
        let end = i.elapsed().as_secs_f64();
        let _ = tx_3.send(end);
        ctx.request_repaint();
    });
}

pub fn get_carton_none_result(
    d1: String,
    d2: String,
    tx_3: Sender<f64>,
    tx: Sender<(Vec<Vec<String>>, i32)>,
    ctx: egui::Context,
) {
    let i = Instant::now();
    tokio::spawn(async move {
        let res = carton_none_work(d1, d2).await;
        // println!("res = {:?}",res);
        let _ = tx.send(res);
        let end = i.elapsed().as_secs_f64();
        let _ = tx_3.send(end);
        ctx.request_repaint();
    });
}

pub fn get_carton_result(
    s: String,
    tx_3: Sender<f64>,
    tx: Sender<(Vec<Vec<String>>, usize)>,
    ctx: egui::Context,
) {
    let i = Instant::now();
    tokio::spawn(async move {
        let res = carton_work(s).await;
        // println!("res = {:?}",res);
        let _ = tx.send(res);
        let end = i.elapsed().as_secs_f64();
        let _ = tx_3.send(end);
        ctx.request_repaint();
    });
}

pub fn get_much_sn(
    v: Vec<String>,
    tx_3: Sender<f64>,
    tx: Sender<Vec<Vec<String>>>,
    ctx: egui::Context,
) {
    let i = Instant::now();
    tokio::spawn(async move {
        let res = multiple_sn_work(v).await;
        let _ = tx.send(res);
        let end = i.elapsed().as_secs_f64();
        let _ = tx_3.send(end);
        ctx.request_repaint();
    });
}

pub fn get_much_carton(
    v: Vec<String>,
    tx_3: Sender<f64>,
    tx: Sender<(Vec<Vec<String>>, usize)>,
    ctx: egui::Context,
) {
    let i = Instant::now();
    tokio::spawn(async move {
        let res = multiple_carton_work(v).await;
        let _ = tx.send(res);
        let end = i.elapsed().as_secs_f64();
        let _ = tx_3.send(end);
        ctx.request_repaint();
    });
}

pub fn get_worker(
    s: String,
    d1: String,
    d2: String,
    tx_3: Sender<f64>,
    tx: Sender<(Vec<Vec<String>>, usize)>,
    ctx: egui::Context,
) {
    let i = Instant::now();
    tokio::spawn(async move {
        let res = workerid_work(s, d1, d2).await;
        // println!("res = {:?}",res);
        let _ = tx.send(res);
        let end = i.elapsed().as_secs_f64();
        let _ = tx_3.send(end);
        ctx.request_repaint();
    });
}
