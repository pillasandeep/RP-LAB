fn main() {

    let n=10;
    let mut sum=0;
    for i in 2..n
    {
        let mut count=0;
        let l = (i as f64).sqrt() as i32;           
        for j in 1..=l
        {
            if i%j==0
            {
            count=count+1;    
            }
        }   
        if count  == 1 
        {
            sum=sum+i;
    }
    }
    println!("{}",sum);
}
