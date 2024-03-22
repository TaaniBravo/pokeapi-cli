/// Handles the retrieval of data from the pokeapi.
///
/// # Arguments
///
/// * `category` - The category endpoint to retrieve data from.
/// * `key` - The key can be either a name or id.
///
/// # Errors
///
/// Errors if the request fails or if the pokemon is not found.
///
/// # Examples
///
/// ```rust
/// let category = "pokemon".to_string();
/// let name = "pikachu".to_string();
/// let pokemon = get_data(category, name).await.unwrap();
///
/// let category2 = "item".to_string();
/// let id = "1".to_string();
/// let item = get_data(item, id).await.unwrap();
/// ```
pub async fn get_data(category: String, key: String) -> Result<String, reqwest::Error> {
    let url = format!("https://pokeapi.co/api/v2/{}/{}", category, key);
    let res = reqwest::get(&url).await?;
    let body = res.text().await?;
    Ok(body)
}
