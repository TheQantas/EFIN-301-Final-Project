use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

use matchup::{get_matchups, Matchup};
use tree::game_tree_builder;

use std::error::Error;

mod season;
use season::{read_in_defenses, read_in_offenses, SeasonMap};

mod matchup;

mod tree;

struct MatchupEval {
    away_act: i16,
    home_act: i16,
    away_exp: f64,
    home_exp: f64,
    away_err: f64,
    home_err: f64
}

fn eval_matchup_error(index: usize,matchup: &Matchup,offenses: &SeasonMap,defenses: &SeasonMap) -> (MatchupEval,u128) {
    use std::time::Instant;

    let start = Instant::now();
    let tree = game_tree_builder(
        false,
        offenses.get(&matchup.get_away_id()).unwrap(),
        defenses.get(&matchup.get_away_id()).unwrap(),
        offenses.get(&matchup.get_home_id()).unwrap(),
        defenses.get(&matchup.get_home_id()).unwrap()
    );
    
    let end = Instant::now();
    let dura = (end-start).as_millis();

    
    let away_exp = tree.get_exp_away_score() * 2.0;
    let away_act = matchup.get_away_score();
    let home_exp = tree.get_exp_home_score() * 2.0;
    let home_act = matchup.get_home_score();
    let eval = MatchupEval {
        away_act,
        away_exp,
        home_act,
        home_exp,
        away_err: away_exp - (away_act as f64),
        home_err: home_exp - (home_act as f64)
    };

    if index % 100 == 0 {
        println!(
            "{}: ({:.3}){} {} @ {} {}({:.3}) [{}] in {}ms w/ {} nodes",
            index,
            eval.away_exp,
            eval.away_act,
            matchup.get_away_id().0,
            matchup.get_home_id().0,
            eval.home_act,
            eval.home_exp,
    
            matchup.get_season(),
            dura,
            tree.get_child_count()
        );
    }

    return (eval,dura);
}

fn mean(data: &[f64]) -> f64 {
    return data.iter().sum::<f64>() / data.len() as f64;
}

fn std_dev(data: &[f64]) -> f64 {
    let mean = mean(data);
    let squared_diffs: Vec<f64> = data.iter().map(|x| (x - mean).powi(2)).collect();
    let variance = squared_diffs.iter().sum::<f64>() / (data.len() as f64);
    return variance.sqrt();
}

fn sim_all(output_path: &str,predict: bool,matchups: &Vec<Matchup>,offenses: &SeasonMap,defenses: &SeasonMap) -> Result<(),Box<dyn Error>> {
    use std::time::Instant;

    fn stringify_distr(vec: &Vec<f64>) -> String {
        let mut s = String::from("[");
        for (index,val) in vec.iter().enumerate() {
            s += &format!("{:.4}",val);
            if index != vec.len() - 1 {
                s += ",";
            }
        }
        return s + &"]";
    }
    
    File::create(output_path)?.set_len(0)?;

    // let mut file = File::create("data/sim_static.csv")?;
    let mut file = OpenOptions::new().append(true).create(true).open(output_path)?;

    // let mut text = String::from("away,home,season,away_score,home_score,away_calc,home_calc,away_distr,home_distr,ou_max,ou_distr,margin_min,margin_max,margin_distr\n");
    file.write_all(
        "away,home,season,away_score,home_score,away_calc,home_calc,away_err,home_err,away_distr,home_distr,ou_max,ou_distr,margin_min,margin_max,margin_distr\n".as_bytes()
    )?;

    let start = Instant::now();
    
    for (index,matchup) in matchups.iter().enumerate() {
        if predict && matchup.get_season() == 2002 {
            continue;
        }
        // if matchup.get_season() <= 2020 {
        //     break;
        // }
        if index % 50 == 0 {
            println!("{}/{} elapsed {}s",index,matchups.len(),(Instant::now()-start).as_secs());
            // if index > 0 {
            //     break;
            // }
        }

        // let inner_start = Instant::now();
        let tree = game_tree_builder(
            false,
            offenses.get(&(if !predict { matchup.get_away_id() } else { matchup.get_away_id_prev() })).unwrap(),
            defenses.get(&(if !predict { matchup.get_away_id() } else { matchup.get_away_id_prev() })).unwrap(),
            offenses.get(&(if !predict { matchup.get_home_id() } else { matchup.get_home_id_prev() })).unwrap(),
            defenses.get(&(if !predict { matchup.get_home_id() } else { matchup.get_home_id_prev() })).unwrap(),
        );
        // println!("a: {} ms",(Instant::now()-inner_start).as_millis());

        // let inner_start = Instant::now();
        // let away_calc_old = tree.get_exp_away_score() * 2.0;
        // let home_calc_old = tree.get_exp_home_score() * 2.0;
        // println!("b: {} ms",(Instant::now()-inner_start).as_millis());
        
        // let inner_start = Instant::now();
        let distrs = tree.get_distributions();
        // println!("c: {} ms",(Instant::now()-inner_start).as_millis());
        
        // let inner_start = Instant::now();
        let mut away_calc = 0.0;
        for (index,prob) in distrs.away_scores.iter().enumerate() {
            away_calc += (index as f64) * prob;
        }
        let away_calc = away_calc * 2.0;
        // println!("{} {}",away_calc_old,away_calc*2.0);
        let mut home_calc = 0.0;
        for (index,prob) in distrs.home_scores.iter().enumerate() {
            home_calc += (index as f64) * prob;
        }
        let home_calc = home_calc * 2.0;
        // println!("{} {}",home_calc_old,home_calc*2.0);
        // println!("b2: {} ms",(Instant::now()-inner_start).as_millis());
        
        // let inner_start = Instant::now();
        let away_score = matchup.get_away_score();
        let home_score = matchup.get_home_score();
        let line = &format!(
            "{},{},{},{},{},{:.2},{:.2},{:.2},{:.2},\"{}\",\"{}\",{},\"{}\",{},{},\"{}\"\n",
            matchup.get_away_id().0,
            matchup.get_home_id().0,
            matchup.get_season(),

            away_score,
            home_score,
            away_calc,
            home_calc,
            away_calc - (away_score as f64),
            home_calc - (home_score as f64),
            
            stringify_distr(&distrs.away_scores),
            stringify_distr(&distrs.home_scores),

            distrs.over_under_max,
            stringify_distr(&distrs.over_under),
            
            distrs.margin_min,
            distrs.margin_max,
            stringify_distr(&distrs.margin)
        );
        // println!("d: {} ms",(Instant::now()-inner_start).as_millis());

        file.write_all(line.as_bytes())?;
        // break;
    }

    // file.write_all(text.as_bytes())?;

    Ok( () )
}

fn main() -> Result<(),Box<dyn Error>> {
    let offenses = read_in_offenses()?;
    let defenses = read_in_defenses()?;
    let matchups = get_matchups()?;

    // let mut a: Vec<f64> = vec![];
    // let mut b: Vec<f64> = vec![];
    // let mut c: Vec<f64> = vec![];
    // let mut d: Vec<f64> = vec![];

    // for ((team_id,year),season) in &defenses {
    //     if year == &2002 {
    //         continue;
    //     }
    //     let actual = season.get_distribution();
    //     let predicted = offenses.get(&(team_id.clone(),*year-1)).unwrap().predict_def_distr();

    //     a.push(actual[0]-predicted[0]);
    //     b.push(actual[1]-predicted[1]);
    //     c.push(actual[2]-predicted[2]);
    //     d.push(actual[3]-predicted[3]);
    // }

    // println!("mean {:.2} sd {:.2}",mean(&a),std_dev(&a));
    // println!("mean {:.2} sd {:.2}",mean(&b),std_dev(&b));
    // println!("mean {:.2} sd {:.2}",mean(&c),std_dev(&c));
    // println!("mean {:.2} sd {:.2}",mean(&d),std_dev(&d));

    sim_all("data/sim_dynamic.csv",true,&matchups,&offenses,&defenses)?;

    // const STEP_BY: usize = 1;
    // let mut actual_scores = Vec::<f64>::with_capacity(matchups.len()/STEP_BY*2);
    // let mut all_errors = Vec::<f64>::with_capacity(matchups.len()/STEP_BY);
    // let mut away_errors = Vec::<f64>::with_capacity(matchups.len()/STEP_BY);
    // let mut home_errors = Vec::<f64>::with_capacity(matchups.len()/STEP_BY);
    // let mut total_time_in_ms = 0u128;

    // let ids = (3..matchups.len()).step_by(50);
    // let ids = (0..matchups.len()).step_by(STEP_BY);

    // // let ids: Vec<usize> = vec![282,259,252,224,208,201,179,167,143,125,118,96,83,64,45,42,25]; //SF
    // // assert!(ids.len()==17);

    // let mut sf_total = 0.0;

    // for i in ids {
    //     let (eval,dura) = eval_matchup_error(i,&matchups[i], &offenses, &defenses);
    //     actual_scores.push(eval.away_act as f64);
    //     actual_scores.push(eval.home_act as f64);

    //     all_errors.push(eval.away_err);
    //     all_errors.push(eval.home_err);
    //     away_errors.push(eval.away_err);
    //     home_errors.push(eval.home_err);
    //     total_time_in_ms += dura;
        // if &matchups[i].get_away_id().0 == "SF" {
        //     sf_total += eval.away_exp;
        // } else {
        //     sf_total += eval.home_exp;
        // }
        
    // }

    // println!("all: mu {} sigma {}",mean(&all_errors),std_dev(&all_errors));
    // println!("away: mu {} sigma {}",mean(&away_errors),std_dev(&away_errors));
    // println!("home: mu {} sigma {}",mean(&home_errors),std_dev(&home_errors));
    // println!("actual: mu {} sigma {}",mean(&actual_scores),std_dev(&actual_scores));
    // println!("total time: {} ms",total_time_in_ms);

    // println!("sf {:.3}",sf_total);

    Ok(())
}
