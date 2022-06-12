mod req;
mod table;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = clap::Command::new("qbcli")
        .about("Social Work Library CLI")
        .name("qbcli")
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
                .value_name("MOBILE")
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
        .arg(
            clap::Arg::new("query-wb")
                .long("query-wb")
                .value_name("UID")
                .help("Weibo binding query interface")
                .required(false),
        )
        .arg(
            clap::Arg::new("reverse-query-wb")
                .long("reverse-query-wb")
                .value_name("MOBILE")
                .help("Weibo binding reverse lookup interface")
                .required(false),
        )
        .arg(
            clap::Arg::new("query-lol")
                .long("query-lol")
                .value_name("UID")
                .help("LOL binding query interface")
                .required(false),
        )
        .arg(
            clap::Arg::new("reverse-query-lol")
                .long("reverse-query-lol")
                .value_name("NAME")
                .help("LOL binding reverse lookup interface")
                .required(false),
        )
        .get_matches();

    let query = req::QBQuery::new().await?;

    if let Some(qq) = matches.value_of("query-qq") {
        let res = query.query_qq_for_qq(qq).await?;
        table::print_query_qq_table(res);
    }

    if let Some(mobile) = matches.value_of("reverse-query-qq") {
        let res = query.reverse_query_qq_for_mobile(mobile).await?;
        table::print_reverse_query_qq_table(res);
    }

    if let Some(qq) = matches.value_of("16e-query-qq") {
        let res = query.query_16e_qq_for_qq(qq).await?;
        table::print_reverse_query_qq_table(res);
    }

    if let Some(uid) = matches.value_of("query-wb") {
        let res = query.query_weibo_for_uid(uid).await?;
        table::print_query_weibo_table(res);
    }

    if let Some(mobile) = matches.value_of("reverse-query-wb") {
        let res = query.reverse_query_weibo_for_mobile(mobile).await?;
        table::print_reverse_query_weibo_table(res);
    }

    if let Some(uid) = matches.value_of("query-lol") {
        let res = query.query_lol_for_uid(uid).await?;
        table::print_query_lol_table(res);
    }

    if let Some(name) = matches.value_of("reverse-query-lol") {
        let res = query.reverse_query_lol_for_name(name).await?;
        table::print_reverse_query_lol_table(res);
    }

    Ok(())
}
