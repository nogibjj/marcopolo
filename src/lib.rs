/*a marco polo function
create a function that if you pass in marco it returns polo
if a name that isnt marco is passed in return "who dis"*/
pub fn marco_polo(name: &str) -> String {
    if name == "marco" {
        "polo".to_string()
    } else {
        "who dis".to_string()
    }
}
