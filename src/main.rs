extern crate core;

mod req;
mod table;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = clap::Command::new("qbcli")
        .about("Social Work Library CLI")
        .arg_required_else_help(true)
        .arg(
            clap::Arg::new("query-qq")
                .long("query-qq")
                .value_name("QQ")
                .help("Q Bind the query interface")
                .required(false),
        )
        .arg(
            clap::Arg::new("reverse-query-qq")
                .long("reverse-query-qq")
                .value_name("PHONE")
                .help("Q Bind the reverse lookup interface")
                .required(false),
        )
        .arg(
            clap::Arg::new("16e-query-qq")
                .long("16e-query-qq")
                .value_name("QQ")
                .help("Q Bind 16e query interface")
                .required(false),
        )
        .get_matches();

    let query = req::QBQuery::new().await?;

    if let Some(qq) = matches.value_of("query-qq") {
        let res = query.query_qq_for_qq(qq).await?;
        table::print_query_qq_table(res);
    }

    if let Some(phone) = matches.value_of("reverse-query-qq") {
        let res = query.reverse_query_qq_for_phone(phone).await?;
        table::print_reverse_query_qq_table(res);
    }

    if let Some(qq) = matches.value_of("16e-query-qq") {
        let res = query.query_16e_qq_for_qq(qq).await?;
        table::print_reverse_query_qq_table(res);
    }

    Ok(())
}
