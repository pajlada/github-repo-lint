use ::reqwest::Url;

pub(super) struct PaginationData {
    pub next: Option<Url>,
}

pub(super) fn get_pagination_data(
    headers: &reqwest::header::HeaderMap<reqwest::header::HeaderValue>,
) -> anyhow::Result<PaginationData> {
    let mut res = PaginationData { next: None };

    if let Some(links_header) = headers.get("link") {
        let links_header_str = links_header.to_str()?;

        for link in links_header_str.split(',') {
            let segments: Vec<&str> = link.split(';').collect();

            if segments.len() != 2 {
                // invalid segment
                continue;
            }

            let url_part = segments[0].trim();
            let rel_part = segments[1].trim();

            if !url_part.starts_with('<') || !url_part.ends_with('>') {
                // invalid href
                continue;
            }

            let len = url_part.len();

            let url = match Url::parse(&url_part[1..len - 1]) {
                Ok(u) => u,
                Err(_) => continue,
            };

            match rel_part {
                "rel=\"next\"" => {
                    if res.next.is_some() {
                        return Err(anyhow::anyhow!("next link found twice"));
                    }

                    res.next = Some(url);
                }
                "rel=\"prev\"" | "rel=\"first\"" | "rel=\"last\"" => {
                    // Valid values, but we don't care about them
                }
                e => return Err(anyhow::anyhow!("unknown rel: {}", e)),
            }
        }
    }

    Ok(res)
}
