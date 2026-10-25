const MAX:usize =200;
struct Vector{
    //definir atributos del objeto
    dimension:usize,
    elemento:[u64;MAX],
}
impl Vector{
    //Constructor
    fn new()->Vector{
        Vector{
            dimension:0,
            elemento:[0;MAX],
        }
    }

    fn dimension(&self)->usize{
        self.dimension
    }

    fn add_element(&mut self,e:u64){
        if self.dimension<MAX{
            self.elemento[self.dimension]=e;
            self.dimension+=1;
        }
    }
    
    fn mostrar(&self){
        for i in 0..self.dimension{
            println!("Elemento[{}]={}",i,self.elemento[i]);
        }
    }
    //Tarea
    //Procedimiento que acomode los impares a la izquierda y los pares a la derecha ordenados ascendentemente
    fn sort_par_imp(&self){
        let mut par=0;
        let mut impar=0;
        let mut c:usize=0;
        for i in 0..self.dimension{
            if self.elemento[i]==0{
                c+=1;}
            if self.elemento[i]%2==0{
                par=par*10+i;
            }else{
                impar=impar*10+i;
            }
            if c>0{
                for j in 1..c{
                    par*=10;
                }
            }

        }
    }

    fn pares_impares(&self)->bool{//funcion que devuelva si hay mas pares que impares
        let mut p=0;
        let mut m=0;
        for i in 0..self.dimension{
            if (self.elemento[i]%2)==0{
                p+=1;
            }else{
                m+=1;
            }
        }
    m>p
    }

    //funcion que devuelva si en el vector hay una escalera
    fn escalera(&self)->bool{
        let mut a=0;
        let mut i=0;
        let n=self.dimension-1;
        while i<=n && a<2{
            if self.elemento[i]+1==self.elemento[i+1]{
                a+=1;
            }
            else {
                a=0;
            }
            
            i+=1;
        }
        a>=2
    }
    //eliminar digitios repetidos consecutivos
    
    //cantidad escalera de 3
      fn cant_escalera(&self)->i32{
        let mut a=0;
        let mut i=0;
        let mut c=0;
        let n=self.dimension-1;
        while i<=n {
            if self.elemento[i]+1==self.elemento[i+1]{
                a+=1;
                if a==2{
                    c+=1;
                    a=0;
                    i+=1;
                }
            }
            else{
                a=0;
            }
            
            i+=1;
        }
        c
    }

    //IA
    fn ordenar_ascendente(&mut self) {
    for i in 0..self.dimension {
        for j in 0..(self.dimension - 1 - i) {
            if self.elemento[j] > self.elemento[j + 1] {
                self.elemento.swap(j, j + 1);
            }
        }
    }
}

fn ordenar_impares_pares(&mut self) {
    let mut v_impares = Vector { dimension: 0, elemento: [0; MAX] };
    let mut v_pares = Vector { dimension: 0, elemento: [0; MAX] };
    for i in 0..self.dimension {
        if self.elemento[i] % 2 != 0 {
            v_impares.elemento[v_impares.dimension] = self.elemento[i];
            v_impares.dimension += 1;
        } else {
            v_pares.elemento[v_pares.dimension] = self.elemento[i];
            v_pares.dimension += 1;
        }
    }
    v_impares.ordenar_ascendente();
    v_pares.ordenar_ascendente();
    let mut k = 0;
    for i in 0..v_impares.dimension {
        self.elemento[k] = v_impares.elemento[i];
        k += 1;
    }
    for i in 0..v_pares.dimension {
        self.elemento[k] = v_pares.elemento[i];
        k += 1;
    }
}

fn ordenar_adentro_afuera(&mut self) {
    let mut v_aux = Vector { dimension: self.dimension, elemento: self.elemento };
    v_aux.ordenar_ascendente();
    let centro = (self.dimension - 1) / 2;
    let mut izq = centro;
    let mut der = centro + 1;
    self.elemento[centro] = v_aux.elemento[0];
    let mut ir_derecha = true;
    for i in 1..v_aux.dimension {
        if ir_derecha {
            self.elemento[der] = v_aux.elemento[i];
            der += 1;
            ir_derecha = false;
        } else {
            izq -= 1;
            self.elemento[izq] = v_aux.elemento[i];
            ir_derecha = true;
        }
    }
}
//ordenamiento por seleccion
fn select_sort(&mut self) {
    for i in 0..self.dimension {
        let mut menor = i; 
        for j in (i + 1)..self.dimension {
            if self.elemento[j] < self.elemento[menor] {
                menor = j;
            }
        }
        self.elemento.swap(i, menor);
    }
}
//ordenamiento de burbuja
fn bubble_sort(&mut self) {
    for i in 0..self.dimension {
        for j in 0..self.dimension - 1 - i {
            if self.elemento[j] > self.elemento[j + 1] {
                self.elemento.swap(j, j + 1);
            }
        }
    }
}

//Insertion Sort
/*fn insertion_sort(&mut self){
    for i in 
}*/

//Merge Sort
fn merge_sort(&mut self){
    if self.dimension<=1{
        self.merge_div(0,self.dimension-1);
    }
}
fn merge_div(&mut self,izq:usize,der:usize){
    if izq<der{
    let mitad:usize=(izq+der)/2;
    self.merge_div(izq, mitad);
    self.merge_div(mitad+1, der);
    self.merge(izq,mitad,der);
}
}
fn merge(&mut self,izq:usize,mitad:usize,der:usize){
    let mut vecizq:[u64;50]=[0_u64;50];
    let mut vecder:[u64;50]=[0_u64;50];
    let mut tamanho_izq=mitad-izq;
    let mut tamanho_der=der-mitad+1;
    for i in 0..tamanho_izq{
        vecizq[i]=self.elemento[izq+i];
    }
}


}

fn main() {
    let mut v=Vector::new();
    v.add_element(1);
    v.add_element(0);
    v.add_element(3);
    v.add_element(7);
    v.add_element(11);
    v.add_element(6);
    v.add_element(18);
    v.add_element(13);
    v.add_element(8);
    v.add_element(2);

    println!("----------------------");
    v.mostrar();
    println!("----------------------");
    println!("EL tamaño del vector es:{}",v.dimension());
    println!("Hay mas impares que pares:{}",v.pares_impares());
    println!("Hay almenos una escalera en el vector:{}",v.escalera());
    println!("Hay un total de {} escalera(s)",v.cant_escalera());
    println!("----------------------");
    println!("-----Select Sort-----");
    let mut v1=v;
    v1.burbuja();
    v1.mostrar();
    /*let mut v1 = v;
    v1.ordenar_impares_pares();
    println!("Ejercicio 1 (Impares a la izq, Pares a la der):");
    v1.mostrar();

    println!("----------------------");
    let mut v2 = v1;
    v2.ordenar_adentro_afuera();
    println!("Ejercicio 2 (De adentro hacia afuera):");
    v2.mostrar();
    println!("----------------------");*/
    
}
