use ::chrono::Local;
use rust_xlsxwriter::{Format, Workbook, XlsxError, FormatAlign, FormatBorder};
pub fn universal_file_out(v: &Vec<Vec<String>>, q: usize) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width_pixels(0, 150)?;
    worksheet.set_column_width_pixels(1, 80)?;
    worksheet.set_column_width_pixels(2, 60)?;
    worksheet.set_column_width_pixels(3, 60)?;
    worksheet.set_column_width_pixels(4, 60)?;
    worksheet.set_column_width_pixels(5, 60)?;
    worksheet.set_column_width_pixels(6, 60)?;
    worksheet.set_column_width_pixels(7, 60)?;
    worksheet.set_column_width_pixels(8, 150)?;
    worksheet.set_column_width_pixels(9, 150)?;
    let format_f2 = Format::new()
        .set_num_format("#0.00")
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_border(FormatBorder::Thin);
    let format_f3 = Format::new()
        .set_num_format("#0.000")
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_border(FormatBorder::Thin);
    let bt_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_font_size(18)
        .set_border(FormatBorder::Thin)
        .set_bold();
    let format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_font_size(10)
        .set_text_wrap()
        .set_border(FormatBorder::Thin)
        .set_bold();
    let str_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_text_wrap()
        .set_border(FormatBorder::Thin);
    let date = Local::now().format("%Y/%m/%d").to_string();
    let quantity = q.clone().to_string();
    let qq = quantity.parse::<i32>().unwrap();
    worksheet.merge_range(0, 0, 0, 9, "B-GDP1218S-49-XE-2发货数据", &bt_format)?;
    worksheet.write_string_with_format(1, 8, "Date", &str_format)?;
    worksheet.write_string_with_format(1, 9, &date, &str_format)?;
    worksheet.write_string_with_format(2, 8, "Quantity", &str_format)?;
    worksheet.write_number_with_format(2, 9, qq, &str_format)?;
    worksheet.write_string_with_format(3, 0, "SN", &str_format)?;
    worksheet.write_string_with_format(3, 1, "Condition Unit", &format)?;
    worksheet.write_string_with_format(3, 2, "Ith （mA）", &str_format)?;
    worksheet.write_string_with_format(3, 3, "SE （W/A）", &str_format)?;
    worksheet.write_string_with_format(3, 4, "Po （mW）", &str_format)?;
    worksheet.write_string_with_format(3, 5, "Vf    （V）", &str_format)?;
    worksheet.write_string_with_format(3, 6, "Im （uA）", &str_format)?;
    worksheet.write_string_with_format(3, 7, "Sen （dBm）", &str_format)?;
    worksheet.write_string_with_format(3, 8, "Box_NO", &str_format)?;
    worksheet.write_string_with_format(3, 9, "Carton_NO", &str_format)?;

    let mut string_m: Vec<Vec<&String>> = vec![];

    let mut f2_m: Vec<Vec<f64>> = vec![];

    let mut f3_m: Vec<Vec<f64>> = vec![];
    for (i, infos) in v.iter().enumerate() {
        let mut string: Vec<&String> = vec![];
        let mut f2: Vec<f64> = vec![];
        let mut f3: Vec<f64> = vec![];
        string.push(&infos.get(0).unwrap());
        string.push(&infos.get(1).unwrap());
        string.push(&infos.get(8).unwrap());
        string.push(&infos.get(9).unwrap());
        let s_c = string.clone();
        string_m.insert(i, s_c);
        f2.push((infos.get(2).unwrap().parse::<f64>()).unwrap());
        f2.push((infos.get(4).unwrap().parse::<f64>()).unwrap());
        f2.push((infos.get(5).unwrap().parse::<f64>()).unwrap());
        f2.push((infos.get(6).unwrap().parse::<f64>()).unwrap());
        f2.push((infos.get(7).unwrap().parse::<f64>()).unwrap());
        let s_f2 = f2.clone();
        f2_m.insert(i, s_f2);
        f3.push((infos.get(3).unwrap().parse::<f64>()).unwrap());
        let s_f3 = f3.clone();
        f3_m.insert(i, s_f3);
    }
    //    string写入
    for (x, e) in string_m.iter().enumerate() {
        let row: u32 = x.try_into().unwrap();
        worksheet.write_string_with_format(row + 4, 0, *e.get(0).unwrap(), &str_format)?;
        worksheet.write_string_with_format(row + 4, 1, *e.get(1).unwrap(), &str_format)?;
        worksheet.write_string_with_format(row + 4, 8, *e.get(2).unwrap(), &str_format)?;
        worksheet.write_string_with_format(row + 4, 9, *e.get(3).unwrap(), &str_format)?;
    }
    for (x_2, e_2) in f2_m.iter().enumerate() {
        let row: u32 = x_2.try_into().unwrap();
        worksheet.write_number_with_format(row + 4, 2, *e_2.get(0).unwrap(), &format_f2)?;
        worksheet.write_number_with_format(row + 4, 4, *e_2.get(1).unwrap(), &format_f2)?;
        worksheet.write_number_with_format(row + 4, 5, *e_2.get(2).unwrap(), &format_f2)?;
        worksheet.write_number_with_format(row + 4, 6, *e_2.get(3).unwrap(), &format_f2)?;
        worksheet.write_number_with_format(row + 4, 7, *e_2.get(4).unwrap(), &format_f2)?;
    }
    for (x_3, e_3) in f3_m.iter().enumerate() {
        let row: u32 = x_3.try_into().unwrap();
        worksheet.write_number_with_format(row + 4, 3, *e_3.get(0).unwrap(), &format_f3)?;
    }
    workbook.save("rust_perf_test.xlsx")?;

    Ok(())
}
