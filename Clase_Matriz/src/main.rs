use std::result;

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
       //Funcion que devuelva el elemento menor de la matriz
    fn menor(&self)->u64{
        let mut menor:u64=self.celda[0][0];
        for i in 0..self.nro_filas{
            for j in 0..self.nro_col{
                if self.celda[i][j]<menor{
                    menor=self.celda[i][j];
                }
            }
        }
        menor
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
//Funcion que sume los elementos de la triangular superior
fn tri_sup(&self)->u64{
    let mut suma=0;
    for f in 0..self.nro_filas{
        for c in f..self.nro_col{
            suma+=self.celda[f][c];
        }
    }
    suma
}
//Funcion que sume los elementos de la triangular inferior
fn tri_inf(&self)->u64{
    let mut suma=0;
    for f in 0..self.nro_filas{
        for c in 0..=f{
            suma+=self.celda[f][c];
        }
    }
    suma
}
//Procedimienton que ordena una matriz
/*fn ordenar(&mut self){
    for f in 0..self.nro_filas{
        for c in 0..self.nro_col{
            if self.celda[f][c]<self.celda[f][c-1]{

            }
        }
    }
}*/
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
    //Funcion que sume todos lo elementos de una fila ingresada por el usuario
    fn suma_fila(&self,x:usize)->u64{
        let mut suma=0;
        for c in 0..self.nro_col{
            suma+=self.celda[x][c];
        }
        suma
    }
    //Encontrar la fila con mayor suma
    fn fila_suma(&self)->usize{
        let mut indice=0;
        let mut suma=0;
        for f in 0..self.nro_filas{
            if self.suma_fila(f)>suma{
                suma=self.suma_fila(f);
                indice=f;
            }
        }
        indice
    }
    //Sumar la diagonal principal para matrices no cuadradas hasta el elemento menor de fila o columna
    fn diag_pri_rec(&self)->u64{
        let mut suma=0;
        let i:usize;
        if self.nro_col>self.nro_filas{
            i=self.nro_filas;
        }else{
            i=self.nro_col;
        }
        for j in 0..i{
            suma+=self.celda[j][j];
        }
        suma
    }
    //Contar los elementos del borde

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

//-----Practica Parcial-----
//Cargar la matriz con la serie de fibonacci (sobreescribiendo la matriz original)
fn cargar_fibonacci(&mut self){
    let mut a=0;
    let mut b=1;
    for f in 0..self.nro_filas{
        for c in 0..self.nro_col{
            self.celda[f][c]=a;
            let x:u64=a+b;
            a=b;
            b=x;
        }
    }
}
//Convertir de binario a decimal y devolver el mayor de los numeros, por filas
fn bin_decimal(mut x:u64)->u64{
let mut resultado=0;
let mut exp=0;
while x>0{
    resultado+=(x%10)*(2i32.pow(exp))as u64;
    exp+=1;
    x/=10;
}
resultado
}
fn mayor_decimal(&self)->u64{
    let mut mayor=0;
    for f in 0..self.nro_filas{
        let mut binario:u64=0;
        for c in 0..self.nro_col{
            binario=binario*10+self.celda[f][c];
        }
        if Self::bin_decimal(binario)>mayor{
            mayor=Self::bin_decimal(binario);
        }
    }
    mayor
}
//Serie de Fibonacci de forma caja repetida
fn fibonacci_box(&mut self){
    let mut x=0;
    let mut b=self.nro_col-1;
    let mut z=self.nro_filas;
    let mut p=1;
    let mut q=1;
    while x<(self.nro_col+1)/2{
    for i in x..z{
        self.celda[x][i]=q;
        self.celda[self.nro_filas-x-1][i]=q;
    }
        let mut r=p+q;
            p=q;q=r;
    for j in x+1..b{
        self.celda[j][x]=q;
        self.celda[j][self.nro_col-x-1]=q;
    }
    x+=1;z-=1;b-=1;
    r=p+q;p=q;q=r;
}
}
//Cargar la matriz con la serie de fibonacci serpiente inversa (sobreescribiendo la matriz original)
fn fibonacci_snake(&mut self){
    let mut a=0;
    let mut b=1;
    let mut q=false;
    for f in (0..self.nro_filas).rev(){
        if q{
        for c in 0..self.nro_col{
            self.celda[f][c]=a;
            let x:u64=a+b;
            a=b;
            b=x;
            q=false;
        }
        }else{
            for c in (0..self.nro_col).rev(){
            self.celda[f][c]=a;
            let x:u64=a+b;
            a=b;
            b=x;
            q=true;
        }
        }
    }
}

//Ordenar las filas de un vector ascendentemente sin usar arreglos
/*fn ord_filas(&mut self){
for f in 0..self.nro_filas{
    for c in 0..self.nro_col{
        for i in c..self.nro_col{
            let
        }
    }
}*/
fn rotar_90_grados(&mut self) {
        let n = self.nro_filas;
        for capa in 0..(n / 2) {
            let primero = capa;
            let ultimo = n - 1 - capa;
            for i in primero..ultimo {
                let offset = i - primero;
                let temp = self.celda[primero][i];
                self.celda[primero][i] = self.celda[ultimo - offset][primero];
                self.celda[ultimo - offset][primero] = self.celda[ultimo][ultimo - offset];
                self.celda[ultimo][ultimo - offset] = self.celda[i][ultimo];
                self.celda[i][ultimo] = temp;
            }
        }
    }

    fn ordenar_matriz(&mut self) {
        let total = self.nro_filas * self.nro_col;
        for i in 0..total {
            for j in (i + 1)..total {
                let f1 = i / self.nro_col;
                let c1 = i % self.nro_col;
                let f2 = j / self.nro_col;
                let c2 = j % self.nro_col;
                if self.celda[f1][c1] > self.celda[f2][c2] {
                    let temp = self.celda[f1][c1];
                    self.celda[f1][c1] = self.celda[f2][c2];
                    self.celda[f2][c2] = temp;
                }
            }
        }
    }
    fn ordenar_espiral(&mut self) {
        self.ordenar_matriz();
        let mut arriba = 0;
        let mut abajo = self.nro_filas;
        let mut izq = 0;
        let mut der = self.nro_col;
        let total = self.nro_filas * self.nro_col;
        let mut paso = 0;

        while arriba < abajo && izq < der && paso < total {
            for c in izq..der {
                let target_f = arriba;
                let target_c = c;
                let min_pos = self.buscar_minimo_desde(paso, arriba, abajo, izq, der, target_f, target_c);
                let temp = self.celda[target_f][target_c];
                self.celda[target_f][target_c] = self.celda[min_pos.0][min_pos.1];
                self.celda[min_pos.0][min_pos.1] = temp;
                paso += 1;
            }
            arriba += 1;

            for f in arriba..abajo {
                let target_f = f;
                let target_c = der - 1;
                let min_pos = self.buscar_minimo_desde(paso, arriba, abajo, izq, der, target_f, target_c);
                let temp = self.celda[target_f][target_c];
                self.celda[target_f][target_c] = self.celda[min_pos.0][min_pos.1];
                self.celda[min_pos.0][min_pos.1] = temp;
                paso += 1;
            }
            der -= 1;

            if arriba < abajo {
                for c in (izq..der).rev() {
                    let target_f = abajo - 1;
                    let target_c = c;
                    let min_pos = self.buscar_minimo_desde(paso, arriba, abajo, izq, der, target_f, target_c);
                    let temp = self.celda[target_f][target_c];
                    self.celda[target_f][target_c] = self.celda[min_pos.0][min_pos.1];
                    self.celda[min_pos.0][min_pos.1] = temp;
                    paso += 1;
                }
                abajo -= 1;
            }

            if izq < der {
                for f in (arriba..abajo).rev() {
                    let target_f = f;
                    let target_c = izq;
                    let min_pos = self.buscar_minimo_desde(paso, arriba, abajo, izq, der, target_f, target_c);
                    let temp = self.celda[target_f][target_c];
                    self.celda[target_f][target_c] = self.celda[min_pos.0][min_pos.1];
                    self.celda[min_pos.0][min_pos.1] = temp;
                    paso += 1;
                }
                izq += 1;
            }
        }
    }

    fn buscar_minimo_desde(&self, _paso: usize, arriba: usize, abajo: usize, izq: usize, der: usize, start_f: usize, start_c: usize) -> (usize, usize) {
        let mut min_f = start_f;
        let mut min_c = start_c;
        for f in arriba..abajo {
            for c in izq..der {
                if self.celda[f][c] < self.celda[min_f][min_c] {
                    min_f = f;
                    min_c = c;
                }
            }
        }
        (min_f, min_c)
    }

    fn ordenar_serpiente(&mut self) {
        self.ordenar_matriz();
        for f in 0..self.nro_filas {
            if f % 2 != 0 {
                let mut izq = 0;
                let mut der = self.nro_col - 1;
                while izq < der {
                    let temp = self.celda[f][izq];
                    self.celda[f][izq] = self.celda[f][der];
                    self.celda[f][der] = temp;
                    izq += 1;
                    der -= 1;
                }
            }
        }
    }
}
//Cargar una matriz ordenado diagonalmente en forma de S
/*fn diag_s(&mut self){
    let mut contador=2;
    let mut arriba=0;
    let mut abajo=0;
    let mut b=true;
    self.celda[0][0]=1;
    for i in 0..(self.nro_col+self.nro_filas)-1{
    if b{
        for i in i..10{

        }
        b=false;
    }else{
        for j in 0..self.nro_filas{

        }
    }
}
}*/






fn main() {
    println!("════════════════════════════════════════");
    println!("  Matrices - POO — Programación I      ");
    println!("════════════════════════════════════════");

    let mut m = Matriz::new(3, 3);

    m.celda[0][0] = 1;   m.celda[0][1] = 0;   m.celda[0][2] = 0;    
    m.celda[1][0] = 1;   m.celda[1][1] = 0;   m.celda[1][2] = 1;   
    m.celda[2][0] = 1;   m.celda[2][1] = 1;   m.celda[2][2] = 1;    

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
    println!("La suma de los elementos de la triangular superior es: {}",m.tri_sup());
    println!("La suma de los elementos de la triangular inferior es: {}",m.tri_inf());
    println!("El mayor de los digitos en decimal es: {}",m.mayor_decimal());
    println!("El decimal es: {}",Matriz::bin_decimal(101));
    println!("=======Fibonacci=======");
    m.cargar_fibonacci();
    m.mostrar();
    m.fibonacci_box();
    m.mostrar();
    m.fibonacci_snake();
    m.mostrar();
}