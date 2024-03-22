/// Handles the retrieval of pokemon data from the pokeapi.
///
/// # Arguments
///
/// * `name` - The name of the pokemon to retrieve.
///
/// # Errors
///
/// Errors if the request fails or if the pokemon is not found.
///
/// # Examples
///
/// ```rust
/// let pokemon = get_pokemon_by_name("pikachu".to_string()).await.unwrap();
/// ```
pub async fn get_data(category: String, name: String) -> Result<String, reqwest::Error> {
    let url = format!("https://pokeapi.co/api/v2/{}/{}", category, name);
    let res = reqwest::get(&url).await?;
    let body = res.text().await?;
    Ok(body)
}
