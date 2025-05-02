

fn main()
{
    execute_count(|| println!("Hello world!"));
    execute_count(simple_count);
    execute_count(thread_count);
    execute_count(thread_scope_count);
    execute_count(channel_count);
}

fn execute_count<F : FnOnce()>(fun:F){
    let now = std::time::Instant::now();
    fun();
    println!("{:#?}",now.elapsed());
}

fn simple_count()
{
    let max = std::i32::MAX / 16; 
    let mut sum = 0;

    for _ in 0..max
    {
        sum += 1;
    }
    println!("sum: {sum}");
}


fn thread_count()
{
    let max = std::i32::MAX / 16; 
    let sum = std::sync::Arc::new(std::sync::Mutex::new(0));
    let mut handles = Vec::new();
    
    for i in 0..16
    {
        let start = i * max / 16;
        let end = (i +1) * max / 16;
        let sum = std::sync::Arc::clone(&sum);

        let handle = std::thread::spawn( move || {
            let mut local_sum = 0;

            for _ in start..end
            {
                local_sum += 1;
            } 
            let mut sum = sum.lock().unwrap(); 
            *sum += local_sum;
        });
        
        handles.push(handle);

    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    println!("sum: {}",sum.lock().unwrap());
}



fn thread_scope_count()
{
    let max = std::i32::MAX / 16; 
    let sum = std::sync::Mutex::new(0);
    
    std::thread::scope(|s|{
        for i in 0..16
        {
            let start = i * max / 16;
            let end = (i +1) * max / 16;
            let sum = &sum; 

            
            s.spawn( move || {
                let mut local_sum = 0;

                for _ in start..end
                {
                    local_sum += 1;
                } 
                let mut sum = sum.lock().unwrap(); 
                *sum += local_sum;
            });
        }
    });
    
    println!("sum: {}",sum.lock().unwrap());
}


fn channel_count()
{
    let max = std::i32::MAX / 16; 
    let mut sum = 0;

    let (tx , rx) = std::sync::mpsc::channel();

    
    
    for i in 0..16
    {
        let start = i * max / 16;
        let end = (i +1) * max / 16;
        let tx = tx.clone();

        
        std::thread::spawn( move || {
            let mut local_sum = 0;

            for _ in start..end
            {
                local_sum += 1;
            } 
            tx.send(local_sum).unwrap();
            
        });
    }
    
    drop(tx);

    while let Ok(local_sum) = rx.recv()
    {
        sum += local_sum;
    }
    
    println!("sum: {}",sum);
}