/*


|-------------|-------------------------|----------------------------------------------|
0             12570                   50270                                          125140


|------|-------------------------|-----------------------------------------------------|
0             12570                   50270                                          125140











*/







// Objective: create a Rust implementation of a simple HMRC tax calculation.
// This should include: salary sacrifice deductions, Income tax as per bands, personal allowance, final take home pay.
// Do not consider NIC

/*
    Band	Taxable income	Tax rate
    Personal Allowance	Up to £12,570	0%
    Basic rate	£12,571 to £50,270	20%
    Higher rate	£50,271 to £125,140	40%
    Additional rate	over £125,140	45%

    Tapering: for every 2 pounds above 100,000, the personal allowance is reduced by 1
*/
use std::env;
use std::cmp;

const MAX_PERSONAL_ALLOWANCE: i32 = 12_570;
const MAX_BAND1: i32 = 50_270 - 12_570;
const MAX_BAND2: i32 = 125_140 - MAX_BAND1;

const TAPER_THRESHOLD: i32 = 100_000;

#[derive(Debug, PartialEq)]
struct Breakdown {
    gross: i32,
    sacrifice: i32,    
    allowance: i32,
    taxable: i32,
    tax: i32
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        panic!("Requires one argument (gross salary) and has one optional argument (salary sacrifice)");
    }

    let gross: i32 = args[1].parse().unwrap();

    let salary_sacrifice: Option<i32>;
    if args.len() > 2 {
        salary_sacrifice = Some(args[2].parse().unwrap())
    } else {
        salary_sacrifice = None
    }

    println!("Arguments. Gross {gross} - Sacrifice {:?}", salary_sacrifice);

    if salary_sacrifice.unwrap() >= gross {
        panic!("Salary sacrifice must be less than gross: {:?} > {:?}", salary_sacrifice, gross)
    }

    let breakdown = calculate_payment_breakdown(gross, salary_sacrifice.unwrap_or(0));
    println!("Results of payment breakdown - {:?}", breakdown)
}

fn calculate_payment_breakdown(gross: i32, sacrifice: i32) -> Breakdown {
    let effective_salary = gross - sacrifice;
    let allowance_reduction = compute_allowance_reduction(effective_salary);
    let updated_allowance = MAX_PERSONAL_ALLOWANCE - allowance_reduction;
    let taxable = effective_salary - updated_allowance;
    let tax = compute_tax_with_bands(taxable);

    return Breakdown {
        gross,
        sacrifice,
        allowance: updated_allowance,
        taxable,
        tax
    }
}

fn compute_tax_with_bands(taxable: i32) -> i32 {

    let band_rates = [20, 40, 45];
    let max_bands: [i32; 2] = [MAX_BAND1, MAX_BAND2];
    let mut salary_bands: [i32; 3] = [0; 3];
    let mut remaining_balance = taxable;

    for band in 0..2 {
        let band_slice = effective_balance(remaining_balance, max_bands[band]);
        salary_bands[band] = band_slice;
        remaining_balance = cmp::max(0, remaining_balance - band_slice);
        println!("Found {} at index {}", band_slice, band);
    }

    salary_bands[2] = remaining_balance;

    let mut tax_accumulator: i32 = 0;
    for i in 0..3 {
        println!("Tax at rate {}%: Over {} = {}", band_rates[i], salary_bands[i], salary_bands[i] * band_rates[i] / 100 );
        tax_accumulator = tax_accumulator + (salary_bands[i] * band_rates[i] / 100);
    }

    return tax_accumulator;
}

fn effective_balance(balance: i32, allowed_in_band: i32) -> i32 {
    return cmp::max(0, cmp::min(balance, allowed_in_band));
}

fn compute_allowance_reduction(taxable: i32) -> i32 {
    if taxable <= TAPER_THRESHOLD {
        return 0;
    }

    return cmp::min(MAX_PERSONAL_ALLOWANCE, (taxable - TAPER_THRESHOLD) / 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gross_salary_20_000() {
        let gross = 20_000;
        let sacrifice = 0;

        let expected: Breakdown = Breakdown {
            gross,
            sacrifice,
            allowance: 12_570,
            taxable: 7_430,
            tax: 1_486,
        };

        assert_eq!(expected, super::calculate_payment_breakdown(gross, sacrifice));
    }

    #[test]
    fn gross_salary_50_000() {
        let gross = 50_000;
        let sacrifice = 0;

        let expected: Breakdown = Breakdown {
            gross,
            sacrifice,
            allowance: 12_570,
            taxable: 37_430,
            tax: 7_486,
        };

        assert_eq!(expected, super::calculate_payment_breakdown(gross, sacrifice));
    }


    #[test]
    fn gross_salary_80_000() {
        let gross = 80_000;
        let sacrifice = 0;

        let expected: Breakdown = Breakdown {
            gross,
            sacrifice,
            allowance: 12_570,
            taxable: 67_430,
            tax: 19_432,
        };

        assert_eq!(expected, super::calculate_payment_breakdown(gross, sacrifice));
    }

    #[test]
    fn gross_salary_100_000() {
        let gross = 100_000;
        let sacrifice = 0;

        let expected: Breakdown = Breakdown {
            gross,
            sacrifice,
            allowance: 12_570,
            taxable: 87_430,
            tax: 27_432,
        };

        assert_eq!(expected, super::calculate_payment_breakdown(gross, sacrifice));
    }

    #[test]
    fn gross_salary_110_000() {
        let gross = 110_000;
        let sacrifice = 0;

        let expected: Breakdown = Breakdown {
            gross,
            sacrifice,
            allowance: 7_570,
            taxable: 102_430,
            tax: 33_432,
        };

        assert_eq!(expected, super::calculate_payment_breakdown(gross, sacrifice));
    }

    #[test]
    fn gross_salary_125_140() {
        let gross = 125_140;
        let sacrifice = 0;

        let expected: Breakdown = Breakdown {
            gross,
            sacrifice,
            allowance: 0,
            taxable: 125_140,
            tax: 42_516,
        };

        assert_eq!(expected, super::calculate_payment_breakdown(gross, sacrifice));
    }

    #[test]
    fn gross_salary_150_000() {
        let gross = 150_000;
        let sacrifice = 0;

        let expected: Breakdown = Breakdown {
            gross,
            sacrifice,
            allowance: 0,
            taxable: 150_000,
            tax: 53_703,
        };

        assert_eq!(expected, super::calculate_payment_breakdown(gross, sacrifice));
    }

    #[test]
    fn gross_salary_125_000_sacrifice_10_000() {
        let gross = 125_000;
        let sacrifice = 10_000;

        let expected: Breakdown = Breakdown {
            gross,
            sacrifice,
            allowance: 5070,
            taxable: 109_930,
            tax: 36_432,
        };

        assert_eq!(expected, super::calculate_payment_breakdown(gross, sacrifice));
    }

    #[test]
    fn gross_salary_125_000_sacrifice_25_000() {
        let gross = 125_000;
        let sacrifice = 25_000;

        let expected: Breakdown = Breakdown {
            gross,
            sacrifice,
            allowance: 12_570,
            taxable: 87_430,
            tax: 27_432,
        };

        assert_eq!(expected, super::calculate_payment_breakdown(gross, sacrifice));
    }

    #[test]
    fn gross_salary_125_000_sacrifice_50_000() {
        let gross = 125_000;
        let sacrifice = 50_000;

        let expected: Breakdown = Breakdown {
            gross,
            sacrifice,
            allowance: 12_570,
            taxable: 62_430,
            tax: 17_432,
        };

        assert_eq!(expected, super::calculate_payment_breakdown(gross, sacrifice));
    }

    #[test]
    fn gross_salary_75_000_sacrifice_50_000() {
        let gross = 75_000;
        let sacrifice = 50_000;

        let expected: Breakdown = Breakdown {
            gross,
            sacrifice,
            allowance: 12_570,
            taxable: 12_430,
            tax: 2_486,
        };

        assert_eq!(expected, super::calculate_payment_breakdown(gross, sacrifice));
    }



}