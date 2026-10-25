

struct Numero {
    valor: u64
}

impl Numero {
    //Constructor
    fn new(valor: u64) -> Self {
        Numero { valor }
    }

    fn es_par(&self) -> bool {
        self.valor % 2 == 0
    }

    //fn es_par(x: u64) -: bool 
    //     x % 2 == 0;

    //Multiplicar el valor (self) del objeto por el número dado.
    //fn multiplicar(&self, num: u64) -....
    //self.valor * num;

//1. Es múltiplo de "n"?
fn esmulti (&self,n:u64)->bool{
    self.valor%n==0
}
//2. Potencia propia (self.valor, elevado a self.valor)
/*fn potencia(&self)->u64{
    self.valor.pow(self.valor as u32)
}*/
//3. Devolver la cantidad de dígitos impares
fn digimp (&self)->u64{
    let mut c:u64=0;
    let mut x:u64=self.valor;
  while x>0 {           
        let dig: u64 = x % 10;
        if dig % 2 != 0 {  
            c += 1;
        }
        x /=10
    }
    c
}
//1. Invertir los dígitos del número
fn inverso(mut n:u64)->u64{
let mut x:u64=n;
let mut y:u64=0;
while x>0{
    let dig=x%10;
    y=(y*10)+dig;
    x/=10;
}
y
}
//2. Es capicua?
//fn capicua(&self)->bool{
  //  self.valor==self.inverso()
//}
//3. Raiz Digital (12345 - 1+ 2 + 3 + 4 + 5 = 15 - 1 + 5 = 6. Razi digital de 12345 = 6)
fn raiz_dig(&self)->u64{
    let residuo = self.valor % 9;
        if residuo == 0 {
            9
        } else {
            residuo
        }
}
//Metodo para insertar un digito x en la posicion n
//Eje: 472 || x=8; n=2
//Rust empieza en la posicion 0 '123';pos 0=1 and pos 1=2
fn insertar_dig (&self,pos:u64,dig:u64)->u64{
    let mut resultado:u64=self.valor;
    let mut c:u64=0;
    let mut x:u64=self.valor;
    while x>0 {
        c+=1;
        x/=10;
    }
    if c>=pos && pos!=0{
        let exp = (c - pos+1) as u32;
        let factor = 10_u64.pow(exp);
        let izq = self.valor / factor;
        let der = self.valor % factor; 
        resultado = ((izq * 10) + dig) * factor + der;
    }
resultado
}
fn adicionar(&self,num:u64)->u64{
let mut mult:u64=1;
let mut ad:u64=num;

while ad>0{
    mult*=10;
    ad/=10;
}
self.valor*mult+num
}
//1.- Método que elimina un dígito, dado el dígito (no la posición).
fn eliminar_dig(&self, dig: u64) -> u64 {
    let mut num = 0;
    let mut mult = 1;
    let mut x = self.valor; 
    while x > 0 {
        let d = x % 10;
        if d != dig {
            num += d * mult;
            mult *= 10;
        }
        x /= 10;
    }
    num
}
//2.- Método que convierte (devuelve) un número decimal, a número binario.
fn decimal_a_binario(&self) -> u64 {
    let mut num=self.valor;
    if num == 0 {
        return 0;
    }
    let mut binario: u64 = 0;
    let mut mult: u64 = 1;
    while num > 0 {
        let digito = num % 2;
        binario = binario + (digito * mult);
        mult = mult * 10;
        num = num / 2;
    }

    binario
}
//3.- Método que convierte (devuelve) un número decimal, a número hexadecimal
fn decimal_a_hexadecimal(&self) -> String {
    let mut num:u64=self.valor;
    if num == 0 {
        return "0".to_string();
    }
    let mut hex = String::new();
    while num > 0 {
        let residuo = num % 16;
        if residuo < 10 {
            hex = format!("{}{}", residuo, hex);
        } else {
            let letra = match residuo {
                10 => 'A',
                11 => 'B',
                12 => 'C',
                13 => 'D',
                14 => 'E',
                15 => 'F',
                _ => '0',
            };
            hex = format!("{}{}", letra, hex);
        }
        num = num / 16;
    }
    hex
}
fn ordenar_ascendente(&self) -> u64{
    let mut  r:u64=0;
    for i in 0..10{
        let mut t=self.valor;
        while t>0{
            let dig=t%10;
            if dig==i{
                r=r*10+dig;
            }
            t/=10;
        }
    }
    r
}
//ordenar un numero impares a la izquierda y pares a la derecha, ordenados de manera ascendente
fn parcial1(&self)->u64{
    let mut x=self.ordenar_ascendente();
    x=Self::inverso(x);
    let mut par=0;
    let mut impar=0;
    let mut exp:u32=0;
    while x>0{
        let dig=x%10;
        if dig%2==0{
            par=par*10+(dig);
            exp+=1;
        }else{
            impar=impar*10+(dig);
        }
        x/=10;
    }
    impar*10_u64.pow(exp)+par
}
// MOD 123%10 ==>3
// DIV 123/10 ==>12

//Metodo para ordenar los digitos de un numero desde el centro Ej: 37210 ==>31027
/*fn parcial(&self)->u64{
    let mut x=self.valor;
    let mut a=x%10;
    let mut r=0;
    while x>0{
        x/=10;
        let d=x%10;
        if a>d{
            a=d;
        }
    }
    r=Self::eliminar_dig(&self,a);
    let mut resultado=a;
    let mut y=self.valor;

    while r>0{
        let dig=y%10;
        
    }
    a


}*/
fn tamanho(&self)->u32{
    let mut x=self.valor;
    let mut i=0;
    while x>0{
        x/=10;
        i+=1;
    }
    i
}
//Metodo para ordenar los digitos de un numero desde el centro Ej: 37210 ==>31027
/*fn parcial4(&self)->u64{
    let mut x=self.ordenar_ascendente();
    let mut b:bool=true;
    x=Self::inverso(x);
    let mut r=x%10;
    let mut p=1;
    x/=10;
    while x>0{
        let dig=x%10;
        if b==true{
            r=dig*p+r;
            b=false;
        }else{
            r=r*10+dig;
            b=true;
        }
        x/=10;
        p*=10;
    }
    r
}*/
fn parcial(&self) -> u64 {
    let mut x = self.ordenar_ascendente();
    x = Self::inverso(x);
    let mut r = x % 10;
    let mut p: u64 = 10;
    let mut b: bool = true;
    x /= 10;
    while x > 0 {
        let dig=x%10;
        if b==true{
            r=dig*p+r;
            b=false;
        }else {
            r = r * 10 + dig;
            b = true;
        }
        x /= 10;
        p *= 10;
    }
    r
}
//reto Fizz-Buzz
fn fizzbuzz(&self)->(){
for i in 1..=self.valor{
    if (i%15)==0{
        println!("Fizz-Buzz");
    }else if (i%3)==0{
        println!("Fizz")
    }else if(i%5)==0{
        println!("Buzz")
    }else{
        println!("{}",i)
    }
}
}
//Reto Numero de Armstrong
fn numero_armstrong(&self)->bool{
    let mut b:bool=false;
    let mut x=self.valor;
    let mut y=0;
    let exp:u32=self.tamanho();
    while x>0{
        let dig=x%10;
        y=dig.pow(exp)+y;
        x/=10;        
    }
    if self.valor==y{
        b=true;
    }
    b
}
//Reto Numero de Collatz
fn numero_collatz(&self)->u64{
    let mut pasos:u64=0;
    let mut x=self.valor;
    while x>1{
        if (x%2)==0{
            x/=2;
        }else{
            x=3*x+1;
        }
        pasos+=1;
    }
    pasos
}

//
}

fn main() {
    //funcion main
    println!("=============================");
    println!("Struct Numero");
    println!("=============================");
    //Crear instancia del objeto Numero: n
    let n = Numero::new(6);

    //println!("El valor de la instancia n es: {}", n.valor);

    //println!("El valor n es par?: {}", n.es_par());

    //println!("El valor n es par?: {}", n.multiplicar(3));
    //println!("La potencia propia de {} es:{}",n.valor,n.potencia());
    //println!("La cantidad de digitos impares de {} es:{}",n.valor,n.digimp());
    //println!("¿Es multiplo de n? = {}",n.esmulti(3));
    //println!("El resultado es:{}",n.inverso());
    //println!("El resultado es:{}",n.capicua());
    //println!("El resultado es:{}",n.raiz_dig());
    //println!("El resultado es:{}",n.insertar_dig(5, 5));
    //println!("El numero final es:{}",n.adicionar(890));
    //println!("El numero final es:{}",n.eliminar_dig(4));
    //println!("El numero en binario es:{}",n.decimal_a_binario());
    //println!("El numero en hexadecimal es:{}",n.decimal_a_hexadecimal());
    println!("El resultado es:{}",n.parcial());
    //n.fizzbuzz();
    println!("Es numero de Armstrong:{}",n.numero_armstrong());
    println!("La cantidad de pasos del numero de Collatz es:{}",n.numero_collatz());
}