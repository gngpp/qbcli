use crate::req;
use prettytable::{color, Attr, Cell, Row};

pub(crate) fn print_query_qq_table(res: req::DataResult) {
    let status = res.code.to_string();
    let msg = res.msg.unwrap_or_default();

    let data = res.data.unwrap_or_default();
    let qq = data.qq.unwrap_or_default();
    let mobile = data.mobile.unwrap_or_default();
    let place = data.place.unwrap_or_default();
    let wb = data.wb.unwrap_or_default();
    let lol_data = data.lol.unwrap_or_default();

    let mut lol_table = prettytable::Table::new();
    lol_table.add_row(Row::new(vec![
        Cell::new("qq"),
        Cell::new("name"),
        Cell::new("area"),
    ]));
    lol_table.add_row(Row::new(vec![
        Cell::new(lol_data.qq.unwrap_or_default().as_str()),
        Cell::new(lol_data.name.unwrap_or_default().as_str()),
        Cell::new(lol_data.area.unwrap_or_default().as_str()),
    ]));

    let mut data_table = prettytable::Table::new();
    data_table.add_row(Row::new(vec![
        Cell::new("status")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("message")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("qq")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("mobile")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("place")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("weibo")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("lol")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));

    data_table.add_row(Row::new(vec![
        Cell::new(status.as_str()),
        Cell::new(msg.as_str()),
        Cell::new(qq.as_str()),
        Cell::new(mobile.as_str()),
        Cell::new(place.as_str()),
        Cell::new(wb.as_str()),
        Cell::new(lol_table.to_string().as_str()),
    ]));

    data_table.printstd();
}

pub(crate) fn print_reverse_query_qq_table(res: req::DataResult) {
    let status = res.code.to_string();
    let msg = res.msg.unwrap_or_default();

    let data = res.data.unwrap_or_default();
    let qq = data.qq.unwrap_or_default();
    let mobile = data.mobile.unwrap_or_default();

    let mut table = prettytable::Table::new();
    table.add_row(Row::new(vec![
        Cell::new("status")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("message")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("qq")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("mobile")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN))
    ]));

    table.add_row(Row::new(vec![
        Cell::new(status.as_str()),
        Cell::new(msg.as_str()),
        Cell::new(qq.as_str()),
        Cell::new(mobile.as_str()),
    ]));

    table.printstd();
}