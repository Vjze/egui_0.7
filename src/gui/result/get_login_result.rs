
use eframe::egui;

use super::sql::login_user;



pub fn get_login_result(i:String,p:String, ctx: egui::Context) ->bool{
    let (tx,rx) = std::sync::mpsc::channel();
    tokio::spawn(async move {
        let res = login_user(i, p).await;
        let _ = tx.send(res);
        ctx.request_repaint();
    });
    let res = rx.recv().unwrap();
    res
}