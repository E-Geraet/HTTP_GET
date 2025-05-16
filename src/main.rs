fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut response = ureq::get("https://httpbin.org/get")
        .call()?;

    // In ureq 3.0 gibt read_to_string() direkt einen String zurück
    let body = response.body_mut().read_to_string()?;

    println!("Response: {}", body);

    Ok(())
}
