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
        println!("candidate yield: {candidate_yield}, present value: {rounded_total_gain}");
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
pv: f64, maturity_val: f64, coupon_interest : f64, years_to_mature: u32, payments_per_year : u32, today: &str, next_payment_date : &str) -> Result<f64,&'static str>{

    let rounded_pv = round2(pv);
    // min and max 
    let mut miny : f64 = 0.0;
    let mut maxy : f64 = 20.0;
    let mut candidate_yield = (maxy+miny)/2.0;  // Starting point

    let periods_to_mature = years_to_mature*payments_per_year;

    let mut payments : Vec<f64> = vec![];
    let num_days_in_period = 360 / payments_per_year;
    let remaining_period_ratio = 1.0; // TODO    

    let payment_per_period = maturity_val*coupon_interest/payments_per_year as f64/100.0;

    for _i in 0..periods_to_mature {
         payments.push(payment_per_period/(1.0f64 + candidate_yield/100.0).powf(remaining_period_ratio));
    }

    // 
    let compute_gain = |payments : &Vec<f64>, maturity_val : f64, candidate_yield : f64| -> f64 {

        let mut total_gain = payments.iter().enumerate().fold(0.0, |mut gain, (i,v)| {
            gain += v/(1.0f64 + candidate_yield/100.0).powf(i as f64);
            gain
        }); 
        total_gain += maturity_val/(1.0 + candidate_yield/100.0).powf(payments.len() as f64);
        total_gain
    };

    let get_aproxy_yield = |payments : &Vec<f64>, maturity_val : f64, pv: f64, miny : f64, maxy : f64| -> f64 {
       let rounded_pv = round2(pv);
       let mut margin = rounded_pv;
       let mut aproxy_yield = miny;
       println!("aproxy miny={} maxy={}", miny*100.0,maxy*100.0);
       for candy in (miny*100.0) as u32.. ((maxy+1.0)*100.0) as u32 {
           let candy : f64 = candy as f64/100.0;
           let gain = round2(compute_gain(payments, maturity_val, candy));           let new_margin = (gain - rounded_pv).abs();
           println!("aproxy candidate yield: {candy}, present value: {gain}");
           if new_margin < margin {
               margin = new_margin;
               aproxy_yield = candy;
           }
       }
       println!("Final aproxy yield: {aproxy_yield}");
       aproxy_yield
    };

    let mut run = true; 
    while run == true  {
        let total_gain = compute_gain(&payments,maturity_val,candidate_yield);

        let rounded_total_gain = round2(total_gain);
        println!("candidate yield: {candidate_yield}, present value: {rounded_total_gain}");
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
    let effective_annual_yield = 100.0*((1.0 + candidate_yield/100.0).powf(payments_per_year as f64) - 1.0);
    Ok(round2(effective_annual_yield))
}

fn round2(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

fn get_current_yield(pv: f64, maturity_val: f64, coupon_interest : f64) -> Result<f64,&'static str>{

   Ok(round2(coupon_interest*maturity_val/pv)) 
}

fn main() {
    println!("Hello, world!");
        let payments: Vec<f64> = vec![
            (100.0),
            (100.0),
            (100.0),
            (1000.0),
        ];
        calculate_yield(903.10,&payments, 1);
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
    fn test_yield_to_maturity_semiannual() -> Result<(), &'static str> {
        assert_eq!(calculate_yield_to_maturity(769.42,1000.0,7.0,15,2, "12 Apr 2024","20 Jul 2024"), Ok(10.25));
        Ok(())
    }
    #[test]
    fn test_current_yield() -> Result<(), &'static str> {
        assert_eq!(get_current_yield(769.42,1000.0,7.0), Ok(9.10));
        Ok(())
    }
    // sing IOWA state hospitals
}
