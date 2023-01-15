use crate::exchange::ExchangeRate;
use num_format::{Locale, ToFormattedString};
use prettytable::{Attr, Cell, Row, Table};

pub fn print_rate_table(exchange_rates: Vec<ExchangeRate>) -> () {
    let mut table = Table::new();

    /* Generate Table Labels */
    let labels = vec!["ID", "Token In", "Token Out", "Rate", "Exchange", "Network"];
    let mut label_cells: Vec<Cell> = Vec::new();

    for label in labels {
        label_cells.push(Cell::new(label).with_style(Attr::Bold));
    }

    table.add_row(Row::new(label_cells));

    /* Generate Table Body */
    for rate in exchange_rates {
        table.add_row(row![
            rate.query.id,
            rate.query.token_in.symbol,
            rate.query.token_out.symbol,
            rate.rate.as_u64().to_formatted_string(&Locale::en),
            rate.query.pool.exchange,
            rate.query.pool.network
        ]);
    }

    table.printstd();
}
