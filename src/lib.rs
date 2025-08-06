use worker::*;

const API_KEY: &str = include_str!("key.txt");
const ORIGIN: &str = "https://open.api.nexon.com";

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let mut url = Url::parse(ORIGIN)?;
    url.set_path(req.url()?.path());
    url.set_query(req.url()?.query());

    let mut request = Request::new(url.as_str(), Method::Get)?;
    let request_headers = request.headers_mut()?;
    request_headers.set("accept", "application/json")?;
    request_headers.set("x-nxopen-api-key", API_KEY)?;

    let mut response = Fetch::Request(request).send().await?.cloned()?;
    let response_headers = response.headers_mut();
    response_headers.set("Access-Control-Allow-Origin", "*")?;

    Ok(response)
}
