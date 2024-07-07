use std::fmt::Display;
use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;
#[derive(Debug, Clone)]
struct ExamRecord {
    id: u64,
    name: String,
    scores: Vec<Option<f32>>,
    total_score: Option<f32>,
    avg_score: Option<f32>
}
impl ExamRecord {
    fn gen_records () -> Vec<ExamRecord> {
        // 生成一个随机id
        let mut id = 0u64;
        // 将获取的数据分割 
        NAMES_CONTENT.split("\n").filter(|s| !s.is_empty()).take(4).map(|s| {
            let mut rng = rand::thread_rng();
            let n = rng.gen_range(1..=4);
            let chinese_score = if id %3 == 0 {
                None
            } else {
                let score: f32 = rng.gen();
                let score = 40.0 + score * 60.0;
                Some(score)

            };
            let match_score = if id %2 == 0 {
                None
            } else {
                let score: f32 = rng.gen();
                let score = 40.0 + score * 60.0;
                Some(score)

            };
            let english_score = if id %4== 0 {
                None
            } else {
                let score: f32 = rng.gen();
                let score = 40.0 + score * 60.0;
                Some(score)

            };
            let scores = vec![chinese_score, match_score, english_score];
            let attend_exams: Vec<&Option<f32>> = scores.iter().filter(|s| s.is_some()).collect();
            let (total_score, avg_score) =  if attend_exams.is_empty() {
                (None, None)
            } else {
                let total = attend_exams.iter().map(|s| s.unwrap()).sum();
                (Some((total)), Some(total/ (attend_exams.len() as f32)))
            };
            id += 1;
            ExamRecord {
                id,
                name: s.to_string(),
                scores,
                total_score,
                avg_score
            }
        }).collect()
    }
}

impl Display for ExamRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "2024-{:03} {} {:>6} {:>6} {:>6} {:>7} {:>6}",
            self.id,
            &self.name,
            match &self.scores[0] {
                Some(score) => format!("{:.2}", score),
                None => "-".to_string()
            },
            match &self.scores[1] {
                Some(score) => format!("{:.2}", score),
                None => "-".to_string()
            },
            match &self.scores[2] {
                Some(score) => format!("{:.2}", score),
                None => "-".to_string()
            },
            match &self.total_score {
                Some(score) => format!("{:.2}", score),
                None => "-".to_string()
            },
            match &self.avg_score {
                Some(score) => format!("{:.2}", score),
                None => "-".to_string()
            }
        )
    } 
}
fn main() {
    let mut records: Vec<ExamRecord> = ExamRecord::gen_records();
    loop {
        print_main_menu();
        print!("\n 请选择 [1-5]: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                println!("{} {} {} {} {} {} {} {}", "#", "学号", "  姓名", "  语文", "  数学", " 英语", "  总分", "  平均分");
                for v in &records {
                    println!("{}", v);
                }
            },
            "2" => {
                sort_by_total_score(&mut records);
                println!("{} {} {} {} {} {} {} {}", "#", "学号", "  姓名", "  语文", "  数学", " 英语", "  总分", "  平均分");
                for v in &records {
                    println!("{}", v);
                }
            },
            "5" => break,
            _ => println!("unkown option")
        }
    }
}
const NAMES_CONTENT: &'static str = include_str!("../assets/names.txt");
fn sort_by_total_score (records: &mut Vec<ExamRecord> ) {
    records.sort_by(|a, b|{
        match (a.total_score, b.total_score) {
            (Some(sa), Some(sb)) => sb.total_cmp(&sa),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal
        }
        
    })
}
fn print_main_menu () {
    println!("\n-------- 程序查询系统 --------");
    println!("1. 打印成绩单");
    println!("2. 排序成绩单");
    println!("3. 查询成绩单");
    println!("4. 全班平均成绩");
    println!("5. 退出查询系统");
}


