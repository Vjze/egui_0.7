use std::fmt::Display;

use eframe::egui;
use std::{
    error::Error,
    fmt::{ Formatter, Result as FmtResult},
    io::Error as IoError,
};

pub mod gui;
pub use gui::*;


pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_ICON: &[u8] = include_bytes!(r"utils/Logo.ico");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Default, PartialEq, Clone, Debug, Copy)]
pub enum Enum {
    #[default]
    SN,
    BoxNo,
    CartonNo,
    WorkerId,
    MultipleSn,
    MultipleCarton,
}

#[derive(Debug, PartialEq, Default)]
enum AppPages {
    Bind,
    #[default]
    Query,
    Bar,
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Tip {
    Wx,
    Bd,
    Gd,
    Hl,
    Add,
    Binded,
    Ok,
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Rec {
    Bcz,
    QxErr,
    PassErr,
    UnbindOk,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CatppuccinTheme {
    Frappe,
    Latte,
    Macchiato,
    Mocha,
}

pub fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = eframe::egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("utils/ht.ttf")),
    );

    fonts
        .families
        .entry(eframe::egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    fonts
        .families
        .entry(eframe::egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();

    style
        .text_styles
        .get_mut(&egui::TextStyle::Body)
        .unwrap()
        .size = 15.;
    style
        .text_styles
        .get_mut(&egui::TextStyle::Heading)
        .unwrap()
        .size = 15.;
    style
        .text_styles
        .get_mut(&egui::TextStyle::Button)
        .unwrap()
        .size = 15.;
    style
        .text_styles
        .get_mut(&egui::TextStyle::Monospace)
        .unwrap()
        .size = 12.;
    style
        .text_styles
        .get_mut(&egui::TextStyle::Small)
        .unwrap()
        .size = 20.;
    ctx.set_style(style);
}

#[derive(Debug)]
enum CustomError {
    ReadError(IoError),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            CustomError::ReadError(ref e) => e.fmt(f),
        }
    }
}

impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            CustomError::ReadError(ref e) => Some(e),
        }
    }
}

//将IoError转为IdError::ReadError
impl From<IoError> for CustomError {
    fn from(error: IoError) -> CustomError {
        CustomError::ReadError(error)
    }
}


#[derive(Debug,Clone)]
pub struct DataInfo {
    pno:Option<String>,
    sn: Option<String>,
    pn: Option<String>,
    workorder: Option<String>,
    creator: Option<String>,
    createtime: Option<String>,
}

impl DataInfo {
    pub fn new() -> Self {
        Self {
            pno: None,
            sn: None,
            pn: None,
            workorder: None,
            creator: None,
            createtime: None,
        }
    }
    pub fn set_pno(&mut self, pno: Option<&str>) -> Self {
        if let Some(pno) = pno {
            self.pno = Some(pno.to_string());
        }

        self.to_owned()
    }
    pub fn pno(&self)->String{
        self.pno.clone().unwrap()
    }
    pub fn set_sn(&mut self, sn: Option<&str>) -> Self {
        if let Some(sn) = sn {
            self.sn = Some(sn.to_string());
        }

        self.to_owned()
    }
    pub fn sn(&self)->String{
        self.sn.clone().unwrap()
    }
 
    pub fn set_pn(&mut self, pn: Option<&str>) -> Self {
        if let Some(pn) = pn {
            self.pn = Some(pn.to_string());
        }

        self.to_owned()
    }
    pub fn pn(&self)->String{
        self.pn.clone().unwrap()
    }
  
    pub fn set_workorder(&mut self, workorder: Option<&str>) -> Self {
        if let Some(workorder) = workorder {
            self.workorder = Some(workorder.to_string());
        }

        self.to_owned()
    }
    pub fn workorder(&self)->String{
        self.workorder.clone().unwrap()
    }
    pub fn set_creator(&mut self, creator: Option<&str>) -> Self {
        if let Some(creator) = creator {
            self.creator = Some(creator.to_string());
        }

        self.to_owned()
    }
    pub fn creator(&self)->String{
        self.creator.clone().unwrap()
    }
  
    pub fn set_createtime(&mut self, createtime: Option<&str>) -> Self {
        if let Some(createtime) = createtime {
            self.createtime = Some(createtime.to_string());
        }

        self.to_owned()
    }
    pub fn createtime(&self)->String{
        self.createtime.clone().unwrap()
    }
}