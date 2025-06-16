trait PaymentStrategy {
    fn pay(&self, amount: f64) -> String;
}

struct CreditCardStrategy {
    card_number: String,
}

impl PaymentStrategy for CreditCardStrategy {
    fn pay(&self, amount: f64) -> String {
        format!("Paid ${:.2} with credit card {}", amount, &self.card_number[..4])
    }
}

struct PayPalStrategy {
    email: String,
}

impl PaymentStrategy for PayPalStrategy {
    fn pay(&self, amount: f64) -> String {
        format!("Paid ${:.2} with PayPal {}", amount, self.email)
    }
}

struct PaymentProcessor {
    strategy: Box<dyn PaymentStrategy>,
}

impl PaymentProcessor {
    fn process_payment(&self, amount: f64) -> String {
        self.strategy.pay(amount)
    }
}

// Usage
fn main() {
    let credit_card = CreditCardStrategy { card_number: "1234567812345678".to_string() };
    let processor = PaymentProcessor {
        strategy: Box::new(credit_card),
    };
    println!("{}", processor.process_payment(49.99));

    let paypal = PayPalStrategy { email: "user@example.com".to_string() };
    let processor = PaymentProcessor {
        strategy: Box::new(paypal),
    };
    println!("{}", processor.process_payment(29.99));
}