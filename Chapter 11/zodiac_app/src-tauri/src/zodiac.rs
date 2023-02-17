#[derive(Debug, Clone)]
pub struct Zodiac{
    name: String,
    description: String,
    language: String
}

impl Zodiac{
    pub fn new(name: &str, description: &str, language: &str) -> Self{
        Self{
            name: name.to_string(), 
            description: description.to_string(), 
            language: language.to_string()
        }
    }
    pub fn aries() -> Self{
        Self::new("Aries", "Loves to be #1!", "Python")
    }
    pub fn taurus() -> Self{
        Self::new("Taurus", "Loves to be relaxed!", "Swift")
    }
    pub fn gemini() -> Self{
        Self::new("Gemini", "Loves to be curious!", "Kotlin")
    }
    pub fn cancer() -> Self{
        Self::new("Cancer", "Highly Intuitive!", "Golang")
    }
    pub fn leo() -> Self{
        Self::new("Leo", "Loves to be in the spotlight!", "C++")
    }
    pub fn virgo() -> Self{
        Self::new("Virgo", "Loves to be a perfectionist!", "TeX")
    }
    pub fn libra() -> Self{
        Self::new("Libra", "Loves to be at equilibrium!", "JavaScript")
    }
    pub fn scorpius() -> Self{
        Self::new("Scorpius", "Loves to be mysterious!", "Assembly")
    }
    pub fn sagittarius() -> Self{
        Self::new("Sagittarius", "Always on the quest for knowledge!","WebAssembly")
    }
    pub fn capricorn() -> Self{
        Self::new("Capricorn","Loves to be patient!","C")
    }
    pub fn aquarius() -> Self{
        Self::new("Aquarius","Wants to make the world better!","Rust")
    }
    pub fn pisces() -> Self{
        Self::new("Pisces","Intuitive, sensitive and empathetic","Java")
    }
    pub fn to_string(&self) -> Vec<String> {
        let sign = format!("Results of zodiac sign: {}",&self.name);
        let why = format!("Why: {}", &self.description);
        let lang = format!("Your programming language: {}", &self.language);
        vec![sign, lang, why]
    }
}
// if zodiac sign doesn't exist 
impl Default for Zodiac{
    fn default() -> Self {
        Self::new("????", "What zodiac sign is that?", "Fortran???")
    }
}