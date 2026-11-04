const NF: usize = 100;
const NC: usize = 100;

struct Matriz{
    nro_filas: usize,
    nro_col: usize,
    celda: [[u64;NC];NF],
}

impl Matriz{
    //Constructor
    fn new(nro_filas:usize, nro_col:usize) ->Matriz{
        Matriz{
            nro_filas,
            nro_col,
            celda: [[0;NC];NF],
        }
    }

    fn obt_nrofilas(&self) ->usize {
        self.nro_filas
    }

    fn obt_nrocol(&self) ->usize{
        self.nro_col
    }

    fn get_celda(&self, f:usize, c:usize) ->u64{
        self.celda[f][c]
    }

    fn set_celda(&mut self, f:usize, c:usize, valor:u64){
        self.celda[f][c] = valor;
    }

    //Funcion para sumar todos los elementos de una matriz.
    fn suma(&self) ->u64{
        let mut suma = 0;
        for i in 0..self.nro_filas{
            for j in 0..self.nro_col{
                suma = suma + self.celda[i][j];
            }
        }
        suma
    }
    //Funcion para calcular el promedio de los elementos de una matriz
    fn promedio(&self)->f64{
        let divi=(self.nro_col*self.nro_filas )as f64;
        self.suma() as f64 /divi
    }

    //Funcion que devuelva el elemento mayor de la matriz
    fn mayor(&self)->u64{
        let mut mayor:u64=self.celda[0][0];
        for i in 0..self.nro_filas{
            for j in 0..self.nro_col{
                if self.celda[i][j]>mayor{
                    mayor=self.celda[i][j];
                }
            }
        }
        mayor
    }

    //Funcion que sume los elementos de la diagonal principal
    fn diag_pri(&self)->u64{
        let mut suma=0;
        for i in 0..self.nro_col{
            suma+=self.celda[i][i];
        }
        suma
    }
    //Funcion que sume los elementos de la diagonal secundaria
    fn diag_sec(&self)->u64{
        let mut suma=0;
        for i in 0..self.nro_col{
            suma+=self.celda[self.nro_filas-1-i][i];
        }
        suma
    }
    //Funcion de sume lo elementos extremos de la matriz
    fn suma_extremo(&self)->u64{
        let mut suma=0;
        let mut x=0;
        let mut i=0;
        let mut j=0;
        while x<2 {
        for c in 0..self.nro_col{
            suma+=self.celda[i][c];
        }
        for f in 1..self.nro_filas-1{
            suma+=self.celda[f][j];
        }
        x+=1;
        i=self.nro_filas-1;
        j=self.nro_col-1;
    }
        suma
    }
    // Función que suma los elementos extremos (borde) de la matriz
fn suma_extremo2(&self) -> u64 {
    let mut suma = 0;
    for f in 0..self.nro_filas {
        for c in 0..self.nro_col {
            if f == 0 || f == self.nro_filas - 1 || c == 0 || c == self.nro_col - 1 {
                suma += self.celda[f][c];
            }
        }
    }
    suma
}
//Funcion que suma los elementos internos
fn suma_interno(&self) -> u64 {
    let mut suma = 0;
    for f in 1..self.nro_filas-1 {
        for c in 1..self.nro_col-1 {
                suma += self.celda[f][c];
        }
    }
    suma
}
    //Metodo para contar los nros pares que tiene la matriz.
    fn cant_pares(&self) ->u16{
        let mut pares = 0;
        for i in 0..self.nro_filas{
            for j in 0..self.nro_col{
                if self.celda[i][j]%2 == 0{
                    pares += 1;
                }
            }
        }
        pares
    }

    fn impares(&self) ->u16{
        let mut impares = 0;
        for i in 0..self.nro_filas{
            for j in 0..self.nro_col{
                if self.celda[i][j]%2 != 0{
                    impares += 1;
                }
            }
        }
        impares
    }

    fn mostrar(&self){
        println!();
        for i in 0..self.nro_filas{
            println!(" ");
            for _ in 0..self.nro_col{
                print!("┌─────────┐");
            }
            println!();
            print!("  ");
            for c in 0..self.nro_col {
                print!("│{:^9}│", self.celda[i][c]);
            }
            println!();
            print!("  ");
            for _ in 0..self.nro_col {
                print!("└─────────┘");
            }
            println!();
        }
        print!("  ");
        for c in 0..self.nro_col {
            print!("{:^11}", format!("c{}", c));
        }
        println!("\n");
    }
}




fn main() {
    println!("════════════════════════════════════════");
    println!("  Matrices - POO — Programación I      ");
    println!("════════════════════════════════════════");

    let mut m = Matriz::new(3, 3);

    m.celda[0][0] = 1;   m.celda[0][1] = 2;   m.celda[0][2] = 3;
    m.celda[1][0] = 14;   m.celda[1][1] = 5;   m.celda[1][2] = 6;
    m.celda[2][0] = 7;   m.celda[2][1] = 8;   m.celda[2][2] = 9;

    println!("Matriz original:");
    m.mostrar();
    println!("════════════════════════════════════════");
    println!("La suma de todos los valores es: {}", m.suma());
    println!("════════════════════════════════════════");
    println!("La cantidad de pares es: {}", m.cant_pares());
    println!("La cantidad de impares es: {}", m.impares());
    println!("El promedio de los elementos de la matriz es: {}",m.promedio());
    println!("El mayor de todos los elemento de la matriz es: {}",m.mayor());
    println!("La suma de los elementos de la diagonal principal es: {}",m.diag_pri());
    println!("La suma de los elementos de la diagonal secundaria es: {}",m.diag_sec());
    println!("La suma de los elementos exteriores es: {}",m.suma_extremo());
    println!("La suma de los elementos exteriores es: {}",m.suma_extremo2());
    println!("La suma de los elementos internos es: {}",m.suma_interno());
}