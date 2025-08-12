use worker::*;

const API_KEY: &str = include_str!("key.txt");
const ORIGIN: &str = "https://open.api.nexon.com";
const CACHE_TTL: u32 = 300;

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let mut url = Url::parse(ORIGIN)?;
    url.set_path(req.url()?.path());
    url.set_query(req.url()?.query());

    let headers = Headers::from_iter([
        ("accept", "application/json"),
        ("x-nxopen-api-key", API_KEY),
    ]);

    let cache_key = Some(format!(
        "{path}?{query}",
        path = req.url()?.path(),
        query = req.url()?.query().unwrap_or_default()
    ));

    let request = Request::new_with_init(
        url.as_str(),
        &RequestInit {
            headers,
            cf: CfProperties {
                cache_key,
                cache_ttl: Some(CACHE_TTL),
                ..CfProperties::default()
            },
            method: Method::Get,
            ..RequestInit::default()
        },
    )
    .unwrap();

    let mut response = Fetch::Request(request).send().await?.cloned()?;
    let response_headers = response.headers_mut();
    response_headers.set("Access-Control-Allow-Origin", "*")?;
    response_headers.set("Cache-Control", &format!("max-age={CACHE_TTL}"))?;

    Ok(response)
}
