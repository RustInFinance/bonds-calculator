// TODO: implement total return 

use chrono::{Local, NaiveDate, ParseError, DateTime};

fn calculate_yield(pv: f64, payments : &Vec<f64>, num_payments_per_year : u32) -> Result<f64,&'static str>{

    let rounded_pv = round2(pv);
    // min and max 
    let mut miny : f64 = 0.0;
    let mut maxy : f64 = 20.0;
    let mut candidate_yield = (maxy+miny)/2.0;  // Starting point

    // 
    let mut run = true; 
    while run == true  {
        let total_gain = payments.iter().enumerate().fold(0.0, |mut gain, (i,v)| {
            gain += v/(1.0f64 + candidate_yield/100.0).powf(i as f64  +1.0);
            gain
        }); 
        
        let rounded_total_gain = round2(total_gain);
        log::info!("candidate yield: {candidate_yield}, present value: {rounded_total_gain}");
        if rounded_total_gain == rounded_pv {
          run = false;
        } else if rounded_total_gain > rounded_pv {
            miny = candidate_yield;
            candidate_yield = (candidate_yield + maxy)/2.0;
            if round2(miny) == round2(candidate_yield) {
                run = false
            }
        }  else if rounded_total_gain < pv {
            maxy = candidate_yield;
            candidate_yield = (candidate_yield + miny)/2.0;
            if round2(maxy) == round2(candidate_yield) {
                run = false
            }
        }
    }
    
    let effective_annual_yield = 100.0*((1.0 + candidate_yield/100.0).powf(num_payments_per_year as f64) - 1.0);
    Ok(round2(effective_annual_yield))
}

fn calculate_interest_on_interest(annual_annuity : f64, annual_investement_rate : f64, investements_per_year : u32, total_num_investements : u32) -> f64 {
    
    let interest_on_interest = if annual_investement_rate == 0.0 {
        0.0
    } else {

    let periodic_annuity = annual_annuity/investements_per_year as f64;
    let periodic_investment_rate = annual_investement_rate/100.0/investements_per_year as f64;
    let total_capital = periodic_annuity*(((1.0 + periodic_investment_rate).powf(total_num_investements as f64) - 1.0)/periodic_investment_rate);

    total_capital - periodic_annuity*total_num_investements as f64
    };
    log::info!("Interest on interest: {interest_on_interest}");
    interest_on_interest
}


fn calculate_total_return(pv: f64, maturity_val: f64, coupon_interest : f64, reinvestment_rate : Option<f64>,payments_per_year : u32, today: &str, next_payment_date : &str, maturity_date : &str) -> Result<(f64,f64,f64),&'static str>{
    let today_date = NaiveDate::parse_from_str(today, "%Y-%m-%d").map_err(|_| "Error converting todays date")?;
    let payment_date = NaiveDate::parse_from_str(next_payment_date, "%Y-%m-%d").map_err(|_| "Error converting next payment date")?;
    let maturity_date = NaiveDate::parse_from_str(maturity_date, "%Y-%m-%d").map_err(|_| "Error converting maturity date")?;
    let days_to_mature = maturity_date.signed_duration_since(today_date).num_days() as u32;
    let num_days_in_period = 366 / payments_per_year;
    let periods_to_mature = (days_to_mature as f64/num_days_in_period as f64).ceil() as u32;
    log::info!("days_to_mature: {days_to_mature}, num_days_in_period: {num_days_in_period}, periods_to_mature: {periods_to_mature}");
    log::info!("periods_to_mature: {periods_to_mature}");

    let mut payments : Vec<f64> = vec![];

    let days_to_next_payment = payment_date.signed_duration_since(today_date).num_days();

    let remaining_period_ratio = days_to_next_payment as f64/num_days_in_period as f64;
    log::info!("Days to next payment: {days_to_next_payment} remaining_period_ratio: {remaining_period_ratio}");

    let payment_per_period = maturity_val*coupon_interest/payments_per_year as f64/100.0;

    for _i in 0..periods_to_mature {
         payments.push(payment_per_period);
    }
    payments.push(maturity_val);
    let mut total_return = payments.iter().sum();
        total_return += match reinvestment_rate {
            Some(reinvestement_rate) => 
                calculate_interest_on_interest(payment_per_period*payments_per_year as f64, reinvestement_rate, payments_per_year, payments.len() as u32),
            None => 0.0,
        };
    Ok((pv,total_return - pv,total_return))
}




fn calculate_yield_to_maturity(
pv: f64, maturity_val: f64, coupon_interest_rate : f64, payments_per_year : u32, reinvestement_rate : Option<f64>,today: &str, next_payment_date : &str, maturity_date : &str) -> Result<f64,&'static str>{

    let rounded_pv = round2(pv);
    // min and max 
    let mut miny : f64 = 0.0;
    let mut maxy : f64 = 20.0;
    let mut candidate_yield = (maxy+miny)/2.0;  // Starting point


    let today_date = NaiveDate::parse_from_str(today, "%Y-%m-%d").map_err(|_| "Error converting todays date")?;
    let payment_date = NaiveDate::parse_from_str(next_payment_date, "%Y-%m-%d").map_err(|_| "Error converting next payment date")?;
    let maturity_date = NaiveDate::parse_from_str(maturity_date, "%Y-%m-%d").map_err(|_| "Error converting maturity date")?;

    let days_to_mature = maturity_date.signed_duration_since(today_date).num_days() as u32;
    let num_days_in_period = 366 / payments_per_year;
    let periods_to_mature = (days_to_mature as f64/num_days_in_period as f64).ceil() as u32;
    log::info!("days_to_mature: {days_to_mature}, num_days_in_period: {num_days_in_period}, periods_to_mature: {periods_to_mature}");
    log::info!("periods_to_mature: {periods_to_mature}");

    let mut payments : Vec<f64> = vec![];

    let days_to_next_payment = payment_date.signed_duration_since(today_date).num_days();

    let remaining_period_ratio = days_to_next_payment as f64/num_days_in_period as f64;
    log::info!("Days to next payment: {days_to_next_payment} remaining_period_ratio: {remaining_period_ratio}");

    let payment_per_period = maturity_val*coupon_interest_rate/payments_per_year as f64/100.0;

    for _i in 0..periods_to_mature {
         payments.push(payment_per_period);
    }

    // 
    let compute_gain = |payments : &Vec<f64>, maturity_val : f64,candidate_yield : f64| -> f64 {

        let mut total_gain = payments.iter().enumerate().fold(0.0, |mut gain, (i,v)| {
            gain += v/((1.0f64 + candidate_yield/100.0).powf(i as f64 + remaining_period_ratio));
            gain
        }); 
        // Maturity value
        total_gain += maturity_val/(1.0 + candidate_yield/100.0).powf(payments.len() as f64 - 1.0 + remaining_period_ratio);
        // Interest on interest
        total_gain += match reinvestement_rate {
            Some(reinvestement_rate) => 
                calculate_interest_on_interest(coupon_interest_rate/100.0*maturity_val, reinvestement_rate, payments_per_year, payments.len() as u32),
            None => 0.0,
        };
        total_gain
    };

    let get_aproxy_yield = |payments : &Vec<f64>, maturity_val : f64, pv: f64, miny : f64, maxy : f64| -> f64 {
       let rounded_pv = round2(pv);
       let mut margin = rounded_pv;
       let mut aproxy_yield = miny;
       log::info!("aproxy miny={} maxy={}", miny*100.0,maxy*100.0);
       for candy in (miny*100.0) as u32.. ((maxy+1.0)*100.0) as u32 {
           let candy : f64 = candy as f64/100.0;
           let gain = round2(compute_gain(payments, maturity_val, candy));           let new_margin = (gain - rounded_pv).abs();
           log::info!("aproxy candidate yield: {candy}, calculated present value: {gain}, actual present value: {pv}");
           if new_margin < margin {
               margin = new_margin;
               aproxy_yield = candy;
           }
       }
       log::info!("Final aproxy yield: {aproxy_yield}");
       aproxy_yield
    };

    let mut run = true; 
    while run == true  {
        let total_gain = compute_gain(&payments,maturity_val,candidate_yield);

        let rounded_total_gain = round2(total_gain);
        log::info!("candidate yield: {candidate_yield}, present value: {rounded_total_gain}");
        if rounded_total_gain == rounded_pv {
          run = false;
        } else if rounded_total_gain > rounded_pv {
            miny = candidate_yield;
            candidate_yield = (candidate_yield + maxy)/2.0;
            if round2(miny) == round2(candidate_yield) {
                candidate_yield = get_aproxy_yield(&payments,maturity_val,pv,miny,maxy);
                run = false
            }
        }  else if rounded_total_gain < pv {
            maxy = candidate_yield;
            candidate_yield = (candidate_yield + miny)/2.0;
            if round2(maxy) == round2(candidate_yield) {
                // Lets compute with some step
                candidate_yield = get_aproxy_yield(&payments,maturity_val,pv,miny,maxy);
                run = false
            }
        }
    }
    
    //let effective_annual_yield = 100.0*((1.0 + candidate_yield/100.0).powf(payments_per_year as f64 - 1.0 + remaining_period_ratio) - 1.0);
    let effective_annual_yield = candidate_yield*payments_per_year as f64; 
    log::info!("effective_annual_yield: {effective_annual_yield}");
    Ok(round2(effective_annual_yield))
}

fn round2(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

fn get_current_yield(pv: f64, maturity_val: f64, coupon_interest : f64) -> Result<f64,&'static str>{

   Ok(round2(coupon_interest*maturity_val/pv)) 
}

fn analyze_bonds(name: &str,cost: f64, maturity_val: f64, coupon_interest : f64, payments_per_year : u32, reinvestment_rate :f64,next_payment_date : &str, maturity_date : &str) -> Result<(),&'static str> {

    let now: DateTime<Local> = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let effective_annual_yield = calculate_yield_to_maturity(cost,maturity_val,coupon_interest,payments_per_year,None,&date_str,next_payment_date,maturity_date)?;
    let promised_annual_yield = calculate_yield_to_maturity(cost,maturity_val,coupon_interest,payments_per_year,Some(reinvestment_rate),&date_str,next_payment_date,maturity_date)?;
    let (pv, total_gain, total_return) = calculate_total_return(cost,maturity_val,coupon_interest,Some(reinvestment_rate),payments_per_year,&date_str,next_payment_date,maturity_date)?;
    let r#yield = get_current_yield(pv,maturity_val,coupon_interest)?;
    println!("{name} Curr Yield[%]: {yield} YTM[%]: {effective_annual_yield}, YTMWR{reinvestment_rate}[%]: {promised_annual_yield} , present value: {pv}, total gain: {total_gain}, total return: {total_return}");
    Ok(())
}

fn main() -> Result<(),&'static str>{

    // Make a default logging level: error
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error")
    }
    simple_logger::SimpleLogger::new().env().init().unwrap();


   analyze_bonds("US 4.5% 11/25",100.7199+0.25,100.0,4.5,2,0.0,"2024-11-15","2025-11-15")?;
   analyze_bonds("US 4.125% 11/32",100.3882+0.25,100.0,4.14,2,0.0,"2024-11-15","2032-11-15")?;
   analyze_bonds("EURO 0.8% 07/25",0.9791*1.0025,1.0,0.8,1,1.35,"2025-07-4","2025-07-4")?;
   analyze_bonds("EURO 2.0% 10/27",0.9939*1.0025,1.0,2.0,1,1.35, "2025-10-4","2027-10-4")?;

   analyze_bonds("POLAND 5.25% 01/25",1039.3543*1.0025,1000.0,5.25,1,1.35,"2025-01-20","2025-01-20")?;
   analyze_bonds("FINLAND 0.5% 01/26",963.6788*1.0025,1000.0,0.5,1,1.35,"2025-04-15","2026-04-15")?;
   analyze_bonds("FRANCE 2.5% 05/30",0.9864*1.0025,1.0,2.5,1,1.35,"2025-05-25","2030-05-25")?;

   analyze_bonds("MERCEDES-BENZ 2.0% 08/26",994.5898*1.0025,1000.0,2.0,1,1.35,"2024-08-25","2026-08-25")?;

   analyze_bonds("ROR 5.75% 08/31",10000.0, 10000.0, 5.75 ,12,3.25,"2024-09-30","2025-08-31")?;
   analyze_bonds("ROS0830 6.75%",71000.0, 71000.0, 6.75 ,12,6.75,"2024-09-30","2030-08-31")?;
   analyze_bonds("ROS0830 7.05%",71000.0, 71000.0, 7.05 ,12,7.05,"2024-09-30","2036-08-31")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_bond_yield() -> Result<(), &'static str> {
        let payments: Vec<f64> = vec![
            (100.0),
            (100.0),
            (100.0),
            (1000.0),
        ];
        assert_eq!(calculate_yield(903.10,&payments, 1), Ok(11.0));
        Ok(())
    }

    #[test]
    fn test_bond_yield_semiannual() -> Result<(), &'static str> {
        let payments: Vec<f64> = vec![
            (100.0),
            (100.0),
            (100.0),
            (1000.0),
        ];
        assert_eq!(calculate_yield(903.10,&payments, 2), Ok(23.21));
        Ok(())
    }
    #[test]
    fn test_yield_to_maturity_full_semiannual() -> Result<(), &'static str> {
        assert_eq!(calculate_yield_to_maturity(769.42,1000.0,7.0,2,None,"2020-03-02","2020-09-01","2035-09-01"), Ok(9.94));
        Ok(())
    }
    #[test]
    fn test_yield_to_maturity_semiannual() -> Result<(), &'static str> {
        assert_eq!(calculate_yield_to_maturity(769.42,1000.0,7.0,2,None,"2024-07-22","2024-09-01","2036-09-01"), Ok(10.88));
        Ok(())
    }
    #[test]
    fn test_yield_to_maturity_annual() -> Result<(), &'static str> {
        assert_eq!(calculate_yield_to_maturity(1039.3543,1000.0,5.25,1,None,"2024-07-27","2025-01-20","2025-01-20"), Ok(2.63));
        Ok(())
    }
    #[test]
    fn test_current_yield() -> Result<(), &'static str> {
        assert_eq!(get_current_yield(769.42,1000.0,7.0), Ok(9.10));
        Ok(())
    }
    #[test]
    fn test_calculate_total_return_no_reinvestment() -> Result<(), &'static str> {
        assert_eq!(calculate_total_return(769.42,1000.0,7.0,None,2,"2020-03-02","2020-09-01","2035-09-01"), Ok((769.42, 1315.58, 2085.0)));
        Ok(())
    }

    #[test]
    fn test_calculate_interest_on_interest() -> Result<(), &'static str> {
        assert_eq!(round2(calculate_interest_on_interest(4.5, 0.0, 2, 3)), 0.0);
        assert_eq!(round2(calculate_interest_on_interest(70.0, 10.0, 2, 30)), 1275.36);
        Ok(())
    }

    #[test]
    fn test_round() -> Result<(), &'static str> {
        assert_eq!(round2(1.234567), 1.23);
        Ok(())
    }

    // sing IOWA state hospitals
}
