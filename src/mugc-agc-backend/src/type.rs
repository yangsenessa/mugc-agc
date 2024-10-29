#[derive(Serialize, Deserialize)]
struct Comfyui_Payload {
    wk_info:&str,
    client_id:&str,
    promts_id:&str
}
#[derive(Serialize, Deserialize)]
struct Comfyui_url {
    url:&str,
    port:&str
}
