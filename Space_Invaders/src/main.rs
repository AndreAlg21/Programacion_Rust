struct Nave{
    //Definimos los atributos de las naves
    vida:u32,
    pos_x:f32,
    pos_y:f32,
    enemy:bool,
}

struct Escudo{
    //definimos los atributos de los escudos
    vida:u32,
    pos_x:f32,
    pos_y:f32,
}

struct Disparo{
    //Definimos los atributos de los disparos
    danho:u32
    velocidad:f32,
    pos_x:f32,
    pos_y:f32,
}

struct Juego{
    //Definimos los atributos del juego
}

fn main() {
    println!("Hello, world!");
}
