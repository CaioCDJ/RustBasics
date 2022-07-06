fn main() {
    println!("Hello, world!");
    //tabuada(10);
    lists();
    bas(3,-15,12);

    println!("\n{}",retorno() );
    vetores();
    
}

fn vetores(){
    // arrays que mudam de tamanho mutavel
// Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    // :?
    println!("Collected (0..10) into: {:?}", collected_iterator);
    
}

fn retorno() -> String{
    return "Ola".to_string();
}

fn bas(a:i64,b:i64,c:i64){
    
    let delta:f64 = ((b * b) -4 * a * c) as f64;
        
    if delta>0.0{
        // conversoes entre parenteses || as type        
        let x1:f64 = ((-b as f64 + delta.sqrt()) / ( 2.0 * a as f64) as f64) as f64;            
        let x2:f64 = ((-b as f64 - delta.sqrt()) / (2.0 * a as f64) as f64) as f64;
        
        println!("x1 = {}\nx2 = {}",x1,x2);
    } 
}

// o importante foi dominado
fn tabuada(x:i32){
    for i in 1..11{
        
        println!("{}*{}={}",i,x,i*x);
    }
}

fn lists(){
    // tuple || tupla
    // isso deveria ser ilegal
    let list:(i32,f32,&str) = (32,32.65,"Cachorro");
    
    println!("Tuble\n{}",list.1);
    
    // array tradicional
    let nomes = ["Oliver", "caio","Lucas"];
    
    // var:[type;lenght] || Peiculiar eu diria 
    let num:[i32;4] = [1,2,3,4];
    
    println!("{:?}",nomes);
    
}
