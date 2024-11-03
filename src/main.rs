#[derive(Debug)]

enum Media {
    Book{title: String , autor : String},
    Movie{title:String , director: String},
    AudioBook{title: String},
    Serie{title: String},

}
//agregamos una impl a el enum Media donde &self es una referencia a cada uno de los elementos de Media 
impl Media {
    fn description (&self) -> String {
        //esta es una expresión de coincidencia de patrones usando "if let". Comprueba si "self" coincide con la variante "Media::Book" del enum.
        //si "self" coincide desestrutura la variante "Media::Book" vinculando los campos "title" y "autor" a variables
        //con el mismo nombre(variables locales del bloque if let).
        //Es importante notar que las variables locales title y autor solo existen dentro del ámbito del bloque if let.
        // Fuera de este bloque, no se puede acceder a estas variables.
        //Es una característica conveniente en Rust que permite crear variables locales con los mismos nombres que los campos de una estructura o enum 
        //cuando se desestructura.
.
        if let Media::Book { title, autor} = self{

            format!("Book: {} {}", title, autor)

        }else if let Media::Movie {title,director} = self {
            format!("Movie: {} {}", title, director)

        }else if let Media::AudioBook {title} = self {
            format!("AudioBook: {}", title)

        } else {
            String::from("Media discription")
        }
    }
}



 fn main() {
    //creamos un enum y asignamos un valor en AudioBook.
    let audiobook = Media::AudioBook { 
        title: String::from("An audiobook")
     };
     let book = Media::Book {
        title: String::from("Good Book"),
        autor: String::from("Good Autor"),
     };
     let movie = Media::Movie {
        title: String::from("Bad movie"),
        director: String::from("Bad director")
     };
     let serie = Media::Serie {
        title: String::from("Good serie")
     };

     println!("{}", audiobook.description());
     println!("{}", book.description());
     println!("{}", movie.description());
     //caso en que self no coincide.
     println!("{}",serie.description());


     

 }