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

fn calculate_yield_to_maturity(
pv: f64, maturity_val: f64, coupon_interest : f64, payments_per_year : u32, today: &str, next_payment_date : &str, maturity_date : &str) -> Result<f64,&'static str>{

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
    let periods_to_mature = (days_to_mature as f64/num_days_in_period as f64).floor() as u32;
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

    // 
    let compute_gain = |payments : &Vec<f64>, maturity_val : f64, candidate_yield : f64| -> f64 {

        let mut total_gain = payments.iter().enumerate().fold(0.0, |mut gain, (i,v)| {
            gain += v/((1.0f64 + candidate_yield/100.0).powf(i as f64 + remaining_period_ratio));
            gain
        }); 
        total_gain += maturity_val/(1.0 + candidate_yield/100.0).powf(payments.len() as f64 - 1.0 + remaining_period_ratio);
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
    
    //let effective_annual_yield = candidate_yield; 
    let effective_annual_yield = 100.0*((1.0 + candidate_yield/100.0).powf(payments_per_year as f64 - 1.0 + remaining_period_ratio) - 1.0);
    Ok(round2(effective_annual_yield))
}

fn round2(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

fn get_current_yield(pv: f64, maturity_val: f64, coupon_interest : f64) -> Result<f64,&'static str>{

   Ok(round2(coupon_interest*maturity_val/pv)) 
}

fn main() -> Result<(),&'static str>{

    // Make a default logging level: error
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error")
    }
    simple_logger::SimpleLogger::new().env().init().unwrap();


    //let effective_annual_yield = calculate_yield_to_maturity(903.10,1000.0,10.0,1,"2024-09-02","2025-09-01","2028-09-01")?;
    let effective_annual_yield =  calculate_yield_to_maturity(769.42,1000.0,7.0,2,"2021-03-02","2021-09-01","2036-09-01")?;

    let now: DateTime<Local> = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    //let effective_annual_yield = calculate_yield_to_maturity(769.42,1000.0,7.0,2,"2024-07-22","2024-09-01","2036-09-01")?;
    println!("Yield to Maturity(as of {date_str}): {effective_annual_yield}");
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
        assert_eq!(calculate_yield_to_maturity(769.42,1000.0,7.0,2,"2020-03-02","2020-09-01","2035-09-01"), Ok(10.25));
        Ok(())
    }
    #[test]
    fn test_yield_to_maturity_semiannual() -> Result<(), &'static str> {
        assert_eq!(calculate_yield_to_maturity(769.42,1000.0,7.0,2,"2024-07-22","2024-09-01","2036-09-01"), Ok(6.76));
        Ok(())
    }
    #[test]
    fn test_yield_to_maturity_annual() -> Result<(), &'static str> {
//        assert_eq!(calculate_yield_to_maturity(903.10,1000.0,10.0,1,"2024-09-01","2025-09-01","2028-09-01"), Ok(11.00));
        Ok(())
    }
    #[test]
    fn test_current_yield() -> Result<(), &'static str> {
        assert_eq!(get_current_yield(769.42,1000.0,7.0), Ok(9.10));
        Ok(())
    }
    // sing IOWA state hospitals
}
