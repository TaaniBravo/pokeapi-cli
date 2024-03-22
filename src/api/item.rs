/// [TODO:description]
///
/// # Arguments
///
/// * `name` - [TODO:description]
///
/// # Errors
///
/// [TODO:describe error types and what triggers them]
///
/// # Examples
///
/// ```
/// [TODO:write some example code]
/// ```
pub async fn get_item_by_name(name: String) -> Result<String, reqwest::Error> {
    let url = format!("https://pokeapi.co/api/v2/item/{}", name);
    let res = reqwest::get(&url).await?;
    let body = res.text().await?;
    Ok(body)
}
