trait Product {
    fn use_product(&self);
}

struct IDCard {
    owner: String,
}

impl IDCard {
    fn new(owner: String) -> IDCard {
        println!("{}のカードを作ります。", owner);
        IDCard {
            owner: owner,
        }
    }

    fn get_owner(&self) -> String {
        self.owner.clone()
    }
}

impl Product for IDCard {
    fn use_product(&self) {
        println!("{}のカードを使います", self.owner);
    }
}

trait Factory {
    type Object;
    fn create(&mut self, owner: String) -> Box<Self::Object> {
        let product = self.create_product(owner);
        self.register_product(&product);
        product
    }

    fn create_product(&self, owner: String) -> Box<Self::Object>;
    fn register_product(&mut self, product: &Box<Self::Object>);
}

struct IDCardFactory {
    owners: Vec<String>,
}

impl IDCardFactory {
    fn new() -> IDCardFactory {
        IDCardFactory {
            owners: Vec::new(),
        }
    }
}

impl Factory for IDCardFactory {
    type Object = IDCard;

    fn create_product(&self, owner: String) -> Box<Self::Object> {
        Box::new(IDCard::new(owner))
    }

    fn register_product(&mut self, product: &Box<Self::Object>) {
        self.owners.push(product.get_owner());
    }
}
fn main() {
    let mut factory = IDCardFactory::new();
    let card1 = factory.create("結城浩".to_string());
    let card2 = factory.create("とむら".to_string());
    let card3 = factory.create("佐藤花子".to_string());

    card1.use_product();
    card2.use_product();
    card3.use_product();

    // [thaodt@tuf-aldinelucifer pfactory]$ cargo run
    // Compiling pfactory v0.1.0 (/home/thaodt/projects/playground/play-rust/pfactory)
    // Finished dev [unoptimized + debuginfo] target(s) in 0.67s
    //  Running `target/debug/pfactory`
    // 結城浩のカードを作ります。
    // とむらのカードを作ります。
    // 佐藤花子のカードを作ります。
    // 結城浩のカードを使います
    // とむらのカードを使います
    // 佐藤花子のカードを使います

    // I will make a card for Hiroshi Yuki.
    // Make a Tomura card.
    // Make a card for Hanako Sato.
    // I will use Hiroshi Yuki's card
    // Use Tomura's card
    // I will use Hanako Sato's card 
}
