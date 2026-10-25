const N: usize = 100;

struct Cadena {
    longitud: usize,
    caracteres: [char; N],
}

impl Cadena {
    fn new(texto: &str) -> Cadena {
        let mut c = Cadena {
            longitud: 0,
            caracteres: ['\0'; N],
        };
        for ch in texto.chars() {
            c.add_char(ch);
        }
        c
    }

    fn obtener_longitud(&self) -> usize {
        self.longitud
    }

    fn add_char(&mut self, c: char) {
        if self.longitud < N {
            self.caracteres[self.longitud] = c;
            self.longitud += 1;
        }
    }

    fn char_pos(&self, pos: usize) -> char {
        if pos > 0 && pos <= self.longitud {
            self.caracteres[pos - 1]
        } else {
            '\0'
        }
    }

    fn cant_dig_user(&self, dig: char) -> u32 {
        let mut c = 0;
        let mut t = 0;
        while t < self.longitud {
            if dig == self.caracteres[t] {
                c += 1;
            }
            t += 1;
        }
        c
    }

    fn char_repeat(&self) -> char {
        if self.longitud == 0 {
            return '\0';
        }
        let mut m = 0;
        let mut r: char = self.caracteres[0];
        for i in 0..self.longitud {
            let mut n = 0;
            let c: char = self.caracteres[i];
            for j in 0..self.longitud {
                if c == self.caracteres[j] {
                    n += 1;
                }
            }
            if n > m {
                m = n;
                r = c;
            }
        }
        r
    }
// Método para insertar un carácter dada una posición (índice base 1)
    fn insert_char_pos(&mut self, c: char, pos: usize) {
        if self.longitud < N && pos > 0 && pos <= self.longitud + 1 {
            let idx = pos - 1;
            let mut i = self.longitud;
            while i > idx {
                self.caracteres[i] = self.caracteres[i - 1];
                i -= 1;
            }
            self.caracteres[idx] = c;
            self.longitud += 1;
        }
    }
}

fn main() {
    let mut cadena = Cadena::new("hola mundo");
    println!("Longitud: {}", cadena.obtener_longitud());
    println!("Caracter en pos 2: {}", cadena.char_pos(2));
    println!("Cantidad de 'o': {}", cadena.cant_dig_user('o'));
    println!("Caracter mas repetido: '{}'", cadena.char_repeat());
}