use nauty_Traces_sys::*;
use std::os::raw::c_int;
use crate::representations::Graph::Graph;

pub fn canon(graph: &Graph) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let mut options = optionblk::default();
    //options.writeautoms = TRUE;
    options.getcanon = TRUE;
    let mut stats = statsblk::default();

    /*print!("\nenter n : ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;*/
    let n = graph.get_num_nodes();
    
    let m = SETWORDSNEEDED(n);

    unsafe {
        nauty_check(WORDSIZE as c_int, m as c_int, n as c_int, NAUTYVERSIONID as c_int);
    }
    //println!("Parametro m AQUI  -> {}", m);
    let mut lab = vec![0; n];
    let mut ptn = vec![0; n];
    let mut orbits = vec![0; n];

    let mut g = empty_graph(m, n);
    let mut canong = empty_graph(m, n);
    
    for v in 0..n {
        let adj = graph.get_adjacences(v);
        for i in adj{
            ADDONEEDGE(&mut g, v, i, m);
        }
    }
    //println!("Graph = {:?}\n", g);
    
    unsafe {
        densenauty(
            g.as_mut_ptr(),
            lab.as_mut_ptr(),
            ptn.as_mut_ptr(),
            orbits.as_mut_ptr(),
            &mut options,
            &mut stats,
            m as c_int,
            n as c_int,
            canong.as_mut_ptr()
        );
    }
    //println!("{:?}", lab);
    Ok(lab.iter().map(|&x| x as usize).collect())
}
