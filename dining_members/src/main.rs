use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

//　メンバー
struct Member {
    name:String,
    left: usize,
    right: usize,
}

struct Table{
    forks: Vec<Mutex<()>>,
}

fn about_app() {
    println!("This is eating process");
    println!("=====================");
}

impl Member{

    fn new(name: &str, left: usize, right: usize) -> Member{
        Member{
            name: name.to_string(),
            left: left,
            right:right,
        }
    }
    fn eat(&self, table: &Table){
       let _left = table.forks[self.left].lock().unwrap();
       thread::sleep(Duration::from_millis(150));
       let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name)
    }
}

fn main(){
    about_app();
    // main関数内でのTableの定義
    let table = Arc::new(Table{forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let members = vec![
        Member::new("John",0,1),
        Member::new("Mike",1,2),
        Member::new("Becky",2,3),
        Member::new("Kate",3,4),
        Member::new("Michal",0,4),
    ];
// IntoIterator で配列を順に辿れるようにする。
    let handles: Vec<_> = members.into_iter()
        .map(|m|{
            let table = table.clone();
            thread::spawn( move ||{
                m.eat(&table);
            })
        }).collect();

    for h in handles{
        h.join().unwrap();
    }
}
