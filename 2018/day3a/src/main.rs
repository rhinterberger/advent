use std::env;
use std::process;
use std::fs;
use std::str;

fn main()
{
    let args:Vec<String> = env::args().collect();

    if args.len() < 2
    {
        println!("You have to provide an inputfile");
        process::exit(1);       
    } 

    let filename = &args[1];
    let filecontents = fs::read_to_string(filename).unwrap();

    let (inches, r) = get_overlapping(filecontents);

    println!("{} inches are overlapping - {:?} is not overlapping", inches, r);
}

fn get_overlapping(claims: String) -> (i32, Rect)
{
    let mut claimdata:Vec<Rect> = Vec::new();
    let mut r:Rect = Rect {id:0, x:0,y:0,w:0,h:0};

    for claim in claims.lines()
    {
        claimdata.push(Rect::new(claim));
    }

    let (width, height) = Map::calculate_dimensions(&claimdata);
    let m = Map {width, height};
    let mut data = vec![0; (m.width * m.height) as usize ];

    // Initialize map of Claims
    for claim in &claimdata
    {
        m.fill_rect(&mut data,cla:im);
    }

    // Check for non-overlapping rectangles
    for claim in &claimdata
    {
       if m.check_rect(&mut data, claim)
       {
           r=*claim;
           break;
       }
    }

    let mut count = 0;
    for d in data
    {
        if d > 1 { count +=1; }
    }

    (count,r)
} 

#[derive(Debug,Clone,Copy)]
struct Rect {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32
}

impl Rect
{
    fn new(def: &str) -> Rect
    {
        let mut tokens = def.split_whitespace().map(|token| token.replace(":","").replace("#",""));

        let id_string = tokens.nth(0).unwrap();
        let pos_string = tokens.nth(1).unwrap();
        let size_string = tokens.next().unwrap();

        let xy:Vec<&str> = pos_string.split(",").collect();
        let wh:Vec<&str> = size_string.split("x").collect();

        let rectangle = Rect {
            id: id_string.parse::<i32>().unwrap(),
            x: xy[0].parse::<i32>().unwrap(),
            y: xy[1].parse::<i32>().unwrap(),
            w: wh[0].parse::<i32>().unwrap(),
            h: wh[1].parse::<i32>().unwrap()
        };

        rectangle
    }
}


#[derive(Clone, Copy)]
struct Map {
    width: i32,
    height: i32
}

impl Map
{
    fn fill_rect(&self, data: &mut Vec<i32>, r: &Rect)
    {

        for x in r.x.. r.x+r.w
        {
            for y in r.y .. r.y+r.h
            {
                let idx: usize = (self.width * y + x) as usize;
                data[idx] += 1;
            }
        }
    }

    fn check_rect(&self, data: &mut Vec<i32>, r: &Rect) -> bool
    {
        for x in r.x .. r.x+r.w+1
        {
            for y in r.y .. r.y+r.h+1
            {
                let idx: usize = (self.width * y + x) as usize;
                if data[idx] > 1
                {
                    return false;
                }
            }
        }

        true
    }


    fn calculate_dimensions(claimdata: &Vec<Rect>) -> (i32, i32)
    {
        let mut width=0;
        let mut height=0;

        for claim in claimdata
        {
            let w=claim.w + claim.x;
            if w > width { width=w; }
            
            let h=claim.h + claim.y;
            if h > height { height=h; }

        }

        (width,height)
    }
}
